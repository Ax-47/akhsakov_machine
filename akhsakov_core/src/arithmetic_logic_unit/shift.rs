use super::accumulator::Accumulator;
use crate::componants::componant::Componant;
use crate::{registers::general_register::GeneralRegister, types::Binary};
use std::ops;
impl ops::Shl<&GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn shl(mut self, gr: &GeneralRegister) -> Self::Output {
        for _ in 0..gr.data.to_int().min(38) {
            for i in 0..37 {
                self.result.0[i] = self.result.0[i + 1];
            }
            self.result.0[37] = false;
        }
        self
    }
}
impl ops::Shr<&GeneralRegister> for Accumulator {
    type Output = Accumulator;

    fn shr(mut self, gr: &GeneralRegister) -> Self::Output {
        for _ in 0..gr.data.to_int().min(38) {
            for i in (1..38).rev() {
                self.result.0[i] = self.result.0[i - 1];
            }
            self.result.0[0] = false;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shr() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();
        gr.load(Binary::from(0b1010_u32));
        alu = alu + &gr;
        gr.load(Binary::from(0b1_u32));
        alu = alu >> &gr;
        assert_eq!(alu.result, Binary::from(0b0101_u32));
    }

    #[test]
    fn test_shl() {
        let mut alu = Accumulator::new();
        let mut gr = GeneralRegister::new();
        gr.load(Binary::from(0b101_u32));
        alu = alu + &gr;
        gr.load(Binary::from(0b1_u32));
        alu = alu << &gr;
        assert_eq!(alu.result, Binary::from(0b01010_u32));
    }
}
