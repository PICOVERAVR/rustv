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
        s.regs[rd] = 1
    } else {
        s.regs[rd] = 0
    }
}

pub fn sltu(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    if s.regs[rs1] < s.regs[rs2] {
        s.regs[rd] = 1
    } else {
        s.regs[rd] = 0
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
        s.regs[rd] = 1
    } else {
        s.regs[rd] = 0
    }
}

pub fn sltiu(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    if s.regs[rs1] < ext_imm {
        s.regs[rd] = 1
    } else {
        s.regs[rd] = 0
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
    // need to mask off high bits of ext_imm since only the lower 5 
    // are used and the 10th bit is set to 1
    s.regs[rd] = (s.regs[rs1] as i32 >> (ext_imm & 0b11111) as i32) as u32
}

// implements lb(u), lh(u), lw
pub fn lx(s: &mut State, rs1: usize, ext_imm: u32, rd: usize, dmem: &[u8], len: usize, signed: bool) {
    assert!(len == 8 || len == 16 || len == 32);

    // wrapping add to handle negative immediates
    let addr = (s.regs[rs1].wrapping_add(ext_imm)) as usize;

    if addr >= dmem.len() {
        panic!("illegal memory read at byte address {:x}", addr)
    }

    match addr % 4 {
        1 | 2 | 3 if len == 32 => panic!("misaligned word load at byte address {:x}", addr),
        1 | 3 if len == 16 => panic!("misaligned halfword load at byte address {:x}", addr),
        _ => (),
    }

    let mut val: u32 = 0;
    for i in (0..len).step_by(8) {
        val |= (dmem[addr + i/8] as u32) << i;
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
    
    // wrapping add to handle negative immediates
    let addr = (s.regs[rs1].wrapping_add(ext_imm)) as usize;
    let word_addr = addr / 4;

    if word_addr >= dmem.len() {
        panic!("illegal memory write at byte address {:x}", addr)
    }

    match addr % 4 {
        1 | 2 | 3 if len == 32 => panic!("misaligned word store at byte address {:x}", addr),
        1 | 3 if len == 16 => panic!("misaligned halfword store at byte address {:x}", addr),
        _ => (),
    }

    let val = s.regs[rs2];

    for i in (0..len).step_by(8) {
        dmem[addr + i/8] = (val >> i) as u8;
    }
}

pub fn jalr(s: &mut State, rs1: usize, ext_imm: u32, rd: usize) {
    let add = (s.regs[rs1] as i32 + ext_imm as i32) & -2; // -2 unsigned is 0b11..110
    let dest = ((s.pc - 4) as i32 + add) as u32;

    if dest % 4 != 0 {
        panic!("misaligned destination byte address {:x}", dest);
    }

    s.regs[rd] = s.pc;
    s.pc = dest;
}

pub fn beq(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] == s.regs[rs2] {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn bne(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] != s.regs[rs2] {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn blt(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if (s.regs[rs1] as i32) < s.regs[rs2] as i32 {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn bge(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if (s.regs[rs1] as i32) >= s.regs[rs2] as i32 {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn bltu(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] < s.regs[rs2] {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn bgeu(s: &mut State, rs1: usize, rs2: usize, ext_imm: u32) {
    if s.regs[rs1] >= s.regs[rs2] {
        let dest = ((s.pc - 4) as i32 + ext_imm as i32) as u32;
        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        s.pc = dest;
    }
}

pub fn lui(s: &mut State, rd: usize, upper_imm: u32) {
    s.regs[rd] = upper_imm
}

pub fn auipc(s: &mut State, rd: usize, upper_imm: u32) {
    s.regs[rd] = (s.pc - 4) + upper_imm
}

pub fn jal(s: &mut State, rd: usize, imm: u32) {
    let dest = ((s.pc - 4) as i32 + sext(imm, 20) as i32) as u32;

    if dest % 4 != 0 {
        panic!("misaligned destination byte address {:x}", dest);
    }

    s.regs[rd] = s.pc;
    s.pc = dest;
}

pub enum Action {
    Terminate, // terminate execution
    Resume, // resume execution
}

pub fn ecall(s: &mut State) -> Action {
    match s.regs[8] {
        0 => assert_eq!(s.regs[9], s.regs[18], "test {}, l: 0x{:08x}, r: 0x{:08x}", s.regs[5], s.regs[9], s.regs[18]),
        err => panic!("unknown ecall parameter 0x{:x} in x8, pc 0x{:x}", err, s.pc - 4)
    }

    Action::Resume
}

pub fn ebreak(_s: &mut State) -> Action {
    println!("terminating execution ({} instructions executed)", _s.ret);
    Action::Terminate
}
