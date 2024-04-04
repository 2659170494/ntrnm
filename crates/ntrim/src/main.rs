extern crate pretty_env_logger;
#[macro_use] extern crate log;

use bytes::{BufMut, BytesMut};
use tokio_util::codec::Encoder;
use ntrim_net::bytes::BytePacketBuilder;
use ntrim_net::packet::packet::{CommandType, UniPacket};

const WELCOME: &str = r#"
  _   _ _____ ____  ___ __  __
 | \ | |_   _|  _ \|_ _|  \/  |
 |  \| | | | | |_) || || |\/| |
 | |\  | | | |  _ < | || |  | |
 |_| \_| |_| |_| \_\___|_|  |_|
 Welcome to ntrim!"#;

fn main() {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    println!("{}", WELCOME);

    let buf = vec![1u8];
    let packet = UniPacket::new(
        CommandType::Register,
        "register".to_string(),
        buf.clone(),
        "1372362033".to_string(),
    );

    let buf = packet.to_wup_buffer();
    info!("hex: {}", hex::encode(buf))
}

