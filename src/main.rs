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
use cpu::op_decode;

use gameboy::GameBoy;
use memory::Memory;
use std::env;

fn main() {

    /*// Load a ROM file and return a Cartridge
    let rom = Cartridge::new(env::args().nth(1).unwrap()).unwrap();

    // Init Cpu registers
    let cpu = Cpu::new();

    // Init memory
    let mem = Memory::new();

    // Plug all emulated componants into the GameBoy
    let mut gb = GameBoy::new(cpu, rom, mem);

    gameboy::debugger::Debugger::new(&mut gb);

    for _ in 0..160 {
        cpu::opcodes::decode(&mut gb);
    }
*/
    let op:u8 = 0xff;
    println!("X={}, Y={}, Z={}, P={}, Q={}",
             cpu::op_decode::get_x(op),
             cpu::op_decode::get_y(op),
             cpu::op_decode::get_z(op),
             cpu::op_decode::get_p(op),
             cpu::op_decode::get_q(op));
}
