use cpu::{Flag, Register8, Register16};
use GameBoy;

/// Iterate the ROM
pub fn decode(gb: &mut GameBoy) {
    for i in 0..gb.cartridge.size() {
        opcode_router(gb.cartridge.read_byte(i as u16), gb);
    }
    println!("{:?}", gb.cpu);
}

/// Map Opcode to function
fn opcode_router(opcode: u8, gb: &mut GameBoy) {
    match opcode {
        0x00 => {},
        0x01 => { ld_bc_nn(gb) },
        0x02 => { ld_bcp_a(gb) },
        0x03 => { inc_bc(gb) },
        0x04 => { inc_bc(gb) },
        0x05 => { unimplemented!() },
        0x06 => { unimplemented!() },
        0x07 => { unimplemented!() },
        0x08 => { unimplemented!() },
        0x09 => { unimplemented!() },
        0x0a => { unimplemented!() },
        0x0b => { unimplemented!() },
        0x0c => { unimplemented!() },
        0x0d => { unimplemented!() },
        0x0e => { unimplemented!() },
        0x0f => { unimplemented!() },
        0x10 => { unimplemented!() },
        0x11 => { unimplemented!() },
        0x12 => { unimplemented!() },
        0x13 => { unimplemented!() },
        0x14 => { unimplemented!() },
        0x15 => { unimplemented!() },
        0x16 => { unimplemented!() },
        0x17 => { unimplemented!() },
        0x18 => { unimplemented!() },
        0x19 => { unimplemented!() },
        0x1a => { unimplemented!() },
        0x1b => { unimplemented!() },
        0x1c => { unimplemented!() },
        0x1d => { unimplemented!() },
        0x1e => { unimplemented!() },
        0x1f => { unimplemented!() },
        0x20 => { unimplemented!() },
        0x21 => { unimplemented!() },
        0x22 => { unimplemented!() },
        0x23 => { unimplemented!() },
        0x24 => { unimplemented!() },
        0x25 => { unimplemented!() },
        0x26 => { unimplemented!() },
        0x27 => { unimplemented!() },
        0x28 => { unimplemented!() },
        0x29 => { unimplemented!() },
        0x2a => { unimplemented!() },
        0x2b => { unimplemented!() },
        0x2c => { unimplemented!() },
        0x2d => { unimplemented!() },
        0x2e => { unimplemented!() },
        0x2f => { unimplemented!() },
        0x30 => { unimplemented!() },
        0x31 => { ld_sp_nn(gb) },
        0x32 => { unimplemented!() },
        0x33 => { unimplemented!() },
        0x34 => { unimplemented!() },
        0x35 => { unimplemented!() },
        0x36 => { unimplemented!() },
        0x37 => { unimplemented!() },
        0x38 => { unimplemented!() },
        0x39 => { unimplemented!() },
        0x3a => { unimplemented!() },
        0x3b => { unimplemented!() },
        0x3c => { unimplemented!() },
        0x3d => { unimplemented!() },
        0x3e => { unimplemented!() },
        0x3f => { unimplemented!() },
        0x40 => { unimplemented!() },
        0x41 => { unimplemented!() },
        0x42 => { unimplemented!() },
        0x43 => { unimplemented!() },
        0x44 => { unimplemented!() },
        0x45 => { unimplemented!() },
        0x46 => { unimplemented!() },
        0x47 => { unimplemented!() },
        0x48 => { unimplemented!() },
        0x49 => { unimplemented!() },
        0x4a => { unimplemented!() },
        0x4b => { unimplemented!() },
        0x4c => { unimplemented!() },
        0x4d => { unimplemented!() },
        0x4e => { unimplemented!() },
        0x4f => { unimplemented!() },
        0x50 => { unimplemented!() },
        0x51 => { unimplemented!() },
        0x52 => { unimplemented!() },
        0x53 => { unimplemented!() },
        0x54 => { unimplemented!() },
        0x55 => { unimplemented!() },
        0x56 => { unimplemented!() },
        0x57 => { unimplemented!() },
        0x58 => { unimplemented!() },
        0x59 => { unimplemented!() },
        0x5a => { unimplemented!() },
        0x5b => { unimplemented!() },
        0x5c => { unimplemented!() },
        0x5d => { unimplemented!() },
        0x5e => { unimplemented!() },
        0x5f => { unimplemented!() },
        0x60 => { unimplemented!() },
        0x61 => { unimplemented!() },
        0x62 => { unimplemented!() },
        0x63 => { unimplemented!() },
        0x64 => { unimplemented!() },
        0x65 => { unimplemented!() },
        0x66 => { unimplemented!() },
        0x67 => { unimplemented!() },
        0x68 => { unimplemented!() },
        0x69 => { unimplemented!() },
        0x6a => { unimplemented!() },
        0x6b => { unimplemented!() },
        0x6c => { unimplemented!() },
        0x6d => { unimplemented!() },
        0x6e => { unimplemented!() },
        0x6f => { unimplemented!() },
        0x70 => { unimplemented!() },
        0x71 => { unimplemented!() },
        0x72 => { unimplemented!() },
        0x73 => { unimplemented!() },
        0x74 => { unimplemented!() },
        0x75 => { unimplemented!() },
        0x76 => { unimplemented!() },
        0x77 => { unimplemented!() },
        0x78 => { unimplemented!() },
        0x79 => { unimplemented!() },
        0x7a => { unimplemented!() },
        0x7b => { unimplemented!() },
        0x7c => { unimplemented!() },
        0x7d => { unimplemented!() },
        0x7e => { unimplemented!() },
        0x7f => { unimplemented!() },
        0x80 => { unimplemented!() },
        0x81 => { unimplemented!() },
        0x82 => { unimplemented!() },
        0x83 => { unimplemented!() },
        0x84 => { unimplemented!() },
        0x85 => { unimplemented!() },
        0x86 => { unimplemented!() },
        0x87 => { unimplemented!() },
        0x88 => { unimplemented!() },
        0x89 => { unimplemented!() },
        0x8a => { unimplemented!() },
        0x8b => { unimplemented!() },
        0x8c => { unimplemented!() },
        0x8d => { unimplemented!() },
        0x8e => { unimplemented!() },
        0x8f => { unimplemented!() },
        0x90 => { unimplemented!() },
        0x91 => { unimplemented!() },
        0x92 => { unimplemented!() },
        0x93 => { unimplemented!() },
        0x94 => { unimplemented!() },
        0x95 => { unimplemented!() },
        0x96 => { unimplemented!() },
        0x97 => { unimplemented!() },
        0x98 => { unimplemented!() },
        0x99 => { unimplemented!() },
        0x9a => { unimplemented!() },
        0x9b => { unimplemented!() },
        0x9c => { unimplemented!() },
        0x9d => { unimplemented!() },
        0x9e => { unimplemented!() },
        0x9f => { unimplemented!() },
        0xa0 => { unimplemented!() },
        0xa1 => { unimplemented!() },
        0xa2 => { unimplemented!() },
        0xa3 => { unimplemented!() },
        0xa4 => { unimplemented!() },
        0xa5 => { unimplemented!() },
        0xa6 => { unimplemented!() },
        0xa7 => { unimplemented!() },
        0xa8 => { unimplemented!() },
        0xa9 => { unimplemented!() },
        0xaa => { unimplemented!() },
        0xab => { unimplemented!() },
        0xac => { unimplemented!() },
        0xad => { unimplemented!() },
        0xae => { unimplemented!() },
        0xaf => { xor(gb, Register8::A) }
        0xb0 => { unimplemented!() },
        0xb1 => { unimplemented!() },
        0xb2 => { unimplemented!() },
        0xb3 => { unimplemented!() },
        0xb4 => { unimplemented!() },
        0xb5 => { unimplemented!() },
        0xb6 => { unimplemented!() },
        0xb7 => { unimplemented!() },
        0xb8 => { unimplemented!() },
        0xb9 => { unimplemented!() },
        0xba => { unimplemented!() },
        0xbb => { unimplemented!() },
        0xbc => { unimplemented!() },
        0xbd => { unimplemented!() },
        0xbe => { unimplemented!() },
        0xbf => { unimplemented!() },
        0xc0 => { unimplemented!() },
        0xc1 => { unimplemented!() },
        0xc2 => { unimplemented!() },
        0xc3 => { unimplemented!() },
        0xc4 => { unimplemented!() },
        0xc5 => { unimplemented!() },
        0xc6 => { unimplemented!() },
        0xc7 => { unimplemented!() },
        0xc8 => { unimplemented!() },
        0xc9 => { unimplemented!() },
        0xca => { unimplemented!() },
        0xcb => { unimplemented!() },
        0xcc => { unimplemented!() },
        0xcd => { unimplemented!() },
        0xce => { unimplemented!() },
        0xcf => { unimplemented!() },
        0xd0 => { unimplemented!() },
        0xd1 => { unimplemented!() },
        0xd2 => { unimplemented!() },
        0xd3 => { unimplemented!() },
        0xd4 => { unimplemented!() },
        0xd5 => { unimplemented!() },
        0xd6 => { unimplemented!() },
        0xd7 => { unimplemented!() },
        0xd8 => { unimplemented!() },
        0xd9 => { unimplemented!() },
        0xda => { unimplemented!() },
        0xdb => { unimplemented!() },
        0xdc => { unimplemented!() },
        0xdd => { unimplemented!() },
        0xde => { unimplemented!() },
        0xdf => { unimplemented!() },
        0xe0 => { unimplemented!() },
        0xe1 => { unimplemented!() },
        0xe2 => { unimplemented!() },
        0xe3 => { unimplemented!() },
        0xe4 => { unimplemented!() },
        0xe5 => { unimplemented!() },
        0xe6 => { unimplemented!() },
        0xe7 => { unimplemented!() },
        0xe8 => { unimplemented!() },
        0xe9 => { unimplemented!() },
        0xea => { unimplemented!() },
        0xeb => { unimplemented!() },
        0xec => { unimplemented!() },
        0xed => { unimplemented!() },
        0xee => { unimplemented!() },
        0xef => { unimplemented!() },
        0xf0 => { unimplemented!() },
        0xf1 => { unimplemented!() },
        0xf2 => { unimplemented!() },
        0xf3 => { unimplemented!() },
        0xf4 => { unimplemented!() },
        0xf5 => { unimplemented!() },
        0xf6 => { unimplemented!() },
        0xf7 => { unimplemented!() },
        0xf8 => { unimplemented!() },
        0xf9 => { unimplemented!() },
        0xfa => { unimplemented!() },
        0xfb => { unimplemented!() },
        0xfc => { unimplemented!() },
        0xfd => { unimplemented!() },
        0xfe => { unimplemented!() },
        0xff => { unimplemented!() },
        _ => {}
    }
}


