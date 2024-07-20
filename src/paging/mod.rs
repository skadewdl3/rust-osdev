use multiboot2::BootInformationHeader;

use crate::println;

pub fn init(multiboot_info_ptr: usize) {
    let multiboot_info_ptr = multiboot_info_ptr as *const BootInformationHeader;
    let boot_info = unsafe { multiboot2::BootInformation::load(multiboot_info_ptr).unwrap() };
    let memory_map_tag = boot_info.memory_map_tag().unwrap();

    println!("Memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!(
            "    start: 0x{:x}, length: 0x{:x}",
            area.start_address(),
            area.size()
        );
    }
}
