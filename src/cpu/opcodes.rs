use std::borrow::BorrowMut;
use std::num::Wrapping;
use std::process::exit;
use bitlab::SingleBits;
use gameboy::GameBoy;
use cpu::{Flag, Register16, Register8};
use cpu::opcode::Opcode;
use ::{high_byte, reverse_endian, low_byte, join_bytes};

/// Iterate the ROM
pub fn decode(gb: &mut GameBoy) {
    let pc = gb.cpu.get_16(Register16::PC);
    let mut op = Opcode::from(gb.read_byte(pc));
    op.fetch_param(&gb);


    // Debug
    // println!("op = {:#02x}, pc = {:#04x}", op.opcode, gb.cpu.get_16(Register16::PC));
    // println!("{:?}", gb.cpu);

    // Call a function who get the current opcode and a mutable gameboy instance
    // should incr the PC and returns arguments*

    if gb.cpu.get_16(Register16::PC) == 0xA3 {
        op_nop();
    }

    let mut add_op_length: bool = true;

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
            op_ld_mem16(gb, op.param, gb.cpu.get_16(Register16::SP));
        },
       /* (0, 2, 0, _, _) => {
            // STOP
            gb.stopped = true;
            println!("STOP");
            exit(1);
        },*/
        (0, 3, 0, _, _) => {
            // JR d
            let pc_val = gb.cpu.get_16(Register16::PC);
            gb.cpu.set_16(Register16::PC, pc_val.wrapping_add(op.param as i8 as u16));
        },
        (0, 4..=7, 0, _, _) => {
            // JR cc[y-4], d
            if gb.get_table_cc(op.y - 4) {
                let mut pc_val = gb.cpu.get_16(Register16::PC);
                pc_val = pc_val.wrapping_add(op.param as i8 as u16);
                gb.cpu.set_16(Register16::PC, pc_val);
                op.ticks = 3;
            } else {
                op.ticks = 2;
            }
        },
        // Z = 1
        (0, 4, 2, 2, 0) => {
            // LD (HL+), A
            gb.write_byte(
                gb.cpu.get_16(Register16::HL),
                gb.cpu.get_8(Register8::A)
            );
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) + 1);
        }
        (0, _, 1, _, 0) => {
            // LD rp[p], nn
            op_ld_reg16(gb, op.p, op.param);

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
            // LD (nn), HL
            gb.write_word(op.param, gb.cpu.get_16(Register16::HL))
        },
        (0, _, 2, 3, 0)  => {
            // LD (HL-), A
            gb.write_byte(gb.cpu.get_16(Register16::HL), gb.cpu.get_8(Register8::A));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) - 1);
        },
        // Q = 1
        (0, _, 2, 0, 1)  => {
            // LD A, (BC)
            gb.cpu.set_8(Register8::A, gb.read_byte(gb.cpu.get_16(Register16::BC)));
        },
        (0, _, 2, 1, 1)  => {
            // LD A, (DE)
            gb.cpu.set_8(Register8::A, gb.read_byte(gb.cpu.get_16(Register16::DE)));
        },
        (0, _, 2, 2, 1)  => {
            // LD HL, (nn)
            gb.cpu.set_16(Register16::HL, gb.read_word(op.param));
        },
        (0, _, 2, 3, 1) => {
            // LD A, (nn)
            gb.cpu.set_8(Register8::A, gb.read_byte(op.param));

        },
        // Z = 3
        (0, _, 3, _, 0) => {
            // INC rp[p]
            // op_inc16(gb, op.p)
            gb.set_table_rp(op.p, (Wrapping(gb.get_table_rp(op.p)) + Wrapping(1)).0);
        },
        (0, _, 3, _, 1) => {
            // DEC rp[p]
            let mut value = gb.get_table_rp(op.p);
            gb.cpu.set_flag(Flag::H, value & 0x0F == 0);
            value -= 1;
            gb.cpu.set_flag(Flag::Z, value == 0);
            gb.cpu.set_flag(Flag::N, true);
            gb.set_table_rp(op.p, value);
        },
        // Z = 4
        (0, _, 4, _, _) => {
            // INC r[y]
            op_inc8(gb, op.y)
        },
        (0, _, 5, _, _) => {
            // DEC r[y]
            let mut value = gb.get_table_r(op.y);
            gb.cpu.set_flag(Flag::H, value & 0x0F == 0);
            value = (Wrapping(value) - Wrapping(1)).0;
            gb.cpu.set_flag(Flag::Z, value == 0);
            gb.cpu.set_flag(Flag::N, true);

            gb.set_table_r(op.y, value);
        },
        (0, _, 6, _, _) => {
            // LD r[y], n
            op_ld_8(gb, op.y, low_byte!(op.param))
        },

        (0, 0, 7, _, _) => rlca(gb), // RLCA
        (0, 1, 7, _, _) => rrca(gb), // RRCA
        (0, 2, 7, _, _) => rla(gb), // RLA
        (0, 3, 7, _, _) => rra(gb), // RRA
        (0, 4, 7, _, _) => daa(gb), // DAA //TODO: Impl DAA
        (0, 5, 7, _, _) => cpl(gb), // CPL
        (0, 6, 7, _, _) => scf(gb), // SCF
        (0, 7, 7, _, _) => ccf(gb), // CCF
        // X = 1
        (1, 6, 6, _, _) => exit(2), // HALT
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
                add_op_length = false;
                op.ticks = 20;
            } else {
                op.ticks = 8;
            }
        }, // RET cc[y]
        (3, 4, 0, 2, 0) => {
            //LDH (n), a
            gb.write_byte(
                0xff00 + op.param,
                gb.cpu.get_8(Register8::A)
            )
        }
        (3, 5, 0, _, _) => {
            //LDH (C), a
            gb.write_byte(
                0xff00 + gb.cpu.get_8(Register8::C) as u16,
                gb.cpu.get_8(Register8::A)
            )
        }
        (3, 6, 0, _, _) => {
            //LD A, 0Xff00 + n
            gb.cpu.set_8(
                Register8::A,
                gb.read_byte(
                    0xff00 + op.param
                )
            );
        }
        (3, 7, 0, _, _) => {
            //LD A, 0Xff00 + C
            gb.cpu.set_8(
                Register8::A,
                gb.read_byte(
                    0xff00 + gb.cpu.get_8(Register8::C) as u16
                )
            );
        }
        (3, _, 1, _, 0) => {
            //POP rp2[p]
            let poped = gb.read_word(gb.cpu.get_16(Register16::SP));
            gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::SP) + 2);
            gb.set_table_rp2(op.p, poped);
        },
        (3, _, 1, 0, 1) => {
            // RET
            op_ret(gb);
            add_op_length = false;
        },
        /*(3, _, 1, 1, 1) => {
            //EXX
            op_nop()
        },*/
        (3, _, 1, 2, 1) => {
            //JP HL
            gb.cpu.set_16(Register16::PC, gb.cpu.get_16(Register16::HL));
            add_op_length = false;
        },
        (3, _, 1, 3, 1) => {
            //LD SP,HL
            gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::HL));
        },
        (3, 4, 2, 2, 0) => {
            // LD (FF00h+C), A
            gb.write_byte(
                0xFF00 + gb.cpu.get_8(Register8::C) as u16,
                gb.cpu.get_8(Register8::A)
            );
        },
        (3, 0..=3, 2, _, _) => {
            // JP cc[y], nn
            if gb.get_table_cc(op.y) {
                gb.cpu.set_16(Register16::PC, op.param);
                add_op_length = false;
                op.ticks = 16;
            } else {
                op.ticks = 12;
            }
        },
        (3, 5, 2, 2, 1) => {
            //LD (nn), a
            gb.write_byte(op.param, gb.cpu.get_8(Register8::A));
        }
        (3, 0, 3, _, _) => {
            //JP nn
            gb.cpu.set_16(Register16::PC, op.param);
            add_op_length = false;
        },
        (3, 1, 3, _, _) => {
            // CB
            let x = (op.param & 0b1100_0000) as u8 >> 6;
            let y = (op.param & 0b0011_1000) as u8 >> 3;
            let z = (op.param & 0b0000_0111) as u8;
            match x {
                0 => {
                    let res = gb.exec_table_rot(y, gb.get_table_r(z));
                    gb.set_table_r(z, res);
                },
                1 => op_bit(gb, y, z),
                2 => op_res(gb, y, z),
                3 => op_set(gb, y, z),
                _ => op_nop(),
            }
        },
        (3, 6, 3, _, _) => {
            //DI
            gb.cpu.set_iter_master(false);
        },
        (3, 7, 3, _, _) => {
            //EI
            gb.cpu.set_iter_master(true);
        },
        (3, 0..=3, 4, _, _) => {
            if gb.get_table_cc(op.y) {
                gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::PC) + op.length as u16);
                gb.cpu.set_16(Register16::PC, op.param);
                add_op_length = false;
                op.ticks = 24;
            } else {
                op.ticks = 12;
            }
        }, // CALL cc[y], nn
        (3, _, 5, _, 0) => {
            //PUSH rp2[p]
            gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::SP) - 2);
            gb.write_word(gb.cpu.get_16(Register16::SP), gb.get_table_rp2(op.p));
        },
        (3, _, 5, 0, 1) => {
            // CALL nn
            gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::SP) - 2);
            gb.write_word(gb.cpu.get_16(Register16::SP), gb.cpu.get_16(Register16::PC) + op.length as u16);
            gb.cpu.set_16(Register16::PC, op.param);
            add_op_length = false;
        },
        /*(3, _, 5, 1, 1) => {
            // DD
        },
        (3, _, 5, 2, 1) => {
            // ED
        },
        (3, _, 5, 3, 1) => {
            // FD
        },*/
        (3, _, 6, _, _) => {
            // alu[y] n
            exec_alu(gb, op.y, op.param as u8);
        },
        (3, _, 7, _, _) => {
            // RST y*8
            gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::PC) + op.length as u16);
            gb.cpu.set_16(Register16::PC, op.y as u16 * 8);
            add_op_length = false;
        },
        _ => op_not_implemented(op.opcode),
    }

    if op.ticks == 0 {
        println!("OP HAS NO TICKS");
        exit(100);
    }


    if add_op_length {
        gb.cpu.inc_pc(op.length);
    }
    println!("OP {:#02x} ADDED ", op.opcode);
    gb.cpu.inc_ticks(op.ticks);
}

