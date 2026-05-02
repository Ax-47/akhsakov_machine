use crate::{componants::componant::Componant, types::Binary};

#[derive(Debug, Copy, Clone)]
pub(crate) struct GeneralRegister {
    pub(crate) data: Binary,
}
impl GeneralRegister {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            data: Binary::zero(),
        }
    }
}

impl Componant for GeneralRegister {
    fn load(&mut self, data: Binary) {
        self.data = data;
    }
    fn clear(&mut self) {
        self.data = Binary::zero();
    }
}
