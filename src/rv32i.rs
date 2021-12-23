use crate::{State, sext};

// core rv32i instructions

pub fn add(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1].wrapping_add(s.regs[rs2])
}

pub fn sub(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1].wrapping_sub(s.regs[rs2])
}

pub fn sll(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1] << (s.regs[rs2] & 0b11111)
}

pub fn slt(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    if (s.regs[rs1] as i32) < s.regs[rs2] as i32 {
        s.regs[rd] = 0
    } else {
        s.regs[rd] = 1
    }
}

pub fn sltu(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    if s.regs[rs1] < s.regs[rs2] {
        s.regs[rd] = 0
    } else {
        s.regs[rd] = 1
    }
}

pub fn xor(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1] ^ s.regs[rs2]
}

pub fn srl(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1] >> (s.regs[rs2] & 0b11111)
}

pub fn sra(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = (s.regs[rs1] as i32 >> (s.regs[rs2] & 0b11111) as i32) as u32
}

pub fn or(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1] | s.regs[rs2]
}

pub fn and(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1] & s.regs[rs2]
}

pub fn addi(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1].wrapping_add(ext_imm)
}

pub fn slti(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    if (s.regs[rs1] as i32) < ext_imm as i32 {
        s.regs[rd] = 0
    } else {
        s.regs[rd] = 1
    }
}

pub fn sltiu(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    if s.regs[rs1] < ext_imm {
        s.regs[rd] = 0
    } else {
        s.regs[rd] = 1
    }
}

pub fn xori(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1] ^ ext_imm
}

pub fn ori(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1] | ext_imm
}

pub fn andi(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1] & ext_imm
}

pub fn slli(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1] << ext_imm
}

pub fn srli(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.regs[rs1] >> ext_imm
}

pub fn srai(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = (s.regs[rs1] as i32 >> ext_imm as i32) as u32
}

// implements lb(u), lh(u), lw
pub fn lx(s: &mut State, rs1: usize, ext_imm: u32, rd: usize, dmem: &[u8], len: usize, signed: bool) {
    assert!(len == 8 || len == 16 || len == 32);

    let addr = (s.regs[rs1] + ext_imm) as usize;

    if addr >= dmem.len() {
        panic!("illegal memory read at byte address {:x}", addr)
    }

    if addr % (s.ialign / 8) as usize != 0 {
        panic!("misaligned memory access at byte address {:x}", addr)
    }

    let mut val: u32 = 0;
    for i in (0..len).step_by(8) {
        val |= (dmem[addr + i/8] as u32) << i;
    }

    // we can't move this up because exceptions still need to happen when rd is x0
    if rd == 0 {
        return
    }

    // lb and lh need sign-extension
    s.regs[rd] = match signed {
        true => sext(val, len),
        false => val,
    };
}

// implements sb, sh, sw
pub fn sx(s: &mut State, rs1: usize, ext_imm: u32, rs2: usize, dmem: &mut [u8], len: usize) {
    assert!(len == 8 || len == 16 || len == 32);
    
    let addr = (s.regs[rs1] + ext_imm) as usize;
    let word_addr = addr / 4;

    if word_addr >= dmem.len() {
        panic!("illegal memory write at byte address {:x}", addr)
    }

    if addr % (s.ialign / 8) as usize != 0 {
        panic!("misaligned memory access at byte address {:x}", addr)
    }

    let val = s.regs[rs2];

    for i in (0..len).step_by(8) {
        dmem[addr + i/8] = (val >> i) as u8;
    }
}

pub fn jalr(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    s.regs[rd] = s.pc;
    s.pc += (s.regs[rs1] + ext_imm) & 0xfffffffe;
}

pub fn beq(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] == s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn bne(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] != s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn blt(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] < s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn bge(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] >= s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn bltu(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] < s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn bgeu(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] < s.regs[rs2] {
        s.pc = ((s.pc - 4) as i32 + ext_imm as i32) as u32
    }
}

pub fn lui(s: &mut State, rd: usize, imm: u32) {
    s.regs[rd] = imm << 12
}

pub fn auipc(s: &mut State, rd: usize, imm: u32) {
    s.regs[rd] = (s.pc - 4) + (imm << 12)
}

pub fn jal(s: &mut State, rd: usize, imm: u32) {
    s.regs[rd] = s.pc;
    s.pc += sext(imm, 20) + s.pc - 4;
}
