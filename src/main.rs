#[allow(dead_code)]
mod data_types;
mod packets;

use std::net::{TcpListener, TcpStream};
use crate::data_types::data_types::read_var_int;
use crate::packets::packets::HandshakePacket;

fn handle_connection(mut stream: TcpStream) {
    // read first packet length
    let (packet_length, _, s) = read_var_int(stream);
    stream = s;
    println!("Packet length : {:#?}", packet_length);

    // read first packet ID
    let (packet_id, _, s) = read_var_int(stream);
    stream = s;
    println!("Packet ID : {:#?}", packet_id);

    match packet_id {
        0 => HandshakePacket::handle_handshake(stream),
        _ => panic!("connection failed..."),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_e) => {
                println!("connection failed...");
            }
        }
    }
    Ok(())
}