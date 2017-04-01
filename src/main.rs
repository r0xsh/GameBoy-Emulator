#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod cpu;

fn main()
{
    let mut rom: Vec<u8> = Vec::new();
    let mut file=File::open("boot.bin").unwrap();
    let _ = file.read_to_end(&mut rom);
    println!("{:#x}", rom[0]);
}
