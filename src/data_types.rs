#[allow(dead_code)]
pub mod data_types {
    use std::{io::Read, net::TcpStream};

    pub enum ServerState{
        Handshake,
        Login,
    }

    /* impl ServerState {
        pub fn new() -> ServerState {
            ServerState::Handshake
        } 
    } */

    const SEGMENT_BITS: u8 = 0x7F;
    const CONTINUE_BIT: u8 = 0x80;

    pub fn read_var_int(stream: &mut TcpStream) -> (i32, u32) {
        let mut value: i32 = 0;
        let mut position: u32 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = read_byte(stream);
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                panic!("VarInt is too big");
            }
        }

        (value, position / 7 + 1)
    }

    /* pub fn read_var_long() -> i64 {
        let mut value: i64 = 0;
        let mut position: u32 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = read_byte();
            value |= ((current_byte & SEGMENT_BITS) as i64) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 64 {
                panic!("VarLong is too big");
            }
        }

        value
    }

    pub fn write_var_int(mut value: i32) {
        loop {
            if (value &!SEGMENT_BITS as i32) == 0 {
                write_byte(value as u8);
                return;
            }

            write_byte((value &SEGMENT_BITS as i32) as u8 | CONTINUE_BIT);

            value >>= 7;
        }
    }

    pub fn write_var_long(mut value: i64) {
        loop {
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                write_byte(value as u8);
                return;
            }

            write_byte((value & SEGMENT_BITS as i64) as u8 | CONTINUE_BIT);

            value >>= 7;
        }
    } */

    pub fn read_byte(stream: &mut TcpStream) -> u8 {
        let mut buf = [0];
        let _ = stream.read_exact(&mut buf);

        *buf.first().expect("no bytes to read...")
    }
    
    pub fn write_byte(_: u8) {
        // Implement your write byte logic here
        unimplemented!()
    }
}

