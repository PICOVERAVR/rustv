use crate::State;

// multiply extension instructions

impl State {
    pub fn mul(&mut self, rs1: usize, rs2: usize, rd: usize) {
        self.regs[rd] = self.regs[rs1].wrapping_mul(self.regs[rs2]);
    }

    pub fn mulh(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let tmp = (self.regs[rs1] as i64).wrapping_mul(self.regs[rs2] as i64);
        self.regs[rd] = ((tmp >> 32) & 0xFFFF_FFFF) as u32;
    }

    pub fn mulhu(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let tmp = (self.regs[rs1] as u64).wrapping_mul(self.regs[rs2] as u64);
        self.regs[rd] = ((tmp >> 32) & 0xFFFF_FFFF) as u32;
    }

    pub fn mulhsu(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let tmp = ((self.regs[rs1] as i64) as u64).wrapping_mul(self.regs[rs2] as u64);
        self.regs[rd] = ((tmp >> 32) & 0xFFFF_FFFF) as u32;
    }

    // dividend = divisor * quotient + remainder

    pub fn div(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let dividend = self.regs[rs1];
        let divisor = self.regs[rs2];

        if divisor == 0 {
            self.regs[rd] = 0xFFFF_FFFF;
        } else {
            self.regs[rd] = ((dividend as i32).wrapping_div(divisor as i32)) as u32;
        }
    }

    pub fn divu(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let dividend = self.regs[rs1];
        let divisor = self.regs[rs2];

        if divisor == 0 {
            self.regs[rd] = 0xFFFF_FFFF;
        } else {
            self.regs[rd] = dividend.wrapping_div(divisor);
        }
    }

    pub fn rem(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let dividend = self.regs[rs1];
        let divisor = self.regs[rs2];

        if divisor == 0 {
            self.regs[rd] = dividend;
        } else {
            self.regs[rd] = ((dividend as i32).wrapping_rem(divisor as i32)) as u32;
        }
    }

    pub fn remu(&mut self, rs1: usize, rs2: usize, rd: usize) {
        let dividend = self.regs[rs1];
        let divisor = self.regs[rs2];

        if divisor == 0 {
            self.regs[rd] = dividend;
        } else {
            self.regs[rd] = dividend.wrapping_rem(divisor);
        }
    }
}

#[test]
// make sure division handles edge cases listed in spec gracefully
fn test_div() {
    let mut s = State {
        ..Default::default()
    };

    s.regs[5] = 0x8000_0000;
    s.regs[4] = 0xFFFF_FFFF;

    s.div(5, 4, 1);

    assert_eq!(s.regs[1], 0x8000_0000);

    s.regs[5] = 4;
    s.regs[4] = 0;

    s.div(5, 4, 1);

    assert_eq!(s.regs[1], 0xFFFF_FFFF);
}

#[test]
// make sure computing a remainder handles edge cases listed in spec gracefully
fn test_rem() {
    let mut s = State {
        ..Default::default()
    };

    s.regs[5] = 0x8000_0000;
    s.regs[4] = 0xFFFF_FFFF;

    s.rem(5, 4, 1);

    assert_eq!(s.regs[1], 0);

    s.regs[5] = 27;
    s.regs[4] = 0;

    s.rem(5, 4, 1);

    assert_eq!(s.regs[1], 27);
}
