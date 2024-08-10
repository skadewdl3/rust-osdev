use crate::memory::{frame::Frame, paging::entry::EntryFlags};

use super::{inactive_page_table::InactivePageTable, mapper::Mapper, page::TemporaryPage};
use core::ops::{Deref, DerefMut};

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F>(
        &mut self,
        table: &mut InactivePageTable,
        temporary_page: &mut TemporaryPage, // new
        f: F,
    ) where
        F: FnOnce(&mut Mapper),
    {
        use x86_64::instructions::tlb;
        use x86_64::registers::control::Cr3;
        let p4_table_address = Cr3::read().0.start_address().as_u64();
        let backup_frame = Frame::containing_address(p4_table_address);
        let p4_table = temporary_page.map_table_frame(backup_frame.clone(), self);

        // overwrite recursive mapping
        self.mapper.p4_mut()[511].set(
            table.p4_frame.clone(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        tlb::flush_all();

        // execute f in the new context
        f(self);
        p4_table[511].set(backup_frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        tlb::flush_all();

        crate::serial_println!("page from with {}", temporary_page.page.p4_index());

        temporary_page.unmap(self);
    }
}
