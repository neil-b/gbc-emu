use crate::registers::Registers;
use crate::memory::MemoryMapping;

pub fn jump(registers: &mut Registers, memory: &mut MemoryMapping) {
    let nn: u16 = memory.read16(registers.program_counter, 0);
    registers.program_counter = nn;
}

pub fn load_immediate8(program_counter: &mut u16, memory: &MemoryMapping, register: &mut u8) {
    let n: u8 = memory.read8(*program_counter, 0);
    *program_counter += 1;
    *register = n;
}

pub fn load_immediate16<F>(registers: &mut Registers, memory: &MemoryMapping, mut set_fn: F)
where F: FnMut(&mut Registers, u16) {
    let nn: u16 = memory.read16(registers.program_counter, 0);
    registers.program_counter += 2;
    set_fn(registers, nn);
}