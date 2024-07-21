pub mod entry;
pub mod table;

use crate::println;

pub const PAGE_SIZE: u64 = 4096; // 4KB
const PAGE_TABLE_ENTRY_COUNT: usize = 512; // 512 * 8 bytes = 4KB

pub type PhsyAddr = usize;
pub type VirtAddr = usize;

pub fn init() {
    println!("Initializing Paging");
}
