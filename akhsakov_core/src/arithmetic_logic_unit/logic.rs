use super::accumulator::Accumulator;
use crate::componants::componant::Componant;
use crate::{registers::general_register::GeneralRegister, types::Binary};
use std::ops;
impl ops::Not for Accumulator {
    type Output = Accumulator;
    fn not(mut self) -> Self::Output {
        self.result.0 = self.result.0.map(|i| !i);
        self
    }
}

impl ops::BitAnd<GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn bitand(mut self, gr: GeneralRegister) -> Self::Output {
        for (a, b) in self.result.0.iter_mut().zip(gr.data.0) {
            *a &= b;
        }
        self
    }
}

impl ops::BitOr<GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn bitor(mut self, gr: GeneralRegister) -> Self::Output {
        for (a, b) in self.result.0.iter_mut().zip(gr.data.0) {
            *a |= b;
        }
        self
    }
}

impl ops::BitXor<GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn bitxor(mut self, gr: GeneralRegister) -> Self::Output {
        for (a, b) in self.result.0.iter_mut().zip(gr.data.0) {
            *a ^= b;
        }
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_not() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();

        gr.load(Binary::from(0b1010_u32));
        alu = alu + &gr;

        let result = !alu;

        // invert ทุก bit
        assert_eq!(result.result, Binary::from(!0b1010_u64));
    }
    #[test]
    fn test_and() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();

        gr.load(Binary::from(0b1100_u32));
        alu = alu + &gr;

        let mut gr2 = GeneralRegister::new();
        gr2.load(Binary::from(0b1010_u32));

        alu = alu & gr2;

        assert_eq!(alu.result, Binary::from(0b1000_u32));
    }
    #[test]
    fn test_or() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();

        gr.load(Binary::from(0b1100_u32));
        alu = alu + &gr;

        let mut gr2 = GeneralRegister::new();
        gr2.load(Binary::from(0b0011_u32));

        alu = alu | gr2;

        assert_eq!(alu.result, Binary::from(0b1111_u32));
    }
    #[test]
    fn test_xor() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();

        gr.load(Binary::from(0b1100_u32));
        alu = alu + &gr;

        let mut gr2 = GeneralRegister::new();
        gr2.load(Binary::from(0b1010_u32));

        alu = alu ^ gr2;

        assert_eq!(alu.result, Binary::from(0b0110_u32));
    }
    #[test]
    fn test_chain_ops() {
        let mut alu = Accumulator::new();

        let mut a = GeneralRegister::new();
        let mut b = GeneralRegister::new();

        a.load(Binary::from(0b1100_u32));
        b.load(Binary::from(0b1010_u32));

        alu = alu + &a; // 1100
        alu = alu & b; // 1000
        alu = !alu; // invert

        assert_eq!(alu.result, Binary::from(!0b1000_u64));
    }
    #[test]
    fn test_zero() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();

        gr.load(Binary::from(0_u32));
        alu = alu + &gr;

        let result = !alu;

        assert_eq!(result.result, Binary::from(!0_u64));
    }
}
