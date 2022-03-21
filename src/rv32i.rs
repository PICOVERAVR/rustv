use crate::{sext, State};

// core rv32i instructions

pub enum Action {
    Terminate, // terminate execution
    Resume,    // resume execution
}

impl State {
    pub fn add(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2])
    }

    pub fn sub(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2])
    }

    pub fn sll(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1] << (self.regs[rs2] & 0b11111)
    }

    pub fn slt(&mut self, rs1: usize, rs2: usize, rd: usize) {
        if (self.regs[rs1] as i32) < self.regs[rs2] as i32 {
            self.regs[rd] = 1
        } else {
            self.regs[rd] = 0
        }
    }

    pub fn sltu(&mut self, rs1: usize, rs2: usize, rd: usize) {
        if self.regs[rs1] < self.regs[rs2] {
            self.regs[rd] = 1
        } else {
            self.regs[rd] = 0
        }
    }

    pub fn xor(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2]
    }

    pub fn srl(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1] >> (self.regs[rs2] & 0b11111)
    }

    pub fn sra(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = (self.regs[rs1] as i32 >> (self.regs[rs2] & 0b11111) as i32) as u32
    }

    pub fn or(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1] | self.regs[rs2]
    }

    pub fn and(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1] & self.regs[rs2]
    }

    pub fn addi(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1].wrapping_add(ext_imm)
    }

    pub fn slti(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        if (self.regs[rs1] as i32) < ext_imm as i32 {
            self.regs[rd] = 1
        } else {
            self.regs[rd] = 0
        }
    }

    pub fn sltiu(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        if self.regs[rs1] < ext_imm {
            self.regs[rd] = 1
        } else {
            self.regs[rd] = 0
        }
    }

    pub fn xori(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1] ^ ext_imm
    }

    pub fn ori(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1] | ext_imm
    }

    pub fn andi(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1] & ext_imm
    }

    pub fn slli(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1] << ext_imm
    }

    pub fn srli(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        self.regs[rd] = self.regs[rs1] >> ext_imm
    }

    pub fn srai(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        // need to mask off high bits of ext_imm since only the lower 5
        // are used and the 10th bit is set to 1
        self.regs[rd] = (self.regs[rs1] as i32 >> (ext_imm & 0b11111) as i32) as u32
    }

    // implements lb(u), lh(u), lw
    pub fn lx(
        &mut self,
        rs1: usize,
        ext_imm: u32,
        rd: usize,
        dmem: &[u8],
        len: usize,
        signed: bool,
    ) {
        assert!(len == 8 || len == 16 || len == 32);

        // wrapping add to handle negative immediates
        let addr = (self.regs[rs1].wrapping_add(ext_imm)) as usize;

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
            val |= (dmem[addr + i / 8] as u32) << i;
        }

        // lb and lh need sign-extension
        self.regs[rd] = match signed {
            true => sext(val, len),
            false => val,
        };
    }

    // implements sb, sh, sw
    pub fn sx(&mut self, rs1: usize, ext_imm: u32, rs2: usize, dmem: &mut [u8], len: usize) {
        assert!(len == 8 || len == 16 || len == 32);

        // wrapping add to handle negative immediates
        let addr = (self.regs[rs1].wrapping_add(ext_imm)) as usize;
        let word_addr = addr / 4;

        if word_addr >= dmem.len() {
            panic!("illegal memory write at byte address {:x}", addr)
        }

        match addr % 4 {
            1 | 2 | 3 if len == 32 => panic!("misaligned word store at byte address {:x}", addr),
            1 | 3 if len == 16 => panic!("misaligned halfword store at byte address {:x}", addr),
            _ => (),
        }

        let val = self.regs[rs2];

        for i in (0..len).step_by(8) {
            dmem[addr + i / 8] = (val >> i) as u8;
        }
    }

    pub fn jalr(&mut self, rs1: usize, ext_imm: u32, rd: usize) {
        let dest = ((self.regs[rs1] as i32 + ext_imm as i32) & -2) as u32; // -2 unsigned is 0b11..110

        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        self.regs[rd] = self.pc;
        self.pc = dest;
    }

    pub fn beq(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if self.regs[rs1] == self.regs[rs2] {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn bne(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if self.regs[rs1] != self.regs[rs2] {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn blt(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if (self.regs[rs1] as i32) < self.regs[rs2] as i32 {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn bge(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if (self.regs[rs1] as i32) >= self.regs[rs2] as i32 {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn bltu(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if self.regs[rs1] < self.regs[rs2] {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn bgeu(&mut self, rs1: usize, rs2: usize, ext_imm: u32) {
        if self.regs[rs1] >= self.regs[rs2] {
            let dest = ((self.pc - 4) as i32 + ext_imm as i32) as u32;
            if dest % 4 != 0 {
                panic!("misaligned destination byte address {:x}", dest);
            }

            self.pc = dest;
        }
    }

    pub fn lui(&mut self, rd: usize, upper_imm: u32) {
        self.regs[rd] = upper_imm
    }

    pub fn auipc(&mut self, rd: usize, upper_imm: u32) {
        self.regs[rd] = (self.pc - 4) + upper_imm
    }

    pub fn jal(&mut self, rd: usize, imm: u32) {
        let dest = ((self.pc - 4) as i32 + sext(imm, 20) as i32) as u32;

        if dest % 4 != 0 {
            panic!("misaligned destination byte address {:x}", dest);
        }

        self.regs[rd] = self.pc;
        self.pc = dest;
    }

    pub fn ecall(&mut self) -> Action {
        match self.regs[8] {
            0 => assert_eq!(
                self.regs[6], self.regs[7],
                "l: 0x{:08x}, r: 0x{:08x}",
                self.regs[6], self.regs[7]
            ),
            1 => print!("{}", self.regs[6] as u8 as char),
            err => panic!(
                "unknown ecall parameter 0x{:x} in x8, pc 0x{:x}",
                err,
                self.pc - 4
            ),
        }

        Action::Resume
    }

    pub fn ebreak(&mut self) -> Action {
        Action::Terminate
    }

    pub fn fence(&mut self, _rd: usize, _rs1: usize, _succ: u8, _pred: u8, _fm: u8) {}
}
