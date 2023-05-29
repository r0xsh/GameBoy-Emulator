extern crate gameboy_emu;

use gameboy_emu::cartridge::Cartridge;
use gameboy_emu::cpu::{Cpu, Flag, Register16, Register8};
use gameboy_emu::cpu::opcodes::decode;
use gameboy_emu::gameboy::GameBoy;
use gameboy_emu::memory::Memory;
use gameboy_emu::utils::get_opcode_from_small;

fn init_env() -> GameBoy {
    let rom = Cartridge::empty(0xFFFF).unwrap();
    let cpu = Cpu::new();
    let mem = Memory::new(rom);
    let mut gb = GameBoy::new(cpu, mem);
    gb
}

#[test]
fn test_jr_d() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x18);
    gb.mem.write_byte(0x1001, 0x10);

    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1010);

    gb.mem.write_byte(0x1001, 0xFC);
    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1000 - 4);
}

#[test]
fn test_jr_cc_d() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, get_opcode_from_small(0, 4, 0, None, None));
    gb.mem.write_byte(0x1001, 0xFC);

    gb.cpu.set_flag(Flag::Z, true);
    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1002);

    gb.cpu.set_flag(Flag::Z, false);
    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1000 - 2);
}

#[test]
fn test_ld() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x01);
    gb.mem.write_byte(0x1001, 0x10);
    gb.mem.write_byte(0x1002, 0x00);

    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::BC), 0x0010);

    //LD (BC), A
    gb.mem.write_byte(0x1000, 0x02);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::BC, 0x1010);
    gb.cpu.set_8(Register8::A, 0x42);
    decode(&mut gb);

    assert_eq!(gb.mem.read_byte(0x1010), 0x42);

    //LD (DE), A
    gb.mem.write_byte(0x1000, 0x12);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::DE, 0x1011);
    gb.cpu.set_8(Register8::A, 0x42);
    decode(&mut gb);

    assert_eq!(gb.mem.read_byte(0x1011), 0x42);

    //LD HL, nn
    gb.mem.write_byte(0x1000, 0x21);
    gb.mem.write_byte(0x1001, 0x43);
    gb.mem.write_byte(0x1002, 0x42);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::HL, 0);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::HL), 0x4243);

    // LD (HL+), A
    gb.mem.write_byte(0x1000, 0x22);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::HL, 0x4242);
    gb.cpu.set_8(Register8::A, 0x42);

    decode(&mut gb);

    assert_eq!(gb.mem.read_word(0x4243), 0x42);
    assert_eq!(gb.cpu.get_16(Register16::HL), 0x4243);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    // LD (nn), SP
    gb.mem.write_byte(0x1000, 0x08);
    gb.mem.write_byte(0x1001, 0x00);
    gb.mem.write_byte(0x1002, 0x20);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::SP, 0x4242);
    decode(&mut gb);

    assert_eq!(gb.mem.read_word(0x2000), 0x4242);

    // LD (nn), A
    gb.mem.write_byte(0x1000, 0x32);
    gb.mem.write_byte(0x1001, 0x00);
    gb.mem.write_byte(0x1002, 0x20);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x42);
    decode(&mut gb);

    assert_eq!(gb.mem.read_byte(0x2000), 0x42);

    // LD A, (BC)
    gb.mem.write_byte(0x1000, 0x0A);
    gb.mem.write_byte(0x2020, 0x42);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::BC, 0x2020);
    gb.cpu.set_8(Register8::A, 0x0);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x42);

    // LD A, (DE)
    gb.mem.write_byte(0x1000, 0x1A);
    gb.mem.write_byte(0x2020, 0x42);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::DE, 0x2020);
    gb.cpu.set_8(Register8::A, 0x0);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x42);

    // LD HL, (nn)
    gb.mem.write_byte(0x1000, 0x2A);
    gb.mem.write_byte(0x1001, 0x20);
    gb.mem.write_byte(0x1002, 0x20);

    gb.mem.write_byte(0x2020, 0x42);
    gb.mem.write_byte(0x2021, 0x41);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::HL, 0);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::HL), 0x4142);

    // LD A, (nn)
    gb.mem.write_byte(0x1000, 0x3A);
    gb.mem.write_byte(0x1001, 0x20);
    gb.mem.write_byte(0x1002, 0x20);

    gb.mem.write_byte(0x2020, 0x42);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0);
    decode(&mut gb);

    // LD SP, HL
    gb.mem.write_byte(0x1000, 0xF9);
    gb.cpu.set_16(Register16::SP, 0x0000);
    gb.cpu.set_16(Register16::HL, 0x4243);
    gb.cpu.set_16(Register16::PC, 0x1000);
    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::SP), 0x4243);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
}

#[test]
fn test_inc_16() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x03);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::BC, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::BC), 0x4243);
}

#[test]
fn test_dec_16() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x0B);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::BC, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::BC), 0x4241);
}

#[test]
fn test_inc_8() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x04);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::B, 0x42);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0x43);
}

#[test]
fn test_dec_8() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x05);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::B, 0x42);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0x41);
}

#[test]
fn test_ld_8() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x06);
    gb.mem.write_byte(0x1001, 0x42);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::B, 0x0);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0x42);

    gb.mem.write_byte(0x1000, 0x41);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::B, 0x0);
    gb.cpu.set_8(Register8::C, 0x42);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0x42);

    //LD (HL-), A
    gb.mem.write_byte(0x1000, 0x32);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x42);
    gb.cpu.set_16(Register16::HL, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.mem.read_byte(0x4241), 0x42);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_16(Register16::HL), 0x4241);
}

#[test]
fn test_rlca() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x07);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x42);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x84);

    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x09);
}

