use ::GameBoy;
use cpu::{Flag, Register16, Register8};
use cpu::opcode::Opcode;

/// Iterate the ROM
pub fn decode(gb: &mut GameBoy) {
    let mut op = Opcode::from(gb.cartridge.read_byte(gb.cpu.get_16(Register16::PC)));
    op.fetch_param(&gb);

    // Debug
    println!("op = {:?}, pc = {}", op, gb.cpu.get_16(Register16::PC));

    gb.cpu.inc_pc(op.length);

    // Call a function who get the current opcode and a mutable gameboy instance
    // should incr the PC and returns arguments



    // X, Y, Z, P, Q
    match op.flag_slice() {
        // X = 0
        // Z = 0
        (0, 0, 0, _, _) => {
            // NOOP
            op_nop()
        },
        (0, 1, 0, _, _) => {
            // LD (nn), SP
            op_ld_mem16(gb, op.param.unwrap(), gb.cpu.get_16(Register16::SP))
        },
        (0, 2, 0, _, _) => {
            // STOP
            op_nop()
        },
        (0, 3, 0, _, _) => {
            // JR d
            let pc_val = gb.cpu.get_16(Register16::PC);
            gb.cpu.set_16(Register16::PC, pc_val + 1 + gb.mem.read_byte(pc_val) as u16);
        },
        (0, 4..=7, 0, _, _) => {
            // JR cc[y-4], d
            if gb.get_table_cc(op.y - 4) {
                let pc_val = gb.cpu.get_16(Register16::PC);
                gb.cpu.set_16(Register16::PC, pc_val + 1 + gb.mem.read_byte(pc_val) as u16);
            }
        },
        // Z = 1
        (0, _, 1, _, 0) => {
            // LD rp[p], nn
            op_ld_reg16(gb, op.q, op.param.unwrap())
        },
        (0, _, 1, _, 1) => {
            // ADD HL, rp[p]
            op_add16(gb, Register16::HL, op.p)
        },

        // Z = 2
        // Q = 0
        (0, _, 2, 0, 0) => {
            // LD (BC), A
            op_ld_mem8(gb, gb.cpu.get_16(Register16::BC), gb.cpu.get_8(Register8::A))
        },
        (0, _, 2, 1, 0)  => {
            // LD (DE), A
            op_ld_mem8(gb, gb.cpu.get_16(Register16::DE), gb.cpu.get_8(Register8::A))
        },
        (0, _, 2, 2, 0)  => {
            // LD (HL+), A
            op_ld_mem8(gb, gb.cpu.get_16(Register16::HL), gb.cpu.get_8(Register8::A));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) + 1);
        },
        (0, _, 2, 3, 0)  => {
            // LD (HL-), A
            op_ld_mem8(gb, gb.cpu.get_16(Register16::HL), gb.cpu.get_8(Register8::A));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) - 1);
        },
        // Q = 1
        (0, _, 2, 0, 1)  => {
            // LD A, (BC)
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::BC)))
        },
        (0, _, 2, 1, 1)  => {
            // LD A, (DE)
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::DE)))
        },
        (0, _, 2, 2, 1)  => {
            // LD A, (HL+)
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::HL)));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) + 1);
        },
        (0, _, 2, 3, 1) => {
            // LD A, (HL-)
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::HL)));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) - 1);
        },
        // Z = 3
        (0, _, 3, _, 0) => {
            // INC rp[p]
            op_inc16(gb, op.p)
        },
        (0, _, 3, _, 1) => {
            // DEC rp[p]
            op_dec16(gb, op.p)
        },
        // Z = 4
        (0, _, 4, _, _) => {
            // INC r[y]
            op_inc8(gb, op.y)
        },
        (0, _, 5, _, _) => {
            // DEC r[y]
            op_dec8(gb, op.y)
        },
        (0, _, 6, _, _) => {
            // LD r[y], n
            op_ld_8(gb, op.y, op.param.unwrap() as u8)
        },

        (0, 0, 7, _, _) => rlca(gb), // RLCA
        (0, 1, 7, _, _) => rrca(gb), // RRCA
        (0, 2, 7, _, _) => rla(gb), // RLA
        (0, 3, 7, _, _) => rra(gb), // RRA
        (0, 4, 7, _, _) => op_nop(), // DAA //TODO: Impl DAA
        (0, 5, 7, _, _) => cpl(gb), // CPL
        (0, 6, 7, _, _) => scf(gb), // SCF
        (0, 7, 7, _, _) => ccf(gb), // CCF
        // X = 1
        (1, 6, 6, _, _) => op_nop(), // HALT
        (1, _, _, _, _) => {
            // LD r[y], r[z]
            let operand = gb.get_table_r(op.z);
            op_ld_8(gb, op.y, operand);
        },
        // X = 2
        (2, _, _, _, _) => {
            call_alu_table_r(gb, op.z, op.y)
        }, // alu[y] r[z]
        // X = 3
        (3, 0..=3, 0, _, _) => {
            if gb.get_table_cc(op.y) {
                op_ret(gb);
            }
        }, // RET cc[y]
        (3, 4, 0, _, _) => {
            op_nop()
        }, // LD (0xFF00 + nn), A
        (3, 5, 0, _, _) => {
            op_nop()
        }, // ADD SP, d
        (3, 6, 0, _, _) => {
            op_nop()
        }, // LD A, (0xFF00 + n)
        (3, 7, 0, _, _) => {
            op_nop()
        }, // LD HL, SP+ d

        (3, _, 1, _, 0) => {
            op_nop()
        }, // POP rp2[p]
        (3, _, 1, 0, 1) => {
            op_ret(gb);
        }, // RET
        (3, _, 1, 1, 1) => {
            op_nop()
        }, // RETI
        (3, _, 1, 2, 1) => {
            op_nop()
        }, // JP HL
        (3, _, 1, 3, 1) => {
            op_nop()
        }, // LD SP, HL

        (3, 0..=3, 2, _, _) => {
            op_nop()
        }, // JP cc[y], nn
        (3, 4, 2, _, _) => {
            op_nop()
        }, // LD (0xFF00+C), A
        (3, 5, 2, _, _) => {
            // LD (nn), A
            let address = gb.mem.read_word(gb.cpu.get_16(Register16::PC) + 1);
            let operand = gb.cpu.get_8(Register8::A);
            op_ld_mem8(gb, address, operand)
        },
        (3, 6, 2, _, _) => {
            op_nop()
        }, // LD A, (0xFF00+C)
        (3, 7, 2, _, _) => {
            // LD A, (nn)
            let operand = gb.mem.read_byte(gb.cpu.get_16(Register16::PC) + 1);
            op_ld_reg8(gb, Register8::A, operand)
        },

        (3, 0, 3, _, _) => {
            op_nop()
        }, // JP nn
        (3, 1, 3, _, _) => {
            op_nop()
        }, // CB Prefix
        (3, 6, 3, _, _) => {
            op_nop()
        }, // DI
        (3, 7, 3, _, _) => {
            op_nop()
        }, // EI

        (3, 0..=3, 4, _, _) => {
            op_nop()
        }, // CALL cc[y], nn

        (3, _, 5, _, 0) => {
            op_nop()
        }, // PUSH rp2[p]
        (3, _, 5, 0, 1) => {
            op_nop()
        }, // CALL nn

        (3, _, 6, _, _) => {
            // alu[y] n
            exec_alu(gb, op.y, gb.cartridge.read_byte(gb.cpu.get_16(Register16::PC) + 1))
        },

        (3, _, 7, _, _) => {
            op_nop()
        }, // RST y*8
        _ => op_nop(),
    }
}

