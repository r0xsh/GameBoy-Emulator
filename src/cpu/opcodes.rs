use ::GameBoy;
use cpu::{Flag, Register16, Register8};
use cpu::opcode::Opcode;

/// Iterate the ROM
pub fn decode(gb: &mut GameBoy) {
    let mut op = Opcode::from(gb.cartridge.read_byte(gb.cpu.get_16(Register16::PC)));
    op.fetch_param(&gb);


    println!("op = {:?}, pc = {}", op, gb.cpu.get_16(Register16::PC));

    gb.cpu.inc_pc(op.length);

    // Call a function who get the current opcode and a mutable gameboy instance
    // should incr the PC and returns arguments



    // X, Y, Z, P, Q
    match op.flag_slice() {
        // X = 0
        // Z = 0
        (0, 0, 0, _, _) => {
            op_nop()
        }, // NOOP
        (0, 1, 0, _, _) => {
            op_ld_mem16(gb, op.param.unwrap(), gb.cpu.get_16(Register16::SP))
        }, // LD (nn), SP
        (0, 2, 0, _, _) => {
            op_nop()
        }, // STOP
        (0, 3, 0, _, _) => {
            let pc_val = gb.cpu.get_16(Register16::PC);
            gb.cpu.set_16(Register16::PC, pc_val + 1 + gb.mem.read_byte(pc_val) as u16);
        }, // JR d
        (0, 4..=7, 0, _, _) => {
            if gb.cpu.get_flag(cc!(op.y - 4)) {
                let pc_val = gb.cpu.get_16(Register16::PC);
                gb.cpu.set_16(Register16::PC, pc_val + 1 + gb.mem.read_byte(pc_val) as u16);
            }
        }, // JR cc[y-4], d
        // Z = 1
        (0, _, 1, _, 0) => {
            op_ld_reg16(gb, rp!(op.q), op.param.unwrap())
        }, // LD rp[p], nn
        (0, _, 1, _, 1) => {
            op_add16(gb, Register16::HL, gb.cpu.get_16(rp!(op.p)))
        }, // ADD HL, rp[p]

        // Z = 2
        // Q = 0
        (0, _, 2, 0, 0) => {
            op_ld_mem8(gb, gb.cpu.get_16(Register16::BC), gb.cpu.get_8(Register8::A))
        }, // LD (BC), A
        (0, _, 2, 1, 0)  => {
            op_ld_mem8(gb, gb.cpu.get_16(Register16::DE), gb.cpu.get_8(Register8::A))
        }, // LD (DE), A
        (0, _, 2, 2, 0)  => {
            op_ld_mem8(gb, gb.cpu.get_16(Register16::HL), gb.cpu.get_8(Register8::A));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) + 1);
        }, // LD (HL+), A
        (0, _, 2, 3, 0)  => {
            op_ld_mem8(gb, gb.cpu.get_16(Register16::HL), gb.cpu.get_8(Register8::A));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) - 1);
        }, // LD (HL-), A
        // Q = 1
        (0, _, 2, 0, 1)  => {
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::BC)))
        }, // LD A, (BC)
        (0, _, 2, 1, 1)  => {
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::DE)))
        }, // LD A, (DE)
        (0, _, 2, 2, 1)  => {
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::HL)));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) + 1);
        }, // LD A, (HL+)
        (0, _, 2, 3, 1) => {
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::HL)));
            gb.cpu.set_16(Register16::HL, gb.cpu.get_16(Register16::HL) - 1);
        }, // LD A, (HL-)
        // Z = 3
        (0, _, 3, _, 0) => {
            op_inc16(gb, rp!(op.p))
        }, // INC rp[p]
        (0, _, 3, _, 1) => {
            op_dec16(gb, rp!(op.p))
        }, // DEC rp[p]
        // Z = 4
        (0, _, 4, _, _) => {
            match r!(op.y) {
                (Some(r8), None) => {
                    op_inc8(gb, r8)
                },
                (None, Some(r16)) => {
                    op_inc_mem(gb, gb.cpu.get_16(r16))
                },
                _ => unreachable!()
            }
            // op_inc8(gb, cpu::op_decode::TABLE_R) // TODO: Get R value from CPU
        }, // INC r[y]
        (0, _, 5, _, _) => {
            match r!(op.y) {
                (Some(r8), None) => {
                    op_dec8(gb, r8)
                },
                (None, Some(r16)) => {
                    op_dec_mem(gb, gb.cpu.get_16(r16))
                },
                _ => unreachable!()
            }
            // op_inc8(gb, cpu::op_decode::TABLE_R) // TODO: Get R value from CPU
        }, // DEC r[y]
        (0, _, 6, _, _) => op_nop(), // LD r[y], n

        (0, 0, 7, _, _) => op_nop(), // RLCA
        (0, 1, 7, _, _) => op_nop(), // RRCA
        (0, 2, 7, _, _) => op_nop(), // RLA
        (0, 3, 7, _, _) => op_nop(), // RRA
        (0, 4, 7, _, _) => op_nop(), // DAA
        (0, 5, 7, _, _) => op_nop(), // CPL
        (0, 6, 7, _, _) => op_nop(), // SCF
        (0, 7, 7, _, _) => op_nop(), // CCF
        // X = 1
        (1, 6, 6, _, _) => op_nop(), // HALT
        (1, _, _, _, _) => op_nop(), // CCF
        // X = 2
        (2, _, _, _, _) => {
            call_alu_table_r(gb, op.z, op.y)
        }, // alu[y] r[z]
        // X = 3
        (3, 0..=3, 0, _, _) => {
            op_nop()
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
            op_nop()
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
            op_ld_mem8(gb, gb.mem.read_word(gb.cpu.get_16(Register16::PC) + 1), gb.cpu.get_8(Register8::A))
        }, // LD (nn), A
        (3, 6, 2, _, _) => {
            op_nop()
        }, // LD A, (0xFF00+C)
        (3, 7, 2, _, _) => {
            op_ld_reg8(gb, Register8::A, gb.mem.read_byte(gb.cpu.get_16(Register16::PC) + 1))
        }, // LD A, (nn)

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
            exec_alu(gb, op.y, gb.cartridge.read_byte(gb.cpu.get_16(Register16::PC) + 1))
        }, // alu[y] n

        (3, _, 7, _, _) => {
            op_nop()
        }, // RST y*8
        _ => op_nop(),
    }
}

