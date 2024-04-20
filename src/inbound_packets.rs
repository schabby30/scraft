#[allow(dead_code)]
use crate::data_types::{read_var_int, read_var_long};
use crate::data_types::{Position, VarInt};
use std::net::TcpStream;

#[derive(Debug)]
pub(crate) struct QueryBlockEntityPacket {
    transaction_id: VarInt,
    location: Position,
}

impl QueryBlockEntityPacket {
    pub fn handle_query_block_entity(stream: &mut TcpStream) -> QueryBlockEntityPacket{
        let transaction_id = read_var_int(stream);
        let position = Position::deserialize(read_var_long(stream));

        QueryBlockEntityPacket {
            transaction_id: VarInt(transaction_id.0),
            location: position,
        }
    }
}