fn op_nop(){
    print!("nop")
}

fn add8(gb: &mut GameBoy, val: u8, add: u8) -> u8 {
    gb.cpu.set_flag(Flag::H, (val & 0x0F) == 0x0F);
    let val = (val + add) as u16;
    gb.cpu.set_flag(Flag::C, val > 0xFF);
    gb.cpu.set_flag(Flag::Z, val == 0);
    //gb.cpu.set_flag(Flag::S, (val > 0x7F) && (val < 0xFF));
    gb.cpu.set_flag(Flag::N, false);
    val as u8
}

fn add16(gb: &mut GameBoy, val: u16, add: u16) -> u16 {
    gb.cpu.set_flag(Flag::H, ((val & 0x0F) + (add & 0x0F)) > 0x0F);
    let val = (val + add) as u32;
    gb.cpu.set_flag(Flag::C, val > 0xFFFF);
    gb.cpu.set_flag(Flag::N, false);
    val as u16
}

fn minus8(gb: &mut GameBoy, val: u8, minus: u8) -> u8 {
    gb.cpu.set_flag(Flag::H, val & 0x0F == 0);
    let val = (val - minus) as i16;
    gb.cpu.set_flag(Flag::C, val < 0);
    gb.cpu.set_flag(Flag::Z, val == 0);
    gb.cpu.set_flag(Flag::N, true);
    val as u8
}

