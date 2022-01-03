use std::time::Instant;

mod rv32i;
use rv32i::*;

mod zifencei;
use zifencei::*;

mod config;
pub use config::*;

// implements rv32i
#[derive(Debug)]
pub struct State {
    regs: [u32; 32],
    pc: u32,
    //xlen: u8, // register width

    ret: u64, // number of retired instructions
}

impl State {
    pub fn new(pc: usize) -> State {
        State {
            regs: [0; 32],
            pc: pc as u32,
            ret: 0,
        }
    }

    pub fn gprs(&self) -> [u32; 32] {
        self.regs
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }
}

// stores all decoded instruction data with immediates unpacked
#[derive(Debug)]
enum Itype {
    // RV32I
    R {_op: u8, rd: usize, funct3: u8, rs1: usize, rs2: usize, funct7: u8},
    I {_op: u8, rd: usize, funct3: u8, rs1: usize, imm: u32},
    S {_op: u8, funct3: u8, rs1: usize, rs2: usize, imm: u32},
    B {_op: u8, funct3: u8, rs1: usize, rs2: usize, imm: u32},
    U {_op: u8, rd: usize, imm: u32},
    J {_op: u8, rd: usize, imm: u32},
    Ecall {imm: u16},
    Fence {_rd: usize, _rs1: usize, _succ: u8, _pred: u8, _fm: u8},
}

fn crack(instr: u32) -> Itype {
    let _op = (instr & 0b1111111) as u8;
    let rd = ((instr >> 7) & 0b11111) as usize;
    let funct3 = ((instr >> 12) & 0b111) as u8;
    let funct7 = (instr >> 25) as u8;
    let rs1 = ((instr >> 15) & 0b11111) as usize;
    let rs2 = ((instr >> 20) & 0b11111) as usize;

    let imm_j = 
        (((instr >> 21) & 0b1111111111) << 1) | 
        (((instr >> 20) & 0b1) << 10) | 
        (((instr >> 12) & 0b11111111) << 12) | 
        ((instr >> 31) << 20);
    
    let imm_b = 
        (((instr >> 8) & 0b1111) << 1) |
        (((instr >> 25) & 0b111111) << 5) |
        (((instr >> 7) & 0b1) << 11) | 
        (((instr >> 31) & 0b1) << 12);
            
    let imm_s = 
        ((instr >> 7) & 0b11111) |
        (((instr >> 25) & 0b1111111) << 5);

    match _op {
        // RV32I
        0b0110111 | 0b0010111 => Itype::U {_op, rd, imm: instr & 0xfffff000}, // lui, auipc
        0b1101111 => Itype::J {_op, rd, imm: imm_j}, // jal
        0b1100111 | 0b0000011 | 0b0010011 => Itype::I {_op, rd, funct3, rs1, imm: instr >> 20}, // jalr, ld, imm instructions (addi, slti, etc)
        0b1100011 => Itype::B {_op, funct3, rs1, rs2, imm: imm_b}, // beq - bgeu
        0b0100011 => Itype::S {_op, funct3, rs1, rs2, imm: imm_s}, // sb - sw
        0b0110011 => Itype::R {_op, rd, funct3, rs1, rs2, funct7}, // reg-reg instructions (add, slt, etc)
        0b1110011 => Itype::Ecall {imm: (instr >> 20) as u16}, // ecall - ebreak

        // Zfencei
        0b0001111 => Itype::Fence {_rd: rd, _rs1: rs1, _succ: (instr >> 20) as u8, _pred: (instr >> 24) as u8, _fm: (instr >> 28) as u8}, // fence

        _ => panic!("unrecognized opcode 0b{:07b}", _op)
    }
}

// sign extend the value in x with b bits
pub fn sext(x: u32, b: usize) -> u32 {
    if (x >> (b - 1)) & 1 == 1 {
        return (u32::MAX << b) | x as u32
    }
    
    x as u32
}

