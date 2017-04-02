#![allow(dead_code)]

use std::env;

mod cpu;
mod cartridge;

use cartridge::Cartridge;

fn main()
{
    println!("LOADING ROM...");
    let rom = Cartridge::new(env::args().nth(1).unwrap()).unwrap();
    println!("GAME: {}", rom.read_title());
    println!("{:?}", rom);
}
