use std::time::Instant;

mod rv32i;
use rv32i::*;

mod rv32m;
mod zifencei;

// list of recognized extensions (may not be fully supported!)
#[derive(Debug, PartialEq)]
pub enum Ext {
    All,
    Zifencei,
    M,
}

// implements rv32i
// xlen == 32
#[derive(Debug, Default)]
pub struct State {
    regs: [u32; 32],
    pc: u32,
    pc_offset: u32,
    ret: u64, // number of retired instructions

    ext: Vec<Ext>,
}

impl State {
    // creates a new state object, pc_offset is used to map the pc into an instruction vector index
    pub fn new(pc: usize, pc_offset: usize, isa: Vec<Ext>) -> State {
        State {
            regs: [0; 32],
            pc: pc as u32,
            pc_offset: pc_offset as u32,
            ret: 0,
            ext: isa,
        }
    }

    pub fn gprs(&self) -> [u32; 32] {
        self.regs
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn has_ext(&self, ext: Ext) -> bool {
        self.ext.contains(&ext) || self.ext.contains(&Ext::All)
    }
}

// stores all decoded instruction data with immediates unpacked
#[derive(Debug)]
enum Itype {
    // RV32I
    R {
        _op: u8,
        rd: usize,
        funct3: u8,
        rs1: usize,
        rs2: usize,
        funct7: u8,
    },
    I {
        _op: u8,
        rd: usize,
        funct3: u8,
        rs1: usize,
        imm: u32,
    },
    S {
        _op: u8,
        funct3: u8,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    B {
        _op: u8,
        funct3: u8,
        rs1: usize,
        rs2: usize,
        imm: u32,
    },
    U {
        _op: u8,
        rd: usize,
        imm: u32,
    },
    J {
        _op: u8,
        rd: usize,
        imm: u32,
    },
    Ecall {
        imm: u16,
    },
    Fence {
        rd: usize,
        funct3: u8,
        rs1: usize,
        succ: u8,
        pred: u8,
        fm: u8,
    },
}

fn crack(instr: u32) -> Itype {
    let _op = (instr & 0b1111111) as u8;
    let rd = ((instr >> 7) & 0b11111) as usize;
    let funct3 = ((instr >> 12) & 0b111) as u8;
    let funct7 = (instr >> 25) as u8;
    let rs1 = ((instr >> 15) & 0b11111) as usize;
    let rs2 = ((instr >> 20) & 0b11111) as usize;

    let imm_j = (((instr >> 21) & 0b1111111111) << 1)
        | (((instr >> 20) & 0b1) << 10)
        | (((instr >> 12) & 0b11111111) << 12)
        | ((instr >> 31) << 20);

    let imm_b = (((instr >> 8) & 0b1111) << 1)
        | (((instr >> 25) & 0b111111) << 5)
        | (((instr >> 7) & 0b1) << 11)
        | (((instr >> 31) & 0b1) << 12);

    let imm_s = ((instr >> 7) & 0b11111) | (((instr >> 25) & 0b1111111) << 5);

    match _op {
        // RV32I
        0b0110111 | 0b0010111 => Itype::U {
            _op,
            rd,
            imm: instr & 0xfffff000,
        }, // lui, auipc
        0b1101111 => Itype::J {
            _op,
            rd,
            imm: imm_j,
        }, // jal
        0b1100111 | 0b0000011 | 0b0010011 => Itype::I {
            _op,
            rd,
            funct3,
            rs1,
            imm: instr >> 20,
        }, // jalr, ld, imm instructions (addi, slti, etc)
        0b1100011 => Itype::B {
            _op,
            funct3,
            rs1,
            rs2,
            imm: imm_b,
        }, // beq - bgeu
        0b0100011 => Itype::S {
            _op,
            funct3,
            rs1,
            rs2,
            imm: imm_s,
        }, // sb - sw
        0b0110011 => Itype::R {
            _op,
            rd,
            funct3,
            rs1,
            rs2,
            funct7,
        }, // reg-reg instructions (add, slt, etc)
        0b1110011 if rd == 0 && funct3 == 0 && rs1 == 0 => Itype::Ecall {
            imm: (instr >> 20) as u16,
        }, // ecall, ebreak
        0b0001111 => Itype::Fence {
            rd,
            funct3,
            rs1,
            succ: (instr >> 20) as u8,
            pred: (instr >> 24) as u8,
            fm: (instr >> 28) as u8,
        }, // fence, fence.i

        _ => panic!("unrecognized opcode 0b{:07b}", _op),
    }
}

// sign extend the value in x with b bits
pub fn sext(x: u32, b: usize) -> u32 {
    if (x >> (b - 1)) & 1 == 1 {
        return (u32::MAX << b) | x as u32;
    }

    x as u32
}

pub fn run(imem: Vec<u8>, mut s: State, dmem: &mut Vec<u8>) -> State {
    let start = Instant::now();
    loop {
        let upc = s.pc as usize - s.pc_offset as usize;

        if upc >= imem.len() {
            panic!(
                "exceeded instruction memory with address 0x{:x} ({}) (access 0x{:x} but memory size 0x{:x})",
                s.pc,
                s.pc,
                upc,
                imem.len()
            );
        }

        let instr_32 = imem[upc] as u32
            | ((imem[upc + 1] as u32) << 8)
            | ((imem[upc + 2] as u32) << 16)
            | ((imem[upc + 3] as u32) << 24);

        assert_eq!(s.regs[0], 0);

        //println!("pc 0x{:x} ({}) fetched instr 0b{:b}", s.pc, s.pc, instr_32);

        s.pc += 4;

        let itype = crack(instr_32);

        //println!("type: {:?}", itype);

        match itype {
            Itype::R {
                _op,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                if rd == 0 {
                    continue;
                }

                match _op {
                    0b0110011 if s.has_ext(Ext::M) && funct7 == 1 => match funct3 {
                        0b000 => s.mul(rs1, rs2, rd),
                        0b001 => s.mulh(rs1, rs2, rd),
                        0b010 => s.mulhsu(rs1, rs2, rd),
                        0b011 => s.mulhu(rs1, rs2, rd),
                        0b100 => s.div(rs1, rs2, rd),
                        0b101 => s.divu(rs1, rs2, rd),
                        0b110 => s.rem(rs1, rs2, rd),
                        0b111 => s.remu(rs1, rs2, rd),
                        _ => panic!("unrecognized funct3 field {:03b}", funct3),
                    },
                    0b0110011 => match funct3 {
                        // need to check funct7 field in all cases because most bits must be set to
                        // 0 and we don't want to accidentally execute an invalid instruction
                        0b000 if funct7 == 0 => s.add(rs1, rs2, rd),
                        0b000 if funct7 == 0b0100000 => s.sub(rs1, rs2, rd),
                        0b001 if funct7 == 0 => s.sll(rs1, rs2, rd),
                        0b010 if funct7 == 0 => s.slt(rs1, rs2, rd),
                        0b011 if funct7 == 0 => s.sltu(rs1, rs2, rd),
                        0b100 if funct7 == 0 => s.xor(rs1, rs2, rd),
                        0b101 if funct7 == 0 => s.srl(rs1, rs2, rd),
                        0b101 if funct7 == 0b0100000 => s.sra(rs1, rs2, rd),
                        0b110 if funct7 == 0 => s.or(rs1, rs2, rd),
                        0b111 if funct7 == 0 => s.and(rs1, rs2, rd),
                        _ => panic!("unrecognized funct3 field {:03b}", funct3),
                    },
                    _ => panic!("unrecognized op field {:07b}", _op),
                }
            }
            Itype::I {
                _op,
                rd,
                funct3,
                rs1,
                imm,
            } => {
                let ext_imm = sext(imm, 12);

                match _op {
                    0b0010011 => match funct3 {
                        0b000 => s.addi(rs1, ext_imm, rd),
                        0b010 => s.slti(rs1, ext_imm, rd),
                        0b011 => s.sltiu(rs1, ext_imm, rd),
                        0b100 => s.xori(rs1, ext_imm, rd),
                        0b110 => s.ori(rs1, ext_imm, rd),
                        0b111 => s.andi(rs1, ext_imm, rd),
                        0b001 => s.slli(rs1, ext_imm, rd),
                        0b101 if imm >> 10 == 0 => s.srli(rs1, ext_imm, rd),
                        0b101 if imm >> 10 != 0 => s.srai(rs1, ext_imm, rd),
                        _ => panic!("unrecognized funct3 field {:03b}", funct3),
                    },
                    0b0000011 => {
                        let ext_imm = sext(imm, 12);
                        match funct3 {
                            0b000 => s.lx(rs1, ext_imm, rd, dmem, 8, true),  // lb
                            0b100 => s.lx(rs1, ext_imm, rd, dmem, 8, false), // lbu
                            0b001 => s.lx(rs1, ext_imm, rd, dmem, 16, true), // lh
                            0b101 => s.lx(rs1, ext_imm, rd, dmem, 16, false), // lhu
                            0b010 => s.lx(rs1, ext_imm, rd, dmem, 32, false), // lw
                            _ => panic!("unrecognized funct3 field {:03b}", funct3),
                        };
                    }
                    0b1100111 => s.jalr(rs1, ext_imm, rd),
                    _ => panic!("unrecognized I type op field {:07b}", _op),
                }
            }
            Itype::S {
                _op,
                funct3,
                rs1,
                rs2,
                imm,
            } => {
                let ext_imm = sext(imm, 12);
                match funct3 {
                    0b000 => s.sx(rs1, ext_imm, rs2, dmem, 8),
                    0b001 => s.sx(rs1, ext_imm, rs2, dmem, 16),
                    0b010 => s.sx(rs1, ext_imm, rs2, dmem, 32),
                    _ => panic!("unrecognized funct3 field {:03b}", funct3),
                }
            }
            Itype::B {
                _op,
                funct3,
                rs1,
                rs2,
                imm,
            } => {
                let ext_imm = sext(imm, 12);

                match funct3 {
                    0b000 => s.beq(rs1, rs2, ext_imm),
                    0b001 => s.bne(rs1, rs2, ext_imm),
                    0b100 => s.blt(rs1, rs2, ext_imm),
                    0b101 => s.bge(rs1, rs2, ext_imm),
                    0b110 => s.bltu(rs1, rs2, ext_imm),
                    0b111 => s.bgeu(rs1, rs2, ext_imm),
                    _ => panic!("unrecognized funct3 field {:03b}", funct3),
                }
            }
            Itype::U { _op, rd, imm } => match _op {
                0b0110111 => s.lui(rd, imm),
                0b0010111 => s.auipc(rd, imm),
                _ => panic!("unrecognized U type op field {:07b}", _op),
            },
            Itype::J { _op, rd, imm } => s.jal(rd, imm),
            Itype::Ecall { imm } => {
                let res = match imm {
                    0 => s.ecall(),
                    1 => s.ebreak(),
                    _ => panic!("unrecognized ecall field {:012b}", imm),
                };

                match res {
                    Action::Resume => (),
                    Action::Terminate => break,
                }
            }
            // NOTE: fence and fence.i both do nothing atm
            Itype::Fence {
                rd,
                funct3,
                rs1,
                succ,
                pred,
                fm,
            } => match funct3 {
                0 => s.fence(rd, rs1, succ, pred, fm),
                1 if s.has_ext(Ext::Zifencei) => s.fencei(rd, rs1, succ, pred, fm),
                _ => panic!("unrecognized funct3 field {:03b}", funct3),
            },
        }

        s.regs[0] = 0; // reset zero register in case anything wrote to it
        s.ret += 1;
    }

    let duration = start.elapsed();
    let ips = s.ret as f64 / duration.as_secs_f64() / 1000000.0;
    println!("terminating execution");
    println!(
        "{} instructions over {:?} ({:.3}M instr/sec)",
        s.ret, duration, ips
    );

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
