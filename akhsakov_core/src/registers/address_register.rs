use crate::{
    componants::componant::Componant,
    types::{Address, Binary},
};

pub(crate) struct AddressRegister {
    pub(crate) address: Address, // TODO: Address
}
impl AddressRegister {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            address: Address::zero(),
        }
    }
    pub(crate) fn load(&mut self, address: Address) {
        self.address = address;
    }
    pub(crate) fn clear(&mut self) {
        self.address = Address::zero();
    }
}
