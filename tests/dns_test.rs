use mcping::{resolve, DEFAULT_PORT};
use std::net::Ipv4Addr;
use std::str::FromStr;

#[tokio::test]
async fn test_resolve() {
    let r = resolve("::1", DEFAULT_PORT).await;
    assert!(r.is_err());
    let r = resolve("nope", DEFAULT_PORT).await;
    assert!(r.is_err());

    let r = resolve("127.0.0.2", 25562u16).await;
    assert!(r.is_ok());
    assert!(r
        .unwrap()
        .contains(&(Ipv4Addr::from_str("127.0.0.2").unwrap(), 25562u16)));

    let r = resolve("localhost", 25563u16).await;
    assert!(r.is_ok());
    assert!(r
        .unwrap()
        .contains(&(Ipv4Addr::from_str("127.0.0.1").unwrap(), 25563u16)));
}
