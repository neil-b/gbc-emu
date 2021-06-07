use std::cmp::max;

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

    fn read8(&self, address: usize) -> u8 {
        match self.bytes.get(address) {
            Some(&value) => value,
            None => panic!("Bad read {}", address),
        }
    }

    fn write(&mut self, address: usize, new_value: u8) {
        match self.bytes.get_mut(address) {
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

const ROM_PAGE_SIZE: usize = 65536;
const VIDEO_RAM_SIZE: usize = 8192;
const CARTRIDGE_RAM_SIZE: usize = 8192;
const WORK_RAM_PAGE_SIZE: usize = 4096;
const WORK_RAM_PAGE_COUNT: usize = 8;

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
    pub fn new() -> MemoryMapping {
        MemoryMapping {
            rom_primary: MemoryBank::new(ROM_PAGE_SIZE),
            rom_secondary: PagedMemoryBank::new(ROM_PAGE_SIZE, 0),
            video_ram: MemoryBank::new(VIDEO_RAM_SIZE),
            cartridge_ram: MemoryBank::new(CARTRIDGE_RAM_SIZE),
            work_ram_primary: MemoryBank::new(WORK_RAM_PAGE_SIZE),
            work_ram_secondary: PagedMemoryBank::new(WORK_RAM_PAGE_SIZE, WORK_RAM_PAGE_COUNT - 1),
        }
    }

    pub fn load_rom(&mut self, rom_bytes: Vec<u8>) {
        let rom_page_count: usize = max(1, rom_bytes.len() / ROM_PAGE_SIZE);
        self.rom_secondary = PagedMemoryBank::new(ROM_PAGE_SIZE, rom_page_count - 1);

        let mut banks: Vec<&mut MemoryBank> = vec![&mut self.rom_primary];
        for mut page in &mut self.rom_secondary.pages {
            banks.push(page);
        }

        let chunks: Vec<&[u8]> = rom_bytes.chunks(ROM_PAGE_SIZE).collect();

        for chunk_index in 0..chunks.len() {
            for byte_index in 0..chunks[chunk_index].len() {
                banks[chunk_index].write(byte_index, chunks[chunk_index][byte_index]);
            }
        }
    }

    fn get_bank_mut(&mut self, address: u16, page_index: usize) -> &mut MemoryBank {
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

    // TODO: merge with get_bank_mut
    fn get_bank(&self, address: u16, page_index: usize) -> &MemoryBank {
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
            ROM_PRIMARY_START ..= ROM_PRIMARY_END => &self.rom_primary,
            ROM_SECONDARY_START ..= ROM_SECONDARY_END => self.rom_secondary.get_page(page_index),
            VIDEO_RAM_START ..= VIDEO_RAM_END => &self.video_ram,
            CARTRIDGE_RAM_START ..= CARTRIDGE_RAM_END => &self.cartridge_ram,
            WORK_RAM_PRIMARY_START ..= WORK_RAM_PRIMARY_END => &self.work_ram_primary,
            WORK_RAM_SECONDARY_START ..= WORK_RAM_SECONDARY_END => self.work_ram_secondary.get_page(page_index),
            _ => panic!("Read out of bounds at {}", address),
        }
    }

    pub fn read8(&self, address: u16, page_index: usize) -> u8 {
        self.get_bank(address, page_index).read8(address.into())
    }

    pub fn read16(&self, address: u16, page_index: usize) -> u16 {
        let bank = self.get_bank(address, page_index);
        let low: u8 = bank.read8(address.into());
        let high: u8 = bank.read8((address + 1).into());
        (high as u16) << 8 | (low as u16)
    }

    pub fn write(&mut self, address: u16, page_index: usize, new_value: u8) {
        self.get_bank_mut(address, page_index).write(address.into(), new_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_write_read8() {
        let mut bank = MemoryBank::new(100);
        bank.write(5, 123);
        assert_eq!(bank.read8(5), 123);
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
        bank.read8(100);
    }

    #[test]
    fn test_paged_bank_write_read8() {
        let mut bank = PagedMemoryBank::new(100, 5);
        bank.get_page_mut(4).write(5,123);
        bank.get_page_mut(3).write(3, 1);
        assert_eq!(bank.get_page(4).read8(5), 123);
        assert_eq!(bank.get_page(3).read8(3), 1);
    }

    #[test]
    #[should_panic]
    fn test_paged_bank_page_oob() {
        let bank = PagedMemoryBank::new(100, 5);
        bank.get_page(100);
    }

    #[test]
    fn test_load_rom() {
        const NUM_PAGES: usize = 4;
        const ADDRESS_OFFSET: usize = 50;
        let mut bytes = vec![1; ROM_PAGE_SIZE * NUM_PAGES];
        bytes[ADDRESS_OFFSET] = 2;
        bytes[ROM_PAGE_SIZE + ADDRESS_OFFSET] = 3;
        bytes[ROM_PAGE_SIZE * 2 + ADDRESS_OFFSET] = 4;

        let mut mapping = MemoryMapping::new();
        mapping.load_rom(bytes);
        assert_eq!(mapping.rom_secondary.pages.len(), NUM_PAGES - 1);
        assert_eq!(mapping.rom_primary.read8(ADDRESS_OFFSET), 2);
        assert_eq!(mapping.rom_secondary.get_page(0).read8(ADDRESS_OFFSET), 3);
        assert_eq!(mapping.rom_secondary.get_page(1).read8(ADDRESS_OFFSET), 4);
    }
}