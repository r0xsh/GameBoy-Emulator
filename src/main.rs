#![allow(dead_code)]

extern crate lazy_static;
extern crate rustyline;

use std::env;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use cartridge::Cartridge;
use cpu::Cpu;
use gameboy::GameBoy;
use memory::Memory;
use debug::debug;

#[macro_use]
mod utils;
mod gameboy;
mod cpu;
mod cartridge;
mod memory;
mod debug;

fn main() {

    // Load a ROM file and return a Cartridge
    let rom = Cartridge::new(env::args().nth(1).unwrap()).unwrap();

    // Init Cpu registers
    let cpu = Cpu::new();

    // Init memory
    let mem = Memory::new();

    // Plug all emulated components into the GameBoy
    let gb = Arc::new(RwLock::new(GameBoy::new(cpu, rom, mem)));

    let pointer = gb.clone();

    //let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    thread::spawn(move || {
        debug(& *pointer.read().unwrap())
    });

    loop {
        //&mut *gb.clone().write().unwrap()
        cpu::opcodes::decode(&mut *gb.clone().write().unwrap());
    }


}