fn op_ld_reg8(gb: &mut GameBoy, reg: Register8, operand: u8) {
    gb.cpu.set_8(reg, operand);
}

fn op_ld_reg16(gb: &mut GameBoy, index: u8, operand: u16) {
    gb.set_table_rp(index, operand);
}

fn op_ld_mem8(gb: &mut GameBoy, address: u16, operand: u8) {
    gb.mem.write_byte(address, operand);
}

fn op_ld_mem16(gb: &mut GameBoy, address: u16, operand: u16) {
    gb.mem.write_word(address, operand);
}

fn op_ld_8(gb: &mut GameBoy, index: u8, operand: u8) {
    gb.set_table_r(index, operand);
}

fn op_inc8(gb: &mut GameBoy, index: u8) {
    let res = add8(gb, gb.get_table_r(index), 1);
    gb.set_table_r(index, res);
}

fn op_inc16(gb: &mut GameBoy, index: u8) {
    let res = gb.get_table_rp(index) + 1;
    gb.set_table_rp(index, res);
}

fn op_dec16(gb: &mut GameBoy, index: u8) {
    let res = gb.get_table_rp(index) - 1;
    gb.set_table_rp(index, res);
}

fn op_dec8(gb: &mut GameBoy, index: u8) {
    let res = minus8(gb, gb.get_table_r(index), 1);
    gb.set_table_r(index, res);
}

fn op_add8(gb: &mut GameBoy, index: u8, operand: u8) {
    let res = add8(gb, gb.get_table_r(index), operand);
    gb.set_table_r(index, res);
}

fn op_add16(gb: &mut GameBoy, reg: Register16, index: u8) {
        let res = add16(gb, gb.get_table_rp(index), gb.cpu.get_16(reg));
        gb.cpu.set_16(reg, res);
}

pub fn call_alu_table_r(gb: &mut GameBoy, z: u8, y: u8) {
    match z {
        0 => exec_alu(gb, y, gb.cpu.get_8(Register8::B)),
        1 => exec_alu(gb, y, gb.cpu.get_8(Register8::C)),
        2 => exec_alu(gb, y, gb.cpu.get_8(Register8::D)),
        3 => exec_alu(gb, y, gb.cpu.get_8(Register8::E)),
        4 => exec_alu(gb, y, gb.cpu.get_8(Register8::H)),
        5 => exec_alu(gb, y, gb.cpu.get_8(Register8::L)),
        6 => exec_alu(gb, y, gb.mem.read_byte(gb.cpu.get_16(Register16::HL))),
        7 => exec_alu(gb, y, gb.cpu.get_8(Register8::A)),
        _ => {},
    }
}

fn exec_alu(gb: &mut GameBoy, y: u8, operand: u8) {
    match y {
        0 => alu_add_a(gb, operand),
        1 => alu_adc_a(gb, operand),
        2 => alu_sub(gb, operand),
        3 => alu_sbc(gb, operand),
        4 => alu_and(gb, operand),
        5 => alu_xor(gb, operand),
        6 => alu_or(gb, operand),
        7 => alu_cp(gb, operand),
        _ => {},
    }
}


