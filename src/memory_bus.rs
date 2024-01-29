const ZERO_PAGE_START: usize = 0x00;
const ZERO_PAGE_END: usize = 0xFF;
const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_START + 1;

pub const MEM_SPACE_END: usize = 0xFFFF;

pub struct MemoryBus {
    zero_page: [u8; ZERO_PAGE_SIZE],
}
