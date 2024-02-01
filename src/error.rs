#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(String),
    #[error("Expected byte argument, found #OTHERTYPE#")] // TODO: Fill #OTHERTYPE#
    ByteExpectedArgument,
    #[error("Expected address argument, found #OTHERTYPE#")] // TODO: Fill #OTHERTYPE#
    AddrExpectedArgument,
}

#[derive(thiserror::Error, Debug)]
pub enum MemoryBusError {
    #[error("ROM Data size out of region bounds")]
    ROMLoadOutOfBounds,
    #[error("Offset out of region bounds: {0:#X}")]
    OffsetOutOfBounds(usize),
}
