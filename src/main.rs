#[allow(dead_code)]
mod data_types;
mod packets;

use std::net::{TcpListener, TcpStream};
use data_types::data_types::ServerState;

use crate::data_types::data_types::read_var_int;
use crate::packets::packets::{HandshakePacket, LoginPacket};

fn handle_connection(mut stream: &mut TcpStream) {
    print!("Handling connection...");

    // set first state to Handshake
    let mut server_state = ServerState::Handshake;

    loop {
        // read packet length
        let (packet_length, _) = read_var_int(&mut stream);
        println!("Packet length : {:#?}", packet_length);

        // read packet ID
        let (packet_id, _) = read_var_int(&mut stream);
        println!("Packet ID : {:#?}", packet_id);

        match packet_id {
            0 => {
                match server_state {
                    ServerState::Handshake => server_state = HandshakePacket::handle_handshake(stream),
                    ServerState::Login => LoginPacket::handle_login(stream),
                }
                
            }
            _ => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                /* let mut s = stream.try_clone().unwrap();
                let s2 = s.borrow_mut(); */
                handle_connection(&mut stream);
            }
            Err(_e) => {
                println!("connection failed...");
            }
        }
    }
    Ok(())
}