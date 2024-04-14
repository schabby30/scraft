pub mod serialize {
    pub trait MinecraftPacketPacket {
        fn serialize_minecraft_packet(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    }

    pub trait MinecraftPacketPacketPart {
        fn serialize_minecraft_packet_part(self: Self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    }
}