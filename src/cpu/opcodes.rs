use cpu::{Register16};
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
        0x00 => {}
        0x31 => { ld_sp_nn(gb) },
        _ => {}
    }
}

/// LD BC, nn 0x01
fn ld_sp_nn(gb: &mut GameBoy) {
    let v: u16 = gb.cartridge.read_word(gb.cpu.get_16(Register16::PC) + 1);
    gb.cpu.set_16(Register16::SP, v);
}