/// XOR generic function
fn xor(gb: &mut GameBoy, reg: Register8) {
    let mut acc = gb.cpu.get_8(Register8::A);
    acc ^= gb.cpu.get_8(reg);
    gb.cpu.set_8(Register8::A, acc);
    gb.cpu.reset_flags();
    match acc {
        0 => gb.cpu.set_flag(Flag::Z, true),
        _ => gb.cpu.set_flag(Flag::Z, false),
    }
    gb.cpu.inc_pc(1);
}

/// OR generic function
fn or(gb: &mut GameBoy, reg: Register8) {
    let mut acc = gb.cpu.get_8(Register8::A);
    acc |= gb.cpu.get_8(reg);
    gb.cpu.set_8(Register8::A, acc);
    gb.cpu.reset_flags();
    match acc {
        0 => gb.cpu.set_flag(Flag::Z, true),
        _ => gb.cpu.set_flag(Flag::Z, false),
    }
    gb.cpu.inc_pc(1);
}

/// AND generic function
fn and(gb: &mut GameBoy, reg: Register8) {
    let mut acc = gb.cpu.get_8(Register8::A);
    acc &= gb.cpu.get_8(reg);
    gb.cpu.set_8(Register8::A, acc);
    gb.cpu.reset_flags();
    match acc {
        0 => gb.cpu.set_flag(Flag::Z, true),
        _ => gb.cpu.set_flag(Flag::Z, false),
    }
    gb.cpu.inc_pc(1);
}

/// LD BC, nn 0x01
fn ld_bc_nn(gb: &mut GameBoy) {
    let v: u16 = gb.cartridge.read_word(gb.cpu.get_16(Register16::PC) + 1);
    gb.cpu.set_16(Register16::BC, v);
    gb.cpu.inc_pc(3);

}

/// LD (BC), a 0x02
fn ld_bcp_a(gb: &mut GameBoy) {
    gb.mem.write_byte(gb.cpu.get_16(Register16::BC), gb.cpu.get_8(Register8::A));
    gb.cpu.inc_pc(1);
}

/// INC BC, 0x03
fn inc_bc(gb: &mut GameBoy) {
    gb.cpu.inc_16(Register16::BC);
    gb.cpu.inc_pc(1);
}

/// INC B, 0x04
fn inc_b(gb: &mut GameBoy) {
    gb.cpu.inc_8(Register8::B);
    gb.cpu.inc_pc(1);
}

/// LD SP, nn 0x31
fn ld_sp_nn(gb: &mut GameBoy) {
    let v: u16 = gb.cartridge.read_word(gb.cpu.get_16(Register16::PC) + 1);
    gb.cpu.set_16(Register16::SP, v);
    gb.cpu.inc_pc(3);
}
