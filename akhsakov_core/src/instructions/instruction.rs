use crate::types::{Address, Binary};

#[derive(Debug)]
pub(crate) enum Instruction {
    Null = 0,
    Move = 1,
}
impl Instruction {
    fn new(ins: u32) -> Self {
        match ins {
            1 => Self::Move,
            _ => Self::Null,
        }
    }
}

#[derive(Debug)]
pub(crate) struct IsDirect(bool);

#[derive(Debug)]
pub(crate) struct Word {
    pub(crate) is_direct: IsDirect,
    pub(crate) instruction: Instruction,
    pub(crate) address1: Address,
    pub(crate) address2: Address,
}
impl Word {
    pub(crate) fn new(
        is_direct: bool,
        instruction: Instruction,
        address1: Address,
        address2: Address,
    ) -> Self {
        Self {
            is_direct: IsDirect(is_direct),
            instruction,
            address1,
            address2,
        }
    }

    pub(crate) fn from(data: Binary) -> Self {
        let data = data.0;
        let mut ins: u64 = 0;
        for (idx, &bit) in data[1..=5].iter().rev().enumerate() {
            if bit {
                ins |= 1u64 << idx;
            }
        }
        let instruction = Instruction::new(ins as u32);

        let mut ins: u64 = 0;
        for (idx, &bit) in data[5..=21].iter().rev().enumerate() {
            if bit {
                ins |= 1u64 << idx;
            }
        }
        let address1 = Address::new(ins as u16);

        let mut ins: u64 = 0;
        for (idx, &bit) in data[21..=37].iter().rev().enumerate() {
            if bit {
                ins |= 1u64 << idx;
            }
        }
        let address2 = Address::new(ins as u16);

        Self {
            is_direct: IsDirect(data[0]),
            instruction,
            address1,
            address2,
        }
    }
    pub(crate) fn zero() -> Self {
        Self {
            is_direct: IsDirect(false),
            instruction: Instruction::Move,
            address1: Address::zero(),
            address2: Address::zero(),
        }
    }
}