#[test]
fn test_ret() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0xC9);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::SP, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4242);

    gb.mem.write_byte(0x1000, 0xC8);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::SP, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    gb.mem.write_byte(0x1000, 0xC8);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::SP, 0x4242);
    gb.cpu.set_flag(Flag::Z, true);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4242);
}

#[test]
fn test_jp() {
    let mut gb = init_env();

    // jp_nn
    gb.mem.write_byte(0x1000, 0xC3);
    gb.mem.write_byte(0x1001, 0x43);
    gb.mem.write_byte(0x1002, 0x42);

    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4243);

    // jp_HL
    gb.mem.write_byte(0x1000, 0xE9);
    gb.cpu.set_16(Register16::HL, 0x4342);

    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4342);

    // JP cc[y], nn
    gb.mem.write_byte(0x1000, 0xC2);
    gb.mem.write_byte(0x1001, 0x42);
    gb.mem.write_byte(0x1002, 0x42);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_flag(Flag::Z, true);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1003);

    // JP cc[y], nn
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_flag(Flag::Z, false);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4242);
}

#[test]
fn test_alu() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0x80);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x10);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x20);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    gb.mem.write_byte(0x1000, 0x80);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0xFF);
    gb.cpu.set_8(Register8::B, 0x10);
    gb.cpu.set_flag(Flag::C, false);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x0F);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_flag(Flag::C), true);

    gb.mem.write_byte(0x1000, 0x88);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x10);
    gb.cpu.set_flag(Flag::C, true);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x21);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    gb.mem.write_byte(0x1000, 0x88);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x10);
    gb.cpu.set_flag(Flag::C, false);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x20);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    gb.mem.write_byte(0x1000, 0x90);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x10);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x00);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);

    //SUB B+
    gb.mem.write_byte(0x1000, 0x90);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x10);
    gb.cpu.set_flag(Flag::C, false);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0x00);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x10);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_flag(Flag::C), false);

    //SUB B
    gb.mem.write_byte(0x1000, 0x90);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_8(Register8::A, 0x10);
    gb.cpu.set_8(Register8::B, 0x11);
    gb.cpu.set_flag(Flag::C, false);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_8(Register8::A), 0xFF);
    assert_eq!(gb.cpu.get_8(Register8::B), 0x11);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_flag(Flag::C), true);
}

#[test]
fn test_pop() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0xC1);
    gb.cpu.set_16(Register16::SP, 0x4242);
    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_16(Register16::BC), 0x4242);
}

#[test]
fn test_call() {
    let mut gb = init_env();

    //CALL NZ, nn
    gb.mem.write_byte(0x1000, 0xC4);
    gb.mem.write_byte(0x1001, 0x42);
    gb.mem.write_byte(0x1002, 0x42);

    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4242);

    //CALL NZ, nn
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_flag(Flag::Z, true);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1003);

    //CALL nn
    gb.mem.write_byte(0x1000, 0xCD);
    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x4242);
}

#[test]
fn test_add() {
    let mut gb = init_env();

    // ADD HL, rp[p]
    gb.mem.write_byte(0x1000, 0x09);

    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::BC, 0x4242);

    gb.cpu.set_16(Register16::HL, 0);
    assert_eq!(gb.cpu.get_16(Register16::HL), 0);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1001);
    assert_eq!(gb.cpu.get_16(Register16::HL), 0x4242);
}

#[test]
fn test_rst() {
    let mut gb = init_env();

    //RST x18
    gb.mem.write_byte(0x1000, 0xDF);
    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::SP), 0x1001);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x0018);

    //RST x08
    gb.mem.write_byte(0x1000, 0xCF);
    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(& mut gb);

    assert_eq!(gb.cpu.get_16(Register16::SP), 0x1001);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x0008);
}

#[test]
fn test_bit() {
    let mut gb = init_env();

    gb.cpu.reset_flags();

    //CB Bit 0, B
    gb.mem.write_byte(0x1000, 0xCB);
    gb.mem.write_byte(0x1001, 0x40);
    gb.cpu.set_8(Register8::B, 0);
    gb.cpu.set_16(Register16::PC, 0x1000);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_flag(Flag::Z), true);
    assert_eq!(gb.cpu.get_flag(Flag::N), false);
    assert_eq!(gb.cpu.get_flag(Flag::H), true);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x1002);

    //CB Bit 0, B
    gb.mem.write_byte(0x1000, 0xCB);
    gb.mem.write_byte(0x1001, 0x40);
    gb.cpu.set_8(Register8::B, 0b1000_0000);
    gb.cpu.set_16(Register16::PC, 0x1000);


    decode(&mut gb);

    assert_eq!(gb.cpu.get_flag(Flag::Z), true);
    assert_eq!(gb.cpu.get_flag(Flag::N), false);
    assert_eq!(gb.cpu.get_flag(Flag::H), true);

    //CB RES 0, B
    gb.mem.write_byte(0x1000, 0xCB);
    gb.mem.write_byte(0x1001, 0x80);
    gb.cpu.set_8(Register8::B, 0b1100_0000);
    gb.cpu.set_16(Register16::PC, 0x1000);


    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0b0100_0000);

    //CB SET 0, B
    gb.mem.write_byte(0x1000, 0xCB);
    gb.mem.write_byte(0x1001, 0xC0);
    gb.cpu.set_8(Register8::B, 0b0100_0000);
    gb.cpu.set_16(Register16::PC, 0x1000);


    decode(&mut gb);

    assert_eq!(gb.cpu.get_8(Register8::B), 0b1100_0000);
}

#[test]
fn test_push() {
    let mut gb = init_env();

    gb.mem.write_byte(0x1000, 0xD5);
    gb.cpu.set_16(Register16::PC, 0x1000);
    gb.cpu.set_16(Register16::DE, 0x4242);

    decode(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::SP), 0x4242);
}