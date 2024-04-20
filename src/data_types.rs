#[allow(dead_code)]
use std::{io::{Read, Write}, net::TcpStream};

pub enum ServerState{
    Handshake,
    Login,
    Play,
}

#[derive(Debug)]
pub struct VarInt(pub i32);

#[derive(Debug)]
pub struct Property{
    name: String,
    value: String,
    is_signed: bool,
    signature: Option<String>,
}

#[derive(Debug)]
pub struct Array{
    items: Vec<Property>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            items: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    x: i32,
    z: i32,
    y: i16,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            z: 0,
            y: 0,
        }
    }
    pub fn deserialize(input: i64) -> Position {
        Position {
            x: (input >> 38) as i32,
            z: (input << 52 >> 52) as i32,
            y: (input << 26 >> 38) as i16,
        }
    }
}

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

pub fn read_var_long(stream: &mut TcpStream) -> i64 {
    let mut value: i64 = 0;
    let mut position: u32 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = read_byte(stream);
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

pub fn write_var_int(output: &mut impl Write, mut value: i32) {
    loop {
        if (value &!SEGMENT_BITS as i32) == 0 {
            write_byte(output, value as u8);
            return;
        }

        write_byte(output, (value &SEGMENT_BITS as i32) as u8 | CONTINUE_BIT);

        value >>= 7;
    }
}

pub fn write_var_long(stream: &mut TcpStream, mut value: i64) {
    loop {
        if (value & !(SEGMENT_BITS as i64)) == 0 {
            write_byte(stream, value as u8);
            return;
        }

        write_byte(stream, (value & SEGMENT_BITS as i64) as u8 | CONTINUE_BIT);

        value >>= 7;
    }
}

pub fn read_byte(stream: &mut TcpStream) -> u8 {
    let mut buf = [0];
    let _ = stream.read_exact(&mut buf);

    *buf.first().expect("no bytes to read...")
}

pub fn write_byte(output: &mut impl Write, byte: u8) {
    let _ = output.write_all(&[byte]);
}