fn op_not_implemented(op: u8) {
    println!("OP {:#02x} NOT IMPLEMENTED", op);
    exit(1);
}

fn op_nop(){
    println!("nop")
}

fn op_res(gb: &mut GameBoy, y: u8, z: u8) {
    let mut r = gb.get_table_r(z);
    gb.set_table_r(z, r.clear_bit(y as u32).unwrap());
}

fn op_set(gb: &mut GameBoy, y: u8, z: u8) {
    let mut r = gb.get_table_r(z);
    gb.set_table_r(z, r.set_bit(y as u32).unwrap());
}

fn op_bit(gb: &mut GameBoy, y: u8, z: u8) {
    let value = gb.get_table_r(z);
    if value.get_bit(7 - y as u32).unwrap() {
        gb.cpu.set_flag(Flag::Z, false);
    } else {
        gb.cpu.set_flag(Flag::Z, true);
    }
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, true);
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
    gb.write_byte(address, operand);
}

fn op_ld_mem16(gb: &mut GameBoy, address: u16, operand: u16) {
    gb.write_word(address, operand);
}

fn op_ld_8(gb: &mut GameBoy, index: u8, operand: u8) {
    gb.set_table_r(index, operand);
}

fn op_inc8(gb: &mut GameBoy, index: u8) {
    let res = add8(gb, gb.get_table_r(index), 1);
    gb.set_table_r(index, res);
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
        6 => exec_alu(gb, y, gb.read_byte(gb.cpu.get_16(Register16::HL))),
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

fn daa(gb: &mut GameBoy) {
    let mut s: u16 = gb.cpu.get_8(Register8::A) as u16;
    if gb.cpu.get_flag(Flag::N) {
        if gb.cpu.get_flag(Flag::H){
            s = (s - 0x06) & 0xFF;
        }
        if gb.cpu.get_flag(Flag::C) {
            s -= 0x60;
        }
    } else {
        if gb.cpu.get_flag(Flag::H) || (s & 0xF) > 9{
            s += 0x06
        }
        if gb.cpu.get_flag(Flag::C) || s > 0x9F {
            s -= 0x60;
        }
    }
    gb.cpu.set_8(Register8::A, s as u8);
    gb.cpu.set_flag(Flag::H, false);
    gb.cpu.set_flag(Flag::Z, s as u8 == 0);
    if s >= 0x100 {
        gb.cpu.set_flag(Flag::C, true);
    }
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
    let res : u16 = (gb.cpu.get_8(Register8::A) as u16 + operand as u16) as u16;
    let final_res : u8 = res as u8;

    gb.cpu.set_flag(Flag::Z, final_res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xF) + (operand & 0xF)) > 0xF);
    gb.cpu.set_flag(Flag::C, res > 0xFF);

    gb.cpu.set_8(Register8::A, res as u8)
}

