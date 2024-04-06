#[allow(dead_code)]
pub mod packets {
    use std::{io::Read, net::TcpStream};
    use crate::data_types::data_types::read_var_int;

    #[derive(Debug)]
    pub(crate) struct HandshakePacket {
        protocol_version: i32,
        server_address_length: i32,
        server_address: String,
        server_port: i32,
        next_state: i32,
    }

    impl HandshakePacket {
        pub fn new() -> HandshakePacket {
            HandshakePacket{
                protocol_version: 0,
                server_address_length: 0,
                server_address: String::from(""),
                server_port: 0,
                next_state: 0,
            }
        }

        pub fn handle_handshake(mut stream: TcpStream) {
            let mut handshake_packet = HandshakePacket::new();

            // read protocol version as VarInt
            let (protocol_version, _, s) = read_var_int(stream);
            stream = s;
            handshake_packet.protocol_version = protocol_version;

            // read server address' length as VarInt
            let (server_address_length, _, s) = read_var_int(stream);
            stream = s;
            handshake_packet.server_address_length = server_address_length;

            // read the server address as UTF-8 String
            let mut buf = [0;1];
            let mut server_address = [0; 255]; //this can be more bytes, so TODO!

            for i in 0..server_address_length {
                let _ = stream.read_exact(&mut buf).expect("no more bytes to read...");
                server_address[i as usize] = buf[0];
            }

            handshake_packet.server_address = String::from_utf8_lossy(&server_address[0..server_address_length as usize]).to_string();

            // read port number as unsigned short (2 bytes)
            let mut port_number = [0; 2];
            let _ = stream.read_exact(&mut port_number).expect("no more bytes to read...");
            handshake_packet.server_port = ((port_number[0] as i32) << 8) + port_number[1] as i32;

            // read the next state enum as VarInt
            let (next_state, _, _) = read_var_int(stream);
            handshake_packet.next_state = next_state;

            println!("HandshakePacket : {:#?}", &handshake_packet);
        }
    }

}