/* ------------------------

 Assorted operations on accumulator/flags

---------------------------*/

/// RLCA rotates the A register to the left one place. The 7th bit is
/// put back into the 0 position.  The 7th bit also goes to the carry flag.
fn rlca(gb: &mut GameBoy) {
    let a = gb.cpu.get_8(Register8::A);
    let seventh = a >> 7;
    let rotated = a << 1 | seventh;

    gb.cpu.set_8(Register8::A, rotated);
    gb.cpu.set_flag(Flag::C, seventh == 0b1);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
}

/// The  register is shifted right by one,  and the 0 bit goes to the
/// carry flag and to the 7th bit.  Flags apart from the carry are as for RLCA.
fn rrca(gb: &mut GameBoy) {
    let a = gb.cpu.get_8(Register8::A);
    let first = a << 7;
    let rotated = a >> 1 | first;

    gb.cpu.set_8(Register8::A, rotated);
    gb.cpu.set_flag(Flag::C, first == 0b1);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
}

/// The  bits in the register are all rotated left,  the 7th bit goes
/// to the carry flag and the carry flag goes to bit 0. Flags apart from the
/// carry flag are as for RLCA.
fn rla(gb: &mut GameBoy) {
    let a = gb.cpu.get_8(Register8::A);
    let carry = gb.cpu.get_flag(Flag::C) as u8;
    let seventh = a >> 7;
    let rotated = a << 1 | carry;

    gb.cpu.set_8(Register8::A, rotated);
    gb.cpu.set_flag(Flag::C, seventh == 0b1);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
}

/// The register is shifted right by one, the 0 bit goes to the carry
/// flag, and  the carry flag goes to bit 7. Flags apart from the
/// carry flag are as for RLCA.
fn rra(gb: &mut GameBoy) {
    let a = gb.cpu.get_8(Register8::A);
    let carry = gb.cpu.get_flag(Flag::C) as u8;
    let first = a << 7;
    let rotated = a >> 1 | carry;

    gb.cpu.set_8(Register8::A, rotated);
    gb.cpu.set_flag(Flag::C, first == 0b1);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
}

fn daa(_gb: &mut GameBoy) {
}

///CPL  complements the A register.  All 0's become 1's and all  1's
///0's. The C, Z P and S flags are all unaffected. The N and H flags
///are both set to 1.
fn cpl(gb: &mut GameBoy) {
    let a = gb.cpu.get_8(Register8::A);
    gb.cpu.set_8(Register8::A, !a);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, true);
}

/// SCF sets the carry flag to 1.
///The Z,  P and S flags are unaffected. The N and H flags are reset to 0.
fn scf(gb: &mut GameBoy) {
    gb.cpu.set_flag(Flag::C, true);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);

}


/// CCF complements the carry flag. If the flag was 1 it is now 0 and
/// vice  versa.  The Z P and S flags are unaffected.  The N flag  is
/// reset to 0. The H flag may be anything.
fn ccf(gb: &mut GameBoy) {
    let carry = gb.cpu.get_flag(Flag::C);
    gb.cpu.set_flag(Flag::C, !carry);
    gb.cpu.set_flag(Flag::N, false);
}

/* ------------------------

        ALU FUNCS

---------------------------*/

fn alu_add_a(gb: &mut GameBoy, operand: u8) {
    let res : u16 = (gb.cpu.get_8(Register8::A) + operand) as u16;
    let final_res : u8 = res as u8;

    gb.cpu.set_flag(Flag::Z, final_res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xF) + (operand & 0xF)) > 0xF);
    gb.cpu.set_flag(Flag::C, res > 0xFF);

    gb.cpu.set_8(Register8::A, res as u8)
}

fn alu_adc_a(gb: &mut GameBoy, operand: u8) {
    let mut carry : u8 = 0;
    if gb.cpu.get_flag(Flag::C) {
        carry = 1;
    }
    let res : u16 = (gb.cpu.get_8(Register8::A) + operand + carry) as u16;
    let final_res : u8 = res as u8;

    gb.cpu.set_flag(Flag::Z, final_res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xF) + (operand & 0xF) + carry) > 0xF);
    gb.cpu.set_flag(Flag::C, res > 0xFF);

    gb.cpu.set_8(Register8::A, final_res as u8)
}

