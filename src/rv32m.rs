use crate::{sext, State};

// multiply extension instructions

pub fn mul(s: &mut State, rs1: usize, rs2: usize, rd: usize) {
    s.regs[rd] = s.regs[rs1].wrapping_mul(s.regs[rs2]);
}

pub fn mulh(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn mulhsu(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn mulhu(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn div(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn divu(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn rem(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}

pub fn remu(_s: &mut State, _rs1: usize, _rs2: usize, _rd: usize) {}
