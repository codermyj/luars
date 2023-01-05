use std::ops::{BitAnd, Shr};
use super::opcodes::OPCODES;
/// sBx为18位有符号整数，MAXARG_sBx为能表示的最大无符号整数的一半
const MAXARG_Bx: i32 = 1 << 18 -1;
const MAXARG_sBx: i32 = MAXARG_Bx >> 1;

pub struct Instruction(pub u32);

impl BitAnd<u32> for &Instruction {
    type Output = i32;

    fn bitand(self, rhs: u32) -> Self::Output {
        (self.0 & rhs) as i32
    }
}

impl Shr<u32> for &Instruction {
    type Output = i32;

    fn shr(self, rhs: u32) -> Self::Output {
        (self.0 >> rhs) as i32
    }
}

impl Instruction {
    pub fn opcode(&self) -> usize {
        (self & 0x3F) as usize
    }

    pub fn abc(&self) -> (i32, i32, i32) {
        let a = self >> 6 & 0xFF;
        let c = self >> 14 & 0x1FF;
        let b = self >> 23 & 0x1FF;
        (a, b, c)
    }

    pub fn a_bx(&self) -> (i32, i32) {
        let a = self >> 6 & 0xFF;
        let bx = self >> 14;
        (a, bx)
    }

    pub fn a_sbx(&self) -> (i32, i32) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_sBx)
    }

    pub fn op_name(&self) -> &str{
        OPCODES[self.opcode()].name
    }

    pub fn op_mode(&self) -> u8 {
        OPCODES[self.opcode()].op_mode
    }

    pub fn b_mode(&self) -> u8 {
        OPCODES[self.opcode()].arg_B_mode
    }

    pub fn c_mode(&self) -> u8 {
        OPCODES[self.opcode()].arg_C_mode
    }
}