// implements RV32I
#[derive(Debug)]
pub struct State {
    regs: [u32; 32],
    pc: u32,
    //xlen: u8, // register width
    ialign: u8, // minimum alignment
}

impl State {
    pub fn new(pc: u32) -> State {
        State {
            regs: [0; 32],
            pc,
            ialign: 32,
        }
    }
}

// stores all decoded instruction data with immediates unpacked
enum Itype {
    // RV32I
    R {_op: u8, rd: usize, funct3: u8, rs1: usize, rs2: usize, funct7: u8},
    I {_op: u8, rd: usize, funct3: u8, rs1: usize, imm: u16},
    S {_op: u8, funct3: u8, rs1: usize, rs2: usize, imm: u16},
    B {_op: u8, funct3: u8, rs1: usize, rs2: usize, imm: u16},
    U {_op: u8, rd: usize, imm: u32},
    J {_op: u8, rd: usize, imm: u32},
    Ecall {imm: u16},

    //Zfencei extension
    //Fence {rd: usize, rs1: usize, succ: u8, pred: u8, fm: u8},
}

fn crack(instr: u32) -> Itype {
    let _op = (instr & 0b111111) as u8;
    let rd = ((instr >> 7) & 0b11111) as usize;
    let funct3 = ((instr >> 12) & 0b111) as u8;
    let funct7 = (instr >> 25) as u8;
    let rs1 = ((instr >> 15) & 0b11111) as usize;
    let rs2 = ((instr >> 20) & 0b11111) as usize;

    let imm_j = 
        (((instr >> 21) & 0b1111111111) << 1) + 
        ((instr >> 20) << 10) + 
        (((instr >> 12) & 0b11111111) << 12) + 
        ((instr >> 31) << 20);
    
    let imm_b = 
        (((instr >> 8) & 0b1111) << 1) +
        (((instr >> 25) & 0b111111) << 5) +
        ((instr >> 7) << 11) + 
        ((instr >> 31) << 12);
    
    let imm_s = 
        ((instr >> 7) & 0b11111) + (((instr >> 25) & 0b1111111) << 5);

    match _op {
        // RV32I
        0b011011 | 0b001011 => Itype::U {_op, rd, imm: instr >> 12}, // lui, auipc
        0b1101111 => Itype::J {_op, rd, imm: imm_j}, // jal
        0b1100111 | 0b0000011 | 0b0010011 => Itype::I {_op, rd, funct3, rs1, imm: (instr >> 20) as u16}, // jalr, ld, imm instructions (addi, slti, etc)
        0b1100011 => Itype::B {_op, funct3, rs1, rs2, imm: imm_b as u16}, // beq - bgeu
        0b0100011 => Itype::S {_op, funct3, rs1, rs2, imm: imm_s as u16}, // sb - sw
        0b0110011 => Itype::R {_op, rd, funct3, rs1, rs2, funct7}, // reg-reg instructions (add, slt, etc)
        0b1110011 => Itype::Ecall {imm: (instr >> 20) as u16}, // ecall - ebreak

        // Zfencei
        //0b0001111 => Itype::Fence {op, rd, rs1, }, // fence

        _ => panic!("unrecognized opcode {}", _op)
    }
}

// sign extend the value in x with b bits
fn sext(x: u32, b: usize) -> u32 {
    if (x >> (b - 1)) & 1 == 1 {
        return (u32::MAX << b) | x as u32
    }
    
    x as u32
}