pub fn run(imem: Vec<u8>, mut s: State, dmem: &mut Vec<u8>) -> State {
    let start = Instant::now();
    loop {
        let upc = s.pc as usize - START_ADDR;

        if upc >= imem.len() {
            panic!("exceeded instruction memory with address 0x{:x} ({}) (access 0x{:x}, len 0x{:x})", s.pc, s.pc, upc, imem.len());
        }

        let instr_32 = imem[upc] as u32 | 
            ((imem[upc + 1] as u32) << 8) | 
            ((imem[upc + 2] as u32) << 16) | 
            ((imem[upc + 3] as u32) << 24);
        
        assert_eq!(s.regs[0], 0);
        
        //println!("pc 0x{:x} ({}) fetched instr 0b{:b}", s.pc, s.pc, instr_32);
        
        s.pc += 4;

        let itype = crack(instr_32);

        //println!("type: {:?}", itype);

        match itype {
            Itype::R {_op, rd, funct3, rs1, rs2, funct7} => {
                if rd == 0 {
                    continue;
                }

                match funct3 {
                    0b000 if funct7 == 0 => add(&mut s, rs1, rs2, rd),
                    0b000 if funct7 != 0 => sub(&mut s, rs1, rs2, rd),
                    0b001 => sll(&mut s, rs1, rs2, rd),
                    0b010 => slt(&mut s, rs1, rs2, rd),
                    0b011 => sltu(&mut s, rs1, rs2, rd),
                    0b100 => xor(&mut s, rs1, rs2, rd),
                    0b101 if funct7 == 0 => srl(&mut s, rs1, rs2, rd),
                    0b101 if funct7 != 0 => sra(&mut s, rs1, rs2, rd),
                    0b110 => or(&mut s, rs1, rs2, rd),
                    0b111 => and(&mut s, rs1, rs2, rd),
                    _ => panic!("unrecognized funct3 field {:03b}", funct3)
                }
            },
            Itype::I {_op, rd, funct3, rs1, imm} => {
                let ext_imm = sext(imm, 12);

                match _op {
                    0b0010011 => {
                        match funct3 {
                            0b000 => addi(&mut s, rs1, ext_imm, rd),
                            0b010 => slti(&mut s, rs1, ext_imm, rd),
                            0b011 => sltiu(&mut s, rs1, ext_imm, rd),
                            0b100 => xori(&mut s, rs1, ext_imm, rd),
                            0b110 => ori(&mut s, rs1, ext_imm, rd),
                            0b111 => andi(&mut s, rs1, ext_imm, rd),
                            0b001 => slli(&mut s, rs1, ext_imm, rd),
                            0b101 if imm >> 10 == 0 => srli(&mut s, rs1, ext_imm, rd),
                            0b101 if imm >> 10 != 0 => srai(&mut s, rs1, ext_imm, rd),
                            _ => panic!("unrecognized funct3 field {:03b}", funct3)
                        }
                    },
                    0b0000011 => {
                        let ext_imm = sext(imm, 12);
                        match funct3 {
                            0b000 => lx(&mut s, rs1, ext_imm, rd, dmem, 8, true), // lb
                            0b100 => lx(&mut s, rs1, ext_imm, rd, dmem, 8, false), // lbu
                            0b001 => lx(&mut s, rs1, ext_imm, rd, dmem, 16, true), // lh
                            0b101 => lx(&mut s, rs1, ext_imm, rd, dmem, 16, false), // lhu
                            0b010 => lx(&mut s, rs1, ext_imm, rd, dmem, 32, false), // lw
                            _ => panic!("unrecognized funct3 field {:03b}", funct3)
                        };

                    },
                    0b1100111 => jalr(&mut s, rs1, ext_imm, rd),
                    0b0001111 if ZIFENCEI => zifence_i(&mut s, rs1, ext_imm, rd),
                    _ => panic!("unrecognized I type op field {:07b}", _op)
                }
            },
            Itype::S {_op, funct3, rs1, rs2, imm} => {
                let ext_imm = sext(imm, 12);
                match funct3 {
                    0b000 => sx(&mut s, rs1, ext_imm, rs2, dmem, 8),
                    0b001 => sx(&mut s, rs1, ext_imm, rs2, dmem, 16),
                    0b010 => sx(&mut s, rs1, ext_imm, rs2, dmem, 32),
                    _ => panic!("unrecognized funct3 field {:03b}", funct3)
                }
            },
            Itype::B {_op, funct3, rs1, rs2, imm} => {
                let ext_imm = sext(imm, 12);

                match funct3 {
                    0b000 => beq(&mut s, rs1, rs2, ext_imm),
                    0b001 => bne(&mut s, rs1, rs2, ext_imm),
                    0b100 => blt(&mut s, rs1, rs2, ext_imm),
                    0b101 => bge(&mut s, rs1, rs2, ext_imm),
                    0b110 => bltu(&mut s, rs1, rs2, ext_imm),
                    0b111 => bgeu(&mut s, rs1, rs2, ext_imm),
                    _ => panic!("unrecognized funct3 field {:03b}", funct3)
                }
            },
            Itype::U {_op, rd, imm} => {
                match _op {
                    0b0110111 => lui(&mut s, rd, imm),
                    0b0010111 => auipc(&mut s, rd, imm),
                    _ => panic!("unrecognized U type op field {:07b}", _op)
                }
            },
            Itype::J {_op, rd, imm} => jal(&mut s, rd, imm),
            Itype::Ecall {imm} => {
                let res = match imm {
                    0 => ecall(&mut s),
                    _ => ebreak(&mut s),
                };

                match res {
                    Action::Resume => (),
                    Action::Terminate => break,
                }
            },
            Itype::Fence {_rd, _rs1, _succ, _pred, _fm} => (),
        }

        s.regs[0] = 0; // reset zero register in case anything wrote to it
        s.ret += 1;
    }

    let duration = start.elapsed();
    let ips = s.ret as f64 / duration.as_secs_f64() / 1000000.0;
    println!("terminating execution");
    println!("{} instructions over {:?} ({:.3}M instr/sec)", s.ret, duration, ips);

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sext_test() {
        let res = sext(0x800, 12);

        assert_eq!(res, 0xfffff800);
    }
}
