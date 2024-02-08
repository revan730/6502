use std::fmt::Debug;

const ZERO_PAGE_START: usize = 0x00;
pub const ZERO_PAGE_END: usize = 0xFF;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_START + 1;

const DATA_STACK_START: usize = 0x100;
const DATA_STACK_END: usize = 0x1FF;
const DATA_STACK_SIZE: usize = DATA_STACK_END - DATA_STACK_START + 1;

const RAM_IO_ROM_START: usize = 0x200;
const RAM_IO_ROM_END: usize = 0xFFF9;
const RAM_IO_ROM_SIZE: usize = RAM_IO_ROM_END - RAM_IO_ROM_START + 1;

const NMI_START: usize = 0xFFFA;
const NMI_END: usize = 0xFFFB;

const RESET_START: usize = 0xFFFC;
const RESET_END: usize = 0xFFFD;

const IRQ_START: usize = 0xFFFE;
const IRQ_END: usize = 0xFFFF;

const VECTOR_SIZE: usize = 2;

pub const MEM_SPACE_END: usize = 0xFFFF;

pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub read_handler: Box<dyn Fn(usize) -> u8>,
    pub write_handler: Box<dyn FnMut(usize, u8)>,
}

pub struct MemoryBus {
    zero_page: [u8; ZERO_PAGE_SIZE],
    data_stack: [u8; DATA_STACK_SIZE],
    ram_io_rom: [u8; RAM_IO_ROM_SIZE],
    nmi_vector: [u8; VECTOR_SIZE],
    reset_vector: [u8; VECTOR_SIZE],
    irq_vector: [u8; VECTOR_SIZE],
    region_maps: Vec<MemoryRegion>,
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
            region_maps: Vec::new(),
        }
    }

    pub fn add_region(&mut self, region: MemoryRegion) {
        self.region_maps.push(region);
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        println!("Read from addr {address:#X}");
        let mapped_region: Option<&MemoryRegion> = self
            .region_maps
            .iter()
            .find(|region| region.start <= address && region.end >= address);

        match mapped_region {
            Some(region) => (region.read_handler)(address - region.start),
            None => panic!("No region found for address {address:#X}"), // TODO: return Result to delegate error handling to the caller
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        println!("write {value:#X} to addr {address:#X}");
        let mapped_region: Option<&mut MemoryRegion> = self
            .region_maps
            .iter_mut()
            .find(|region| region.start <= address && region.end >= address);

        match mapped_region {
            Some(region) => (region.write_handler)(address - region.start, value),
            None => panic!("No region found for address {address:#X}"),
        }
    }
}

impl Debug for MemoryBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.region_maps
            .iter()
            .try_for_each(|region| writeln!(f, "Region: {:#X} - {:#X}", region.start, region.end))
    }
}
