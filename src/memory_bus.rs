use std::fmt::Debug;

use crate::error::MemoryBusError;

const ZERO_PAGE_START: usize = 0x00;
const ZERO_PAGE_END: usize = 0xFF;
const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_START + 1;

const DATA_STACK_START: usize = 0x100;
const DATA_STACK_END: usize = 0x1FF;
const DATA_STACK_SIZE: usize = DATA_STACK_END - DATA_STACK_START + 1;

const RAM_IO_ROM_START: usize = 0x200;
const RAM_IO_ROM_END: usize = 0xFFF9;
const RAM_IO_ROM_SIZE: usize = RAM_IO_ROM_END - RAM_IO_ROM_START + 1;

const VECTOR_SIZE: usize = 2;

pub const MEM_SPACE_END: usize = 0xFFFF;

pub struct MemoryBus {
    zero_page: [u8; ZERO_PAGE_SIZE],
    data_stack: [u8; DATA_STACK_SIZE],
    ram_io_rom: [u8; RAM_IO_ROM_SIZE],
    nmi_vector: [u8; VECTOR_SIZE],
    reset_vector: [u8; VECTOR_SIZE],
    irq_vector: [u8; VECTOR_SIZE],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            zero_page: [0; ZERO_PAGE_SIZE],
            data_stack: [0; DATA_STACK_SIZE],
            ram_io_rom: [0; RAM_IO_ROM_SIZE],
            nmi_vector: [0; VECTOR_SIZE],
            reset_vector: [0; VECTOR_SIZE],
            irq_vector: [0; VECTOR_SIZE],
        }
    }

    pub fn load_rom(&mut self, offset: usize, data: &[u8]) -> Result<(), MemoryBusError> {
        if offset > RAM_IO_ROM_SIZE {
            return Err(MemoryBusError::OffsetOutOfBoundsError(offset));
        }
        if RAM_IO_ROM_START + offset + data.len() > RAM_IO_ROM_END {
            return Err(MemoryBusError::ROMLoadOutOfBoundsError);
        }

        let (_, copy_start) = self.ram_io_rom.split_at_mut(offset);
        copy_start[..data.len()].copy_from_slice(data);

        Ok(())
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        println!("Read from addr {:#X}", address);
        match address {
            RAM_IO_ROM_START..=RAM_IO_ROM_END => self.ram_io_rom[address - RAM_IO_ROM_START],
            _ => todo!(),
        }
    }
}

impl Debug for MemoryBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Data at offset 0x200: {:#X} {:#X} {:#X} {:#X}",
            self.ram_io_rom[0x0], self.ram_io_rom[0x1], self.ram_io_rom[0x2], self.ram_io_rom[0x3]
        )
    }
}
