use crate::{
    instructions::instruction::{Instruction, Word},
    types::Binary,
};

pub(crate) struct InstructionRegister {
    pub(crate) encoded_instruction: Binary, // TODO: instruction
    pub(crate) instruction: Word,           // TODO: instruction
}
impl InstructionRegister {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            encoded_instruction: Binary::zero(),
            instruction: Word::zero(),
        }
    }
    pub(crate) fn load(&mut self, encoded_instruction: Binary) {
        self.encoded_instruction = encoded_instruction;
    }

    pub(crate) fn decode(&mut self) {
        self.instruction = Word::from(self.encoded_instruction);
    }
    pub(crate) fn execute(&self) -> Instruction {
        self.instruction.instruction
    }
}