fn alu_sub(gb: &mut GameBoy, operand: u8) {
    let res : u8 = gb.cpu.get_8(Register8::A) - operand;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xf) - (operand & 0xf)) < 0);
    gb.cpu.set_flag(Flag::C, gb.cpu.get_8(Register8::A) < operand);

    gb.cpu.set_8(Register8::A, res);
}

fn alu_sbc(gb: &mut GameBoy, operand: u8) {
    let mut carry : u8 = 0;
    if gb.cpu.get_flag(Flag::C) {
        carry = 1;
    }
    let full_res : u16 = (gb.cpu.get_8(Register8::A) - operand - carry) as u16;
    let res : u8 = full_res as u8;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xf) - (operand & 0xf) - carry) < 0);
    gb.cpu.set_flag(Flag::C, full_res < 0);

    gb.cpu.set_8(Register8::A, res);
}

fn alu_and(gb: &mut GameBoy, operand: u8) {
    let res: u8 = gb.cpu.get_8(Register8::A) & operand;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, true);
    gb.cpu.set_flag(Flag::C, false);

    gb.cpu.set_8(Register8::A, res);
}

fn alu_xor(gb: &mut GameBoy, operand: u8) {
    let res: u8 = gb.cpu.get_8(Register8::A) ^ operand;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
    gb.cpu.set_flag(Flag::C, false);

    gb.cpu.set_8(Register8::A, res);
}

fn alu_or(gb: &mut GameBoy, operand: u8) {
    let res: u8 = gb.cpu.get_8(Register8::A) | operand;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, false);
    gb.cpu.set_flag(Flag::C, false);

    gb.cpu.set_8(Register8::A, res);
}

fn alu_cp(gb: &mut GameBoy, operand: u8) {
    let res: u8 = gb.cpu.get_8(Register8::A) - operand;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xF) - (res & 0xF)) > 0);
    gb.cpu.set_flag(Flag::C, gb.cpu.get_8(Register8::A) < operand);
}

fn op_ret(gb: &mut GameBoy) {
    let sp: u16 = gb.cpu.get_16(Register16::SP);
    let value: u16 = reverse_endian!(gb.mem.read_word(sp));
    gb.cpu.set_16(Register16::SP, sp + 2);
    gb.cpu.set_16(Register16::PC, value);
}

#[test]
fn ret() {
    use cpu::Cpu;
    use memory::Memory;
    use cartridge::Cartridge;

    let rom = Cartridge::empty(0xFFFF).unwrap();
    let cpu = Cpu::new();
    let mem = Memory::new();
    let mut gb = GameBoy::new(cpu, rom, mem);

    gb.cpu.set_16(Register16::PC, 0x3535);
    gb.cpu.set_16(Register16::SP, 0x2000);
    gb.mem.write_byte(0x2000, 0xB5);
    gb.mem.write_byte(0x2001, 0x18);

    op_ret(&mut gb);

    assert_eq!(gb.cpu.get_16(Register16::SP), 0x2002);
    assert_eq!(gb.cpu.get_16(Register16::PC), 0x18B5);
}

#[test]
fn ld_nn_pp() {
    use cpu::Cpu;
    use memory::Memory;
    use cartridge::Cartridge;

    let rom = Cartridge::empty(0xFFFF).unwrap();
    let cpu = Cpu::new();
    let mem = Memory::new();
    let mut gb = GameBoy::new(cpu, rom, mem);

    gb.cpu.set_16(Register16::SP, 0x4644);
    let value = gb.cpu.get_16(Register16::SP);

    op_ld_mem16(&mut gb, 0x1000, value);

    assert_eq!(gb.mem.read_byte(0x1000), 0x44);
    assert_eq!(gb.mem.read_byte(0x1001), 0x46);
}

