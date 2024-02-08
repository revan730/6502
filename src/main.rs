use std::{env, fs, io, process::exit};

use cpu::Cpu;

use crate::memory_bus::MemoryBus;

#[macro_use]
extern crate lazy_static;

mod cpu;
mod error;
mod flags_register;
mod instruction;
mod memory_bus;
mod opcode_decoders;

static mut ROM_STORAGE: [u8; 0x1000] = [0; 0x1000];

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

    let rom_region = memory_bus::MemoryRegion {
        start: 0x200,
        end: 0x400,
        read_handler: Box::new(|address| unsafe { ROM_STORAGE[address] }),
        write_handler: Box::new(|address, value| unsafe {
            ROM_STORAGE[address] = value;
        }),
    };

    memory.add_region(rom_region);

    unsafe { load_rom(&rom_data) };
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

pub unsafe fn load_rom(data: &[u8]) {
    ROM_STORAGE[..data.len()].copy_from_slice(data);
}
