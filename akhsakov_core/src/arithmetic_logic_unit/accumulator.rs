use crate::{componants::componant::Componant, types::Binary};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Accumulator {
    pub(crate) result: Binary,
}

impl Accumulator {
    pub(crate) fn new() -> Self {
        Self {
            result: Binary::zero(),
        }
    }
}

impl Componant for Accumulator {
    fn load(&mut self, data: Binary) {
        self.result = data;
    }
    fn clear(&mut self) {
        self.result = Binary::zero();
    }
}
