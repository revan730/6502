use std::fmt::Debug;

pub const MEM_SPACE_END: usize = 0xFFFF;
pub const STACK_BOTTOM: usize = 0x0100;

pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub read_handler: Box<dyn Fn(usize) -> u8>,
    pub write_handler: Box<dyn FnMut(usize, u8)>,
}

pub struct MemoryBus {
    region_maps: Vec<MemoryRegion>,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
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
