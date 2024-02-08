#[macro_use]
extern crate lazy_static;

pub mod cpu;
pub mod error;
mod flags_register;
mod instruction;
pub mod memory_bus;
mod opcode_decoders;