pub fn run(imem: Vec<u8>, mut s: State, dmem_len: usize) {
    let mut dmem: Vec<u32> = vec![0; dmem_len];

    loop {
        let upc = s.pc as usize;

        if upc >= imem.len() {
            println!("hit end of instruction memory");
            break;
        }

        let instr_32 = imem[upc] as u32 + 
            (imem[upc + 1] as u32 >> 8) + 
            (imem[upc + 2] as u32 >> 16) + 
            (imem[upc + 3] as u32 >> 24);
        
        s.pc += 4;

        let itype = crack(instr_32);

        match itype {
            Itype::R {_op, rd, funct3, rs1, rs2, funct7} => {
                if rd == 0 {
                    continue;
                }

                match funct3 {
                    0b000 if funct7 == 0 => s.regs[rd] = s.regs[rs1].wrapping_add(s.regs[rs2]), // add
                    0b000 if funct7 != 0 => s.regs[rd] = s.regs[rs1].wrapping_sub(s.regs[rs2]), // sub
                    0b001 => s.regs[rd] = s.regs[rs1] << (s.regs[rs2] & 0b11111), // sll
                    0b010 => {
                        if (s.regs[rs1] as i32) < s.regs[rs2] as i32 {
                            s.regs[rd] = 0
                        } else {
                            s.regs[rd] = 1
                        }
                    }, // slt
                    0b011 => {
                        if s.regs[rs1] < s.regs[rs2] {
                            s.regs[rd] = 0
                        } else {
                            s.regs[rd] = 1
                        }
                    }, // sltu
                    0b100 => s.regs[rd] = s.regs[rs1] ^ s.regs[rs2], // xor
                    0b101 if funct7 == 0 => s.regs[rd] = s.regs[rs1] >> (s.regs[rs2] & 0b11111), // srl
                    0b101 if funct7 != 0 => s.regs[rd] = (s.regs[rs1] as i32 >> (s.regs[rs2] & 0b11111) as i32) as u32, // sra
                    0b110 => s.regs[rd] = s.regs[rs1] | s.regs[rs2], // or
                    0b111 => s.regs[rd] = s.regs[rs1] & s.regs[rs2], // and
                    _ => panic!("illegal funct3 field {}", funct3)
                }
            },
            Itype::I {_op, rd, funct3, rs1, imm} => {
                if rd == 0 {
                    continue;
                }

                let ext_imm = sext(imm as u32, 12);

                match _op {
                    0b0010011 => {
                        match funct3 {
                            0b000 => s.regs[rd] = s.regs[rs1].wrapping_add(ext_imm), // addi
                            0b010 => {
                                if (s.regs[rs1] as i32) < ext_imm as i32 {
                                    s.regs[rd] = 0
                                } else {
                                    s.regs[rd] = 1
                                }
                            }, // slti
                            0b011 => {
                                if s.regs[rs1] < ext_imm {
                                    s.regs[rd] = 0
                                } else {
                                    s.regs[rd] = 1
                                }
                            }, // sltiu
                            0b100 => s.regs[rd] = s.regs[rs1] ^ ext_imm, // xori
                            0b110 => s.regs[rd] = s.regs[rs1] | ext_imm, // ori
                            0b111 => s.regs[rd] = s.regs[rs1] & ext_imm, // andi
                            0b001 => s.regs[rd] = s.regs[rs1] << ext_imm, // slli
                            0b101 if imm == 0 => s.regs[rd] = s.regs[rs1] >> ext_imm, // srli
                            0b101 if imm != 0 => s.regs[rd] = (s.regs[rs1] as i32 >> ext_imm as i32) as u32, // srai
                            _ => panic!("illegal funct3 field {}", funct3)
                        }
                    }, // immediate instruction
                    0b0000011 => {
                        let ext_imm = sext(imm as u32, 12);
                        let addr = (rs1 as u32 + ext_imm) as usize;

                        if addr >= dmem_len {
                            panic!("illegal memory read at address {:x}", addr)
                        }

                        if addr % (s.ialign / 8) as usize != 0 {
                            panic!("misaligned memory access {}", addr)
                        }

                        let mut val = match funct3 {
                            0b000 | 0b100 => dmem[addr] % u8::MAX as u32, // lb(u)
                            0b001 | 0b101 => dmem[addr] % u16::MAX as u32, // lh(u)
                            0b010 => dmem[addr], // lw
                            _ => panic!("illegal funct3 field {}", funct3)
                        };

                        // we can't move this up because exceptions still need to happen when rd is x0
                        if rd == 0 {
                            continue;
                        }

                        // lb and lh need sign-extension
                        match funct3 {
                            0b000 => val = sext(val, 8),
                            0b001 => val = sext(val, 16),
                            _ => (),
                        }

                        s.regs[rd] = val;

                    }, // load
                    0b1100111 => {
                        s.regs[rd] = s.pc;
                        s.pc += (s.regs[rs1] + sext(imm as u32, 12)) & 0xfffffffe;
                    }, // jalr
                    _ => panic!("unrecognized opcode {}", _op)
                }
            },
            Itype::S {_op, funct3, rs1, rs2, imm} => {
                let ext_imm = if imm >> 12 & 1 == 1 { u32::MAX << 12 & imm as u32 } else { imm as u32 };
                let addr = (rs1 as u32 + ext_imm) as usize;

                if addr >= dmem_len {
                    panic!("illegal memory write at address {:x}", addr)
                }

                if addr % (s.ialign / 8) as usize != 0 {
                    panic!("misaligned memory access {}", addr)
                }

                match funct3 {
                    0b000 => dmem[addr] = (s.regs[rs2] as u8) as u32,
                    0b001 => dmem[addr] = (s.regs[rs2] as u16) as u32,
                    0b010 => dmem[addr] = s.regs[rs2],
                    _ => panic!("illegal funct3 field {}", funct3)
                }
            },
            Itype::B {_op, funct3, rs1, rs2, imm} => {
                let ext_imm = if imm >> 12 & 1 == 1 { u32::MAX << 12 & imm as u32 } else { imm as u32 };
                let new_addr = ((s.pc - 4) as i32 + ext_imm as i32) as u32;

                match funct3 {
                    0b000 => {
                        if s.regs[rs1] == s.regs[rs2] {
                            s.pc = new_addr;
                        }
                    }, // beq
                    0b001 => {
                        if s.regs[rs1] != s.regs[rs2] {
                            s.pc = new_addr;
                        }
                    }, // bne
                    0b100 => {
                        if (s.regs[rs1] as i32) < s.regs[rs2] as i32 {
                            s.pc = new_addr;
                        }
                    }, // blt
                    0b101 => {
                        if s.regs[rs1] as i32 >= s.regs[rs2] as i32 {
                            s.pc = new_addr;
                        }
                    }, // bge
                    0b110 => {
                        if s.regs[rs1] < s.regs[rs2] {
                            s.pc = new_addr;
                        }
                    }, // bltu
                    0b111 => {
                        if s.regs[rs1] >= s.regs[rs2] {
                            s.pc = new_addr;
                        }
                    }, // bgeu
                    _ => panic!("illegal funct3 field {}", funct3)
                }
            },
            Itype::U {_op, rd, imm} => {
                match _op {
                    0b0110111 => s.regs[rd] = imm << 12, // lui
                    0b0010111 => s.regs[rd] = (s.pc - 4) + (imm << 12), // auipc
                    _ => panic!("unknown op field {}", _op)
                }
            },
            Itype::J {_op, rd, imm} => {
                s.regs[rd] = s.pc;
                s.pc += sext(imm, 20) + s.pc - 4;
            }, // jal
            Itype::Ecall {imm} => {
                panic!("unimplemented Ecall type instruction, imm {}", imm);
            },
        }
    }
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
