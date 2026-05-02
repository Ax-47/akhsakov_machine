use crate::{componants::componant::Loader, instructions::instruction::Word, types::Binary};

pub(crate) struct InstructionRegister {
    pub(crate) instruction: Word, // TODO: instruction
}
impl InstructionRegister {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            instruction: Word::zero(),
        }
    }
    pub(crate) fn load(&mut self, instruction: Word) {
        self.instruction = instruction;
    }
}
