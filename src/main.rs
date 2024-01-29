use cpu::Cpu;

#[macro_use]
extern crate lazy_static;

mod cpu;
mod error;
mod instruction;
mod memory_bus;
mod opcode_decoders;

fn main() {
    let mut cpu = Cpu::new();
    cpu.step();
    println!("Cpu state: {:?}", cpu);
    cpu.step();
    println!("Cpu state: {:?}", cpu);
}
