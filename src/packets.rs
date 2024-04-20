#[allow(dead_code)]
use std::{io::{Read, Write}, net::TcpStream};
use crate::{data_types::{read_var_int, write_var_int, Array, ServerState, VarInt}, serialize::serialize::{MinecraftPacketPacket, MinecraftPacketPacketPart}};

#[derive(Debug)]
pub(crate) struct HandshakePacket {
    protocol_version: i32,
    server_address: String,
    server_port: i32,
    next_state: i32,
}

#[derive(Debug)]
pub(crate) struct LoginPacket {
    username: String,
    uuid: u128,
}

#[derive(Debug)]
pub(crate) struct LoginSuccessPacket {
    packet_id: VarInt,
    uuid: u128,
    username: String,
    num_of_properties: VarInt,
    properties: Array,
}

#[derive(Debug)]
pub(crate) struct FinishConfigurationPacket {
    packet_id: VarInt,
}

impl HandshakePacket {
    pub fn new() -> HandshakePacket {
        HandshakePacket{
            protocol_version: 0,
            server_address: String::from(""),
            server_port: 0,
            next_state: 0,
        }
    }

    pub fn handle_handshake(stream: &mut TcpStream) -> ServerState {
        let mut handshake_packet = HandshakePacket::new();

        // read protocol version as VarInt
        let (protocol_version, _) = read_var_int(stream);
        handshake_packet.protocol_version = protocol_version;

        // read server address' length as VarInt
        let (server_address_length, _) = read_var_int(stream);

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
        let (next_state, _) = read_var_int(stream);
        handshake_packet.next_state = next_state;

        println!("HandshakePacket : {:#?}", &handshake_packet);

        ServerState::Login
    }
}

impl LoginPacket {
    pub fn new() -> LoginPacket{
        LoginPacket {
            username: String::from(""),
            uuid: 0,
        }
    }

    pub fn handle_login(stream: &mut TcpStream) {
        let mut login_packet = LoginPacket::new();

        // read username's length as VarInt
        let (username_length, _) = read_var_int(stream);

        // read username as UTF-8 String
        let mut buf = [0;1];
        let mut username = [0; 255]; //this can be more bytes, so TODO!

        for i in 0..username_length {
            let _ = stream.read_exact(&mut buf).expect("no more bytes to read...");
            username[i as usize] = buf[0];
        }

        login_packet.username = String::from_utf8_lossy(&username[0..username_length as usize]).to_string();

        // read player's uuid as u128 (16 bytes)
        let mut uuid_buf = [0; 16];
        let _ = stream.read_exact(&mut uuid_buf).expect("no more bytes to read...");
        for i in 0..16 {
            login_packet.uuid = (login_packet.uuid << 8) + uuid_buf[i] as u128;
        }

        println!("Login packet : {:#?}", login_packet);

        //
        // handle LoginSuccess
        //
        let login_success_packet = LoginSuccessPacket{
            packet_id: VarInt(2),
            username: String::from(login_packet.username),
            uuid: login_packet.uuid,
            num_of_properties: VarInt(0),
            properties: Array::new(),
        };

        login_success_packet.handle_login_success(stream);
    }
}

impl LoginSuccessPacket {
    pub fn handle_login_success(self, stream: &mut TcpStream) {
        println!("LoginSuccessPacket : {:#?}", self);
        let mut output: Vec<u8> = Vec::new(); 
        let _ = Self::serialize_minecraft_packet(self, &mut output);
        let _ = stream.write_all(&mut output);
    }
}

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        FinishConfigurationPacket {
            packet_id: VarInt(0),
        }
    }

    pub fn handle_finish_configuration(self, stream: &mut TcpStream) {
        println!("FinishConfigurationPacket : {:#?}", self);
        let mut output: Vec<u8> = Vec::new(); 
        let _ = Self::serialize_minecraft_packet(self, &mut output);
        let _ = stream.write_all(&mut output);
    }
}

impl MinecraftPacketPacket for LoginSuccessPacket {
    fn serialize_minecraft_packet(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let mut tmp: Vec<u8> = Vec::new();

        // serlialize packet id
        write_var_int(&mut tmp, self.packet_id.0);

        // serlialize UUID
        let _ = self.uuid.serialize_minecraft_packet_part(&mut tmp);

        // serlialize username
        let username_in_bytes = self.username.as_bytes();
        write_var_int(&mut tmp, username_in_bytes.len() as i32);
        tmp.append(&mut Vec::from(username_in_bytes));

        // serialize num of properties
        write_var_int(&mut tmp, self.num_of_properties.0);

        // Properties serialization missing
        
        // write length of serialized fields to output
        write_var_int(output, tmp.len() as i32);

        // append serialized fields to output
        output.append(&mut tmp);
        
        Ok(())
    }
}

impl MinecraftPacketPacket for FinishConfigurationPacket {
    fn serialize_minecraft_packet(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let mut tmp: Vec<u8> = Vec::new();

        // serlialize packet id
        write_var_int(&mut tmp, self.packet_id.0);

        output.append(&mut tmp);

        Ok(())
    }
}

impl MinecraftPacketPacketPart for u128 {
    fn serialize_minecraft_packet_part(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let bytes = self.to_le_bytes();

        output.push(bytes[15]);
        output.push(bytes[14]);
        output.push(bytes[13]);
        output.push(bytes[12]);
        output.push(bytes[11]);
        output.push(bytes[10]);
        output.push(bytes[9]);
        output.push(bytes[8]);
        output.push(bytes[7]);
        output.push(bytes[6]);
        output.push(bytes[5]);
        output.push(bytes[4]);
        output.push(bytes[3]);
        output.push(bytes[2]);
        output.push(bytes[1]);
        output.push(bytes[0]);

        Ok(())
    }
}

impl MinecraftPacketPacketPart for usize {
    fn serialize_minecraft_packet_part(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        for i in 0..std::mem::size_of::<usize>() {
            let byte = ((self >> (i * 8)) & 0xFF) as u8;
            output.push(byte);
        }

        Ok(())
    }
}