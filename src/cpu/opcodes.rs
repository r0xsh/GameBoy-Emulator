use cpu::{Register8, Register16};
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
        0x31 => { ld_sp_nn(gb) },
        _ => {}
    }
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
