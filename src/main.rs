#![allow(dead_code)]

use std::env;

mod cpu;
mod cartridge;

use cartridge::Cartridge;

fn main()
{
    let oui = Cartridge::new(env::args().nth(1).unwrap()).unwrap();
    println!("{:?}", oui.read_range(0x0134, 0x0143));
}
