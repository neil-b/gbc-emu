use crate::memory::MemoryMapping;
use crate::registers::Registers;
use crate::instructions::*;

pub struct Emulator {
    registers: Registers,
    memory: MemoryMapping,
}

impl Emulator {
    pub fn new(rom_bytes: Vec<u8>) -> Emulator {
        let mut memory = MemoryMapping::new();
        memory.load_rom(rom_bytes);
        Emulator {
            registers: Registers::new(),
            memory,
        }
    }

    pub fn step(&mut self) {
        let opcode: u8 = self.memory.read8(self.registers.program_counter, 0);
        self.registers.program_counter += 1;
        self.handle_instruction(opcode);
    }

    fn handle_instruction(&mut self, opcode: u8) {
        // Opcode reference: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
        match opcode {
            0x00 => (), // No-op
            0xc3 => jump(&mut self.registers, &mut self.memory), // jump 2-bytes
            0xaf => self.registers.a ^= self.registers.a,
            0x21 => load_immediate16(&mut self.registers, &self.memory, &Registers::set_hl),
            0x0e => load_immediate8(&mut self.registers.program_counter, &self.memory, &mut self.registers.c),
            0x06 => load_immediate8(&mut self.registers.program_counter, &self.memory, &mut self.registers.b),
            // TODO:
            // 0x32 =>
            // 0x05 =>
            // 0x20
            _ => panic!("Unknown opcode {:#04x}", opcode),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
