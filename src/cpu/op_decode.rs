#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use cpu::Register8;
use cpu::Register16;
use cpu::Flag;


lazy_static! {
    static pub ref TABLE_R: HashMap<u8, Register8> => {
        let mut m = HashMap::new();
        m.insert(0, Register8::B);
        m.insert(1, Register8::C);
        m.insert(2, Register8::D);
        m.insert(3, Register8::E);
        m.insert(4, Register8::H);
        m.insert(5, Register8::L);
        // m.insert(6, Register8::B); Uh oh this should be (HL) here
        m.insert(7, Register8::A);
    }
}

lazy_static! {
    static ref TABLE_RP: HashMap<u8, Register16> => {
        let mut m = HashMap::new();
        m.insert(0, Register16::BC);
        m.insert(1, Register16::DE);
        m.insert(2, Register16::HL);
        m.insert(3, Register16::SP);
    }
}

lazy_static! {
    static ref TABLE_RP2: HashMap<u8, Register16> => {
        let mut m = HashMap::new();
        m.insert(0, Register16::BC);
        m.insert(1, Register16::DE);
        m.insert(2, Register16::HL);
        m.insert(3, Register16::AF);
    }
}

lazy_static! {
    static ref TABLE_CC: HashMap<u8, Flag> => {
        let mut m = HashMap::new();
        m.insert(0, Flag::NZ);
        m.insert(1, Flag::Z);
        m.insert(2, Flag::NC);
        m.insert(3, Flag::C);
    }
}

pub fn get_x(opcode: u8) -> u8{
    let byte = (opcode & 0b1100_0000) as u8 >> 6;
    byte
}

pub fn get_y(opcode: u8) -> u8 {
    let byte = (opcode & 0b0011_1000) as u8 >> 3;
    byte
}

pub fn get_z(opcode: u8) -> u8{
    let byte = (opcode & 0b0000_0111) as u8;
    byte
}

pub fn get_p(opcode: u8) -> u8 {
    let y = get_y(opcode);
    let byte = (y & 0b0000_0110) as u8 >> 1;
    byte
}

pub fn get_q(opcode: u8) -> u8{
    let y = get_y(opcode);
    let byte = (y & 0b0000_0001) as u8;
    byte
}