#![allow(dead_code)]

extern crate ws;

#[macro_use]
mod utils;
mod gameboy;
mod cpu;
mod cartridge;
mod memory;

use cartridge::Cartridge;
use cpu::Cpu;

use gameboy::GameBoy;
use memory::Memory;
use std::env;

fn main() {

    // Load a ROM file and return a Cartridge
    let rom = Cartridge::new(env::args().nth(1).unwrap()).unwrap();

    // Init Cpu registers
    let mut cpu = Cpu::new();

    // Init memory
    let mut mem = Memory::new();

    // Plug all emulated componants into the GameBoy
    let mut gb = GameBoy::new(&mut cpu, &rom, &mut mem);
    gb.enable_debug();

    loop {
        cpu::opcodes::decode(&mut gb);
    }



}