fn op_nop(){
    ()
}

fn inc(gb: &mut GameBoy, val: u8) -> u8 {
    gb.cpu.set_flag(Flag::H, (val & 0x0F) == 0x0F);
    let val = val + 1;
    gb.cpu.set_flag(Flag::Z, val == 0);
    gb.cpu.set_flag(Flag::N, false);
    val
}

fn dec(gb: &mut GameBoy, val: u8) -> u8 {
    gb.cpu.set_flag(Flag::H, val & 0x0F == 0);
    let val = val - 1;
    gb.cpu.set_flag(Flag::Z, val == 0);
    gb.cpu.set_flag(Flag::N, true);
    val
}

fn op_ld_reg8(gb: &mut GameBoy, reg: Register8, operand: u8) {
    gb.cpu.set_8(reg, operand);
}

fn op_ld_reg16(gb: &mut GameBoy, reg: Register16, operand: u16) {
    gb.cpu.set_16(reg, operand);
}

fn op_ld_mem8(gb: &mut GameBoy, address: u16, operand: u8) {
    gb.mem.write_byte(address, operand);
}

fn op_ld_mem16(gb: &mut GameBoy, address: u16, operand: u16) {
    gb.mem.write_word(address, operand);
}

fn op_inc8(gb: &mut GameBoy, reg: Register8) {
    let res = inc(gb, gb.cpu.get_8(reg));
    gb.cpu.set_8(reg, res);
}

fn op_inc16(gb: &mut GameBoy, reg: Register16) {
    let res = gb.cpu.get_16(reg) + 1;
    gb.cpu.set_16(reg, res);
}

fn op_inc_mem(gb: &mut GameBoy, address: u16) {
    let res = inc(gb, gb.mem.read_byte(address));
    gb.mem.write_byte(address, res);
}

fn op_dec16(gb: &mut GameBoy, reg: Register16) {
    let res = gb.cpu.get_16(reg) - 1;

    gb.cpu.set_16(reg, res);
}

fn op_dec8(gb: &mut GameBoy, reg: Register8) {
    let res = dec(gb, gb.cpu.get_8(reg));
    gb.cpu.set_8(reg, res);
}



fn op_dec_mem(gb: &mut GameBoy, address: u16) {
    let res = dec(gb, gb.mem.read_byte(address));
    gb.mem.write_byte(address, res);
}

fn op_add8(gb: &mut GameBoy, reg: Register8, operand: u8) {
    let res: u16 = (gb.cpu.get_8(reg) + operand) as u16;
    let final_res: u8 = res as u8;

    gb.cpu.set_flag(Flag::Z, final_res == 0);
    gb.cpu.set_flag(Flag::N, false);
    gb.cpu.set_flag(Flag::H, ((gb.cpu.get_8(reg) & 0xF) + (operand & 0xF)) > 0xF);
    gb.cpu.set_flag(Flag::C, res > 0xFF);

    gb.cpu.set_8(reg, res as u8)
}

fn op_add16(gb: &mut GameBoy, reg: Register16, operand: u16) {
        let res: u32 = (gb.cpu.get_16(reg) + operand) as u32;

        gb.cpu.set_flag(Flag::C, res > 0xFFFF);
        gb.cpu.set_flag(Flag::H, ((gb.cpu.get_16(reg) & 0x0F) + (operand & 0x0F)) > 0x0F);
        gb.cpu.set_flag(Flag::N, false);

        gb.cpu.set_16(reg, res as u16);
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
        0 => op_add8(gb, Register8::A, operand),
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
