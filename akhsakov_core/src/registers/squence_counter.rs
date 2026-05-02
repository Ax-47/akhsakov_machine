use crate::types::Binary;

pub(crate) struct SquenceCounterRegister {
    pub(crate) squence: u32, // TODO: instruction
}
impl SquenceCounterRegister {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self { squence: 0 }
    }

    pub(crate) fn incease(&mut self) {
        self.squence += 1;
    }
}
