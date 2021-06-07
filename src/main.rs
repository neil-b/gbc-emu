mod memory;
mod emulator;
mod registers;
mod instructions;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use crate::emulator::Emulator;

fn main() {
    match File::open("resources/tetris.gb") {
        Ok(mut f) => {
            let mut rom_bytes: Vec<u8> = Vec::new();
            f.read_to_end(&mut rom_bytes);
            let mut emu = Emulator::new(rom_bytes);

            loop {
                emu.step();
            }
        },
        Err(e) => panic!("Could not open rom {}", e),
    };
}
