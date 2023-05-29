#![allow(dead_code)]

extern crate lazy_static;
extern crate rustyline;
extern crate gameboy_emu;

use std::{env, io};
use std::io::{stdout, Write};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;
use gameboy_emu::cartridge::Cartridge;
use gameboy_emu::cpu::{Cpu, Register16};
use gameboy_emu::cpu::opcodes::decode;
use gameboy_emu::debug::debug;
use gameboy_emu::gameboy::GameBoy;
use gameboy_emu::memory::Memory;

/*use cartridge::Cartridge;
use cpu::Cpu;
use gameboy::GameBoy;
use memory::Memory;
use debug::debug;*/

/*#[macro_use]
mod utils;
mod gameboy;
mod cpu;
mod cartridge;
mod memory;
mod debug;*/

fn main() {

    // Load a ROM file and return a Cartridge
    let rom = Box::new(Cartridge::new(env::args().nth(1).unwrap()).unwrap());

    // Init Cpu registers
    let cpu = Box::new(Cpu::new());

    // Init memory
    let mut mem = Box::new(Memory::new());
    //mem.write_from_file(0, String::from("/home/tiemajor/projects/perso/rust/GameBoy-Emulator/boot.bin"));

    let mut boot_rom =
        Memory::get_from_file(
            String::from("/home/tiemajor/projects/perso/rust/GameBoy-Emulator/boot.bin")
        );

    // Plug all emulated components into the GameBoy
    let mut gb = GameBoy::new(cpu, boot_rom, rom, mem);

    // let pointer = gb.clone();

    //let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    /*thread::spawn(move || {
        debug(& *pointer.read().unwrap())
    });*/

    let mut breakpoint: u16 = 0x64;
    let mut stepping = true;

    loop {

        if stepping || gb.cpu.get_16(Register16::PC) == breakpoint {
            println!("{:?}", gb.cpu);
            println!("op = {:#02x}",
                     gb.read_byte(gb.cpu.get_16(Register16::PC))
            );
            print!("$> ");
            stdout().lock().flush();
            stepping = true;
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);
            buffer = buffer.replace("\n", "");
            if buffer.starts_with("b") {
                buffer = buffer.replace("b 0x", "");
                breakpoint = u16::from_str_radix(buffer.as_str(), 16).unwrap_or(0);
                println!("Breakpoint set {:#04x}", breakpoint);
            }
            if buffer.eq("n") || buffer.eq("") {
                step(&mut gb);
            }
            if buffer.eq("c") {
                stepping = false;
            }
            if buffer.starts_with("p") {
                buffer = buffer.replace("p 0x", "");
                let addr = u16::from_str_radix(buffer.as_str(), 16).unwrap_or(0);
                println!("value @ {:#04x} = {:#04x}", addr, gb.read_byte(addr));
            }

        } else {
            step(&mut gb);
        }
        //&mut *gb.clone().write().unwrap()
    }

    fn step(mut gameboy: &mut GameBoy) {
        println!("PC = {:#04x}", gameboy.cpu.get_16(Register16::PC));
        println!("SCL = {}", gameboy.cpu.ppu.scanline);
        println!("ppu_tick : {}", gameboy.cpu.ppu.tick);

        decode(&mut gameboy);
        gameboy.cpu.ppu_step();
        gameboy.interrupt_step();
    }


}