fn alu_adc_a(gb: &mut GameBoy, operand: u8) {
    let mut carry : u16 = 0;
    if gb.cpu.get_flag(Flag::C) {
        carry = 1;
    }
    let res : u16 = gb.cpu.get_8(Register8::A) as u16 + operand as u16 + carry;
    let final_res : u8 = res as u8;

    gb.cpu.set_flag(Flag::Z, final_res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(Register8::A) & 0xF) as u16 + (operand as u16 & 0xF) + carry) > 0xF);
    gb.cpu.set_flag(Flag::C, res > 0xFF);

    gb.cpu.set_8(Register8::A, final_res as u8)
}

#[allow(arithmetic_overflow)]
fn alu_sub(gb: &mut GameBoy, operand: u8) {
    let res : u8 = (Wrapping(gb.cpu.get_8(Register8::A)) - Wrapping(operand)).0;

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, (Wrapping(gb.cpu.get_8(Register8::A) & 0xf) - Wrapping(operand & 0xf)).0 < 0);
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
    let res: u8 = (Wrapping(gb.cpu.get_8(Register8::A)) - Wrapping(operand)).0;

    gb.cpu.reset_flags();

    gb.cpu.set_flag(Flag::Z, res == 0);
    gb.cpu.set_flag(Flag::N, true);
    gb.cpu.set_flag(Flag::H, gb.cpu.get_8(Register8::A) & 0x0F < (res & 0x0F));
    gb.cpu.set_flag(Flag::C, gb.cpu.get_8(Register8::A) < operand);
}

fn op_ret(gb: &mut GameBoy) {
    gb.cpu.set_16(
        Register16::PC,
        gb.read_word(gb.cpu.get_16(Register16::SP))
    );
    gb.cpu.set_16(Register16::SP, gb.cpu.get_16(Register16::SP) + 2);
}

/*#[test]
fn test_ld() {
    use cpu::Cpu;
    use memory::Memory;
    use cartridge::Cartridge;

    let rom = Cartridge::empty(0xFFFF).unwrap();
    let cpu = Cpu::new();
    let mem = Memory::new(rom);
    let mut gb = GameBoy::new(cpu, mem);

    gb.cpu.set_16(Register16::SP, 0x4644);
    let value = gb.cpu.get_16(Register16::SP);

    op_ld_mem16(&mut gb, 0x1000, value);

    assert_eq!(gb.read_byte(0x1000), 0x44);
    assert_eq!(gb.read_byte(0x1001), 0x46);

    op_ld_reg16(&mut gb, 0, 0xCACA);
    assert_eq!(gb.cpu.get_16(Register16::BC), 0xCACA);
}
*/





