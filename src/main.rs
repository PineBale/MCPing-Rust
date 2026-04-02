use crate::protocol::get_protocol_number_by_version;
use clap::{ArgAction, Parser};
use std::io::{stdout, Write};

mod dns;
mod packet;
mod protocol;

#[derive(Parser)]
#[command(about = "Minecraft Server List Ping tool", long_about = None, disable_help_flag = true)]
struct MCPingArguments {
    #[arg(short, long = "help", action = ArgAction::Help, help = "Print this help information")]
    _help: Option<bool>,

    #[arg(help = "Server address", required = true)]
    address: String,

    #[arg(short, long = "fakehost")]
    fakehost: Option<String>,

    #[arg(short, long = "minecraft-version", default_value = "1.8")]
    minecraft_version: Option<String>,

    #[arg(long, default_value_t = 5u8, help = "Connection timeout in seconds")]
    timeout: u8,
}

#[tokio::main]
async fn main() {
    let args: MCPingArguments = MCPingArguments::parse();
    let vs = &args.minecraft_version.unwrap_or_default();
    let v = match get_protocol_number_by_version(vs) {
        Some(v) => v,
        None => panic!("Unknown Minecraft version {vs}"),
    };
    let res = mcping::ping(
        &args.address,
        &args.fakehost.unwrap_or_default(),
        v,
        args.timeout,
    )
    .await
    .expect("Failed to ping");
    let _ = stdout().write_all(&res);
}
