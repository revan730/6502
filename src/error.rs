#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Unknown opcode: {0}")]
    UnknownOpcodeError(u8),
}

#[derive(thiserror::Error, Debug)]
pub enum MemoryBusError {
    #[error("ROM Data size out of region bounds")]
    ROMLoadOutOfBoundsError,
    #[error("Offset out of region bounds: {0:X}")]
    OffsetOutOfBoundsError(usize),
}
