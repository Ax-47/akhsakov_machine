use super::accumulator::Accumulator;
use crate::componants::componant::Componant;
use crate::{registers::general_register::GeneralRegister, types::Binary};
use std::ops;
pub(crate) trait Arithmetic {
    fn increase(&mut self);
    fn two_complement(&mut self);
}

impl Arithmetic for Accumulator {
    fn increase(&mut self) {
        let mut curry = false;
        let mut next_curry;
        for (a, b) in self.result.0.iter_mut().zip(Binary::from(0b1_u32).0).rev() {
            next_curry = (b & curry) | (*a & (b ^ curry));
            *a ^= b ^ curry;
            curry = next_curry;
        }
    }
    fn two_complement(&mut self) {
        *self = !(*self);
        self.increase();
    }
}
impl ops::Add<&GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn add(mut self, gr: &GeneralRegister) -> Self::Output {
        let mut curry = false;
        let mut next_curry;
        for (a, b) in self.result.0.iter_mut().zip(gr.data.0).rev() {
            next_curry = (b & curry) | (*a & (b ^ curry));
            *a ^= b ^ curry;
            curry = next_curry;
        }
        self
    }
}

impl ops::Add<&Binary> for Accumulator {
    type Output = Accumulator;
    fn add(mut self, word: &Binary) -> Self::Output {
        let mut curry = false;
        let mut next_curry;
        for (a, b) in self.result.0.iter_mut().zip(word.0).rev() {
            next_curry = (b & curry) | (*a & (b ^ curry));
            *a ^= b ^ curry;
            curry = next_curry;
        }
        self
    }
}

impl ops::Sub<&GeneralRegister> for Accumulator {
    type Output = Accumulator;
    fn sub(mut self, gr: &GeneralRegister) -> Self::Output {
        self.two_complement();
        self + gr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adder() {
        let mut ac = Accumulator::new();
        let mut gr = GeneralRegister::new();
        let data = Binary::from(5_u32);
        gr.load(data);
        ac = ac + &gr;
        assert_eq!(Binary::from(0b101_u32), ac.result);
        ac = ac + &gr;
        assert_eq!(Binary::from(0b1010_u32), ac.result);
    }

    #[test]
    fn complement() {
        let mut ac = Accumulator::new();
        let mut gr = GeneralRegister::new();
        let data = Binary::from(0b101_u32);
        gr.load(data);
        ac = ac + &gr;
        ac = !ac;
        assert_eq!(Binary::from(!0b101_u64), ac.result);
    }

    #[test]
    fn sub() {
        let mut ac = Accumulator::new();
        let mut gr = GeneralRegister::new();
        gr.load(Binary::from(0b101_u32));
        ac = ac + &gr;
        gr.load(Binary::from(0b1010_u32));
        ac = ac - &gr;
        assert_eq!(Binary::from(0b101_u32), ac.result);
    }
}
