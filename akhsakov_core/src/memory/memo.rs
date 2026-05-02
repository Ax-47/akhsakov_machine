use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    registers::{address_register::AddressRegister, general_register::GeneralRegister},
    types::Binary,
};

pub(crate) struct Memory {
    data: Vec<Binary>,
    ar: Rc<RefCell<AddressRegister>>,
}
impl Memory {
    pub(crate) fn new(ar: Rc<RefCell<AddressRegister>>) -> Self {
        Self {
            data: vec![Binary::zero(); 65536],
            ar,
        }
    }
    pub(crate) fn read(&self) -> Binary {
        self.data[self.ar.borrow().address.0 as usize]
    }

    pub(crate) fn write(&mut self, write: Binary) {
        self.data[self.ar.borrow().address.0 as usize] = write;
    }
}
