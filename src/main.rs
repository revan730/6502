use std::{env, fs, io, process::exit};

use cpu::Cpu;

use crate::memory_bus::MemoryBus;

#[macro_use]
extern crate lazy_static;

mod cpu;
mod error;
mod instruction;
mod memory_bus;
mod opcode_decoders;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_data = match load_file(&args[1]) {
        Err(e) => {
            println!("Failed to read ROM: {e:?}");
            exit(123)
        }
        Ok(data) => data,
    };

    let mut memory = MemoryBus::new();
    memory.load_rom(0x0, &rom_data).unwrap();
    println!("{:?}", memory);
    let mut cpu = Cpu::new(memory);

    loop {
        cpu.step();
        println!("Cpu state: {:?}", cpu);
    }
}

fn load_file(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}
