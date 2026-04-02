#![allow(unused)]

use crate::dns::*;
use crate::packet::*;
use crate::protocol::*;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::slice;
use std::time::Duration;

pub async fn ping(
    input_address: &str,
    input_fakehost: &str,
    protocol_number: ProtocolNum,
    timeout: u8,
) -> Result<Vec<u8>, &'static str> {
    if !is_known_protocol_number(protocol_number) {
        return Err("Unknown protocol number");
    }
    if timeout == 0 {
        return Err("Timeout in seconds must be bigger than 0");
    }

    let address_to_use;
    let port_to_use;

    match input_address.find(":") {
        Some(p) => {
            address_to_use = &input_address[..p];
            port_to_use = String::from(&input_address[p + 1..])
                .parse::<u16>()
                .expect("Invalid port number");
        }
        None => {
            address_to_use = input_address;
            port_to_use = DEFAULT_PORT;
        }
    }

    let hostname_to_use = if input_fakehost.is_empty() {
        address_to_use
    } else {
        input_fakehost
    };

    let ips = resolve(address_to_use, port_to_use).await?;

    for v in ips.iter() {
        let dur = Duration::from_secs(timeout as u64);

        let stream = TcpStream::connect_timeout(&SocketAddr::from(*v), dur);
        if stream.is_err() {
            continue;
        }
        let mut stream = stream.unwrap();
        stream.set_read_timeout(Some(dur));
        stream.set_write_timeout(Some(dur));

        stream
            .write_all(&compose_handshake_packet(
                hostname_to_use,
                port_to_use,
                protocol_number,
            ))
            .expect("Cannot perform handshake");
        stream
            .write_all(&compose_status_request_packet())
            .expect("Cannot send status request");

        let mut byte = 0u8;

        let resize =
            read_varint(&mut stream).expect("Cannot read varint whilst receiving response");

        stream
            .read_exact(slice::from_mut(&mut byte))
            .expect("Cannot read packet ID");
        if byte != 0u8 {
            shutoff_stream(&stream);
            return Err("Malformed response, unknown packet ID");
        }

        let resize =
            read_varint(&mut stream).expect("Cannot read varint whilst reading JSON response");
        let mut res: Vec<u8> = Vec::new();
        let mut buf = [0; 4096];
        let mut bytes_read = 0;
        while bytes_read < resize {
            let read_res = stream.read(&mut buf).expect("The stream cannot be read");
            bytes_read += read_res;
            res.extend_from_slice(&buf[..read_res]);
        }
        shutoff_stream(&stream);
        return Ok(res);
    }
    Err("All IP addresses tried, none reachable")
}

fn read_varint(stream: &mut TcpStream) -> Result<usize, &'static str> {
    let mut byte = 0x00;
    let mut res = 0i32;
    for i in 0.. {
        if i > 5 {
            shutoff_stream(stream);
            return Err("Malformed response, not a valid varint");
        }
        let buf = slice::from_mut(&mut byte);
        #[allow(clippy::unused_io_amount)]
        stream.read(buf).expect("The stream cannot be read");
        if buf.is_empty() {
            break;
        }
        res |= (((buf[0] as i32) & 0x7Fi32) << (7 * i));
        if ((buf[0] as i32) & 0x80i32) == 0 {
            break;
        }
    }
    if res <= 0 {
        shutoff_stream(stream);
        return Err("Malformed response, varint not bigger than 0");
    }
    Ok(res as usize)
}

fn shutoff_stream(stream: &TcpStream) {
    stream.shutdown(Shutdown::Both);
}
