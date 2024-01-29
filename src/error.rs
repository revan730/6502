#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Unknown opcode: {0}")]
    UnknownOpcodeError(u8),
}
