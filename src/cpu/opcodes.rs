use Cpu;
use GameBoy;

pub fn decode(gb: &mut GameBoy) {
    for i in 0..gb.cartridge.size() {
        opcode_router(gb.cartridge.read_byte(i as u16), &mut gb.cpu);
    }
}

fn opcode_router(opcode: u8, _: &mut Cpu) {
    match opcode {
        0x00 => {}
        0x31 => println!("oui"),
        _ => {}
    }
}

/// NOP 0x00
fn nop(_: &mut Cpu) {}

/// LD BC, nn 0x01
fn ld_bc_nn(_: &mut Cpu) {}
