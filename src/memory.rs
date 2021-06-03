#[derive(Clone)]
struct MemoryBank {
    bytes: Vec<u8>,
}

impl MemoryBank {
    fn new(size: usize) -> MemoryBank {
        MemoryBank {
            bytes: vec![0; size],
        }
    }

    fn read(&self, address: u16) -> u8 {
        match self.bytes.get(address as usize) {
            Some(&value) => value,
            None => panic!("Bad read {}", address),
        }
    }

    fn write(&mut self, address: u16, new_value: u8) {
        match self.bytes.get_mut(address as usize) {
            Some(value) => *value = new_value,
            None => panic!("Bad write {}", address),
        };
    }
}

struct PagedMemoryBank {
    pages: Vec<MemoryBank>,
}

impl PagedMemoryBank {
    fn new(size: usize, num_swappable_pages: usize) -> PagedMemoryBank {
        PagedMemoryBank {
            pages: vec![MemoryBank::new(size); num_swappable_pages]
        }
    }

    fn get_page_mut(&mut self, page_index: usize) -> &mut MemoryBank {
        match self.pages.get_mut(page_index) {
            Some(page) => page,
            None => panic!("Bad get of page {}", page_index)
        }
    }

    fn get_page(&self, page_index: usize) -> &MemoryBank {
        match self.pages.get(page_index) {
            Some(page) => &page,
            None => panic!("Bad get of page {}", page_index)
        }
    }
}

// Maps 16 bit memory addresses to the appropriate MemoryBank
pub struct MemoryMapping {
    rom_primary: MemoryBank,
    rom_secondary: PagedMemoryBank,
    video_ram: MemoryBank,
    cartridge_ram: MemoryBank, // May need to be swappable in the future
    work_ram_primary: MemoryBank,
    work_ram_secondary: PagedMemoryBank,
}

impl MemoryMapping {
    pub fn new(rom_page_count: usize) -> MemoryMapping {
        const ROM_PAGE_SIZE: usize = 65536;
        const VIDEO_RAM_SIZE: usize = 8192;
        const CARTRIDGE_RAM_SIZE: usize = 8192;
        const WORK_RAM_PAGE_SIZE: usize = 4096;
        const WORK_RAM_PAGE_COUNT: usize = 8;

        MemoryMapping {
            rom_primary: MemoryBank::new(ROM_PAGE_SIZE),
            rom_secondary: PagedMemoryBank::new(ROM_PAGE_SIZE, rom_page_count - 1),
            video_ram: MemoryBank::new(VIDEO_RAM_SIZE),
            cartridge_ram: MemoryBank::new(CARTRIDGE_RAM_SIZE),
            work_ram_primary: MemoryBank::new(WORK_RAM_PAGE_SIZE),
            work_ram_secondary: PagedMemoryBank::new(WORK_RAM_PAGE_SIZE, WORK_RAM_PAGE_COUNT - 1),
        }
    }

    pub fn get_bank_mut(&mut self, address: u16, page_index: usize) -> &mut MemoryBank {
        const ROM_PRIMARY_START: u16 = 0x0000;
        const ROM_PRIMARY_END: u16 = 0x3FFF;
        const ROM_SECONDARY_START: u16 = 0x4000;
        const ROM_SECONDARY_END: u16 = 0x7FFF;
        const VIDEO_RAM_START: u16 = 0x8000;
        const VIDEO_RAM_END: u16 = 0x9FFF;
        const CARTRIDGE_RAM_START: u16 = 0xA000;
        const CARTRIDGE_RAM_END: u16 = 0xBFFF;
        const WORK_RAM_PRIMARY_START: u16 = 0xC000;
        const WORK_RAM_PRIMARY_END: u16 = 0xCFFF;
        const WORK_RAM_SECONDARY_START: u16 = 0xD000;
        const WORK_RAM_SECONDARY_END: u16 = 0xDFFF;
        match address {
            ROM_PRIMARY_START ..= ROM_PRIMARY_END => &mut self.rom_primary,
            ROM_SECONDARY_START ..= ROM_SECONDARY_END => self.rom_secondary.get_page_mut(page_index),
            VIDEO_RAM_START ..= VIDEO_RAM_END => &mut self.video_ram,
            CARTRIDGE_RAM_START ..= CARTRIDGE_RAM_END => &mut self.cartridge_ram,
            WORK_RAM_PRIMARY_START ..= WORK_RAM_PRIMARY_END => &mut self.work_ram_primary,
            WORK_RAM_SECONDARY_START ..= WORK_RAM_SECONDARY_END => self.work_ram_secondary.get_page_mut(page_index),
            _ => panic!("Read out of bounds at {}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_write_read() {
        let mut bank = MemoryBank::new(100);
        bank.write(5, 123);
        assert_eq!(bank.read(5), 123);
    }

    #[test]
    #[should_panic]
    fn test_bank_write_oob() {
        let mut bank = MemoryBank::new(100);
        bank.write(100, 0);
    }

    #[test]
    #[should_panic]
    fn test_bank_read_oob() {
        let bank = MemoryBank::new(100);
        bank.read(100);
    }

    #[test]
    fn test_paged_bank_write_read() {
        let mut bank = PagedMemoryBank::new(100, 5);
        bank.get_page_mut(4).write(5,123);
        bank.get_page_mut(3).write(3, 1);
        assert_eq!(bank.get_page(4).read(5), 123);
        assert_eq!(bank.get_page(3).read(3), 1);
    }

    #[test]
    #[should_panic]
    fn test_paged_bank_page_oob() {
        let bank = PagedMemoryBank::new(100, 5);
        bank.get_page(100);
    }
}