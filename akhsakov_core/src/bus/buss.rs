use std::cell::RefCell;
use std::rc::Rc;

use crate::componants::componant::Componant;
use crate::instructions::instruction::{Instruction, Word};
use crate::memory::memo::Memory;
use crate::registers::address_register::AddressRegister;
use crate::registers::instruction_register::InstructionRegister;
use crate::registers::squence_counter::SquenceCounterRegister;
use crate::types::Address;
use crate::{
    arithmetic_logic_unit::accumulator::Accumulator, registers::general_register::GeneralRegister,
};

pub(crate) struct Bus {
    g_rax: [GeneralRegister; 15],
    ac: [Accumulator; 2],
    ins_r: InstructionRegister,
    pc: AddressRegister,
    sc: SquenceCounterRegister,
    ar: Rc<RefCell<AddressRegister>>,

    memory: Memory,
}
impl Bus {
    pub(crate) fn new() -> Self {
        let ar = Rc::new(RefCell::new(AddressRegister::new()));
        return Self {
            g_rax: [GeneralRegister::new(); 15],
            ac: [Accumulator::new(); 2],
            ins_r: InstructionRegister::new(),
            pc: AddressRegister::new(),
            sc: SquenceCounterRegister::new(),
            memory: Memory::new(Rc::clone(&ar)),
            ar,
        };
    }
    pub(crate) fn fetch(&mut self) {
        self.ar.borrow_mut().load(self.pc.address);
        self.sc.incease();
    }

    pub(crate) fn decode(&mut self) {
        let data = self.memory.read();
        let word = Word::from(data);
        self.ins_r.load(word);
        self.sc.incease();
    }
    pub(crate) fn execute(&mut self) {
        self.sc.incease();
    }
}
#[cfg(test)]
mod tests {
    use crate::types::Binary;

    use super::*;

    #[test]
    fn test_bus_init() {
        let bus = Bus::new();
        assert_eq!(bus.g_rax.len(), 15);
        assert_eq!(bus.ac.len(), 2);
    }

    #[test]
    fn test_memory_read_write() {
        let mut bus = Bus::new();
        let value = Binary::from(67);
        bus.ar.borrow_mut().load(Binary::from(10).into());
        bus.memory.write(value);
        let read = bus.memory.read();
        assert_eq!(read.to_int(), value.to_int());
    }

    #[test]
    fn test_execute() {
        let mut bus = Bus::new();
        let value = Binary::from(67);

        bus.ar.borrow_mut().load(Binary::from(10).into());

        bus.memory.write(value);
        let read = bus.memory.read();

        assert_eq!(read.to_int(), value.to_int());
    }
}
