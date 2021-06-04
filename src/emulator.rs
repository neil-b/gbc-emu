use crate::memory::MemoryMapping;

pub struct Emulator {
    memory: MemoryMapping,
}

impl Emulator {
    pub fn new(rom_bytes: Vec<u8>) -> Emulator {
        let mut memory = MemoryMapping::new();
        memory.load_rom(rom_bytes);
        Emulator {
            memory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}