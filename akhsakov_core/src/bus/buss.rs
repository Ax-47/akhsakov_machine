use std::cell::RefCell;
use std::rc::Rc;

use crate::instructions::instruction::Word;
use crate::memory::memo::Memory;
use crate::registers::address_register::AddressRegister;
use crate::registers::instruction_register::InstructionRegister;
use crate::registers::squence_counter::{SquenceCounterRegister, sequence};
use crate::{
    arithmetic_logic_unit::accumulator::Accumulator, registers::general_register::GeneralRegister,
};

pub(crate) struct Bus<T> {
    pub(crate) g_rax: [GeneralRegister; 15],
    pub(crate) ac: [Accumulator; 2],
    pub(crate) ins_r: InstructionRegister,
    pub(crate) pc: AddressRegister,
    pub(crate) sc: SquenceCounterRegister<T>,
    pub(crate) ar: Rc<RefCell<AddressRegister>>,
    pub(crate) memory: Memory,
}

impl<T> Bus<T> {
    pub(crate) fn new(
        g_rax: [GeneralRegister; 15],
        ac: [Accumulator; 2],
        ins_r: InstructionRegister,
        pc: AddressRegister,
        sc: SquenceCounterRegister<T>,
        ar: Rc<RefCell<AddressRegister>>,
        memory: Memory,
    ) -> Self {
        Self {
            g_rax,
            ac,
            ins_r,
            pc,
            sc,
            memory,
            ar,
        }
    }
    pub(crate) fn reset(self) -> Bus<sequence::T0> {
        Bus::<sequence::T0>::new(
            self.g_rax,
            self.ac,
            self.ins_r,
            self.pc,
            self.sc.reset(),
            self.ar,
            self.memory,
        )
    }
}
impl<T0> Bus<T0> {
    pub(crate) fn default() -> Self {
        let ar = Rc::new(RefCell::new(AddressRegister::new()));
        Self {
            g_rax: [GeneralRegister::new(); 15],
            ac: [Accumulator::new(); 2],
            ins_r: InstructionRegister::new(),
            pc: AddressRegister::new(),
            sc: SquenceCounterRegister::new(),
            memory: Memory::new(Rc::clone(&ar)),
            ar,
        }
    }
}

impl<T1> Bus<T1> {}
#[cfg(test)]
mod tests {
    use crate::bus::increase::Increase;
    use crate::{bus::execute::Execute, registers::squence_counter::sequence::*, types::Binary};

    use super::*;

    #[test]
    fn test_bus_init() {
        let bus = Bus::<T0>::default();
        assert_eq!(bus.g_rax.len(), 15);
        assert_eq!(bus.ac.len(), 2);
    }

    #[test]
    fn test_memory_read_write() {
        let mut bus = Bus::<T0>::default();
        let value = Binary::from(67);
        bus.ar.borrow_mut().load(Binary::from(10).into());
        bus.memory.write(value);
        let read = bus.memory.read();
        assert_eq!(read.to_int(), value.to_int());
    }

    #[test]
    fn test_execute() {
        let mut bus = Bus::<T0>::default();

        let value = Binary::from(67);
        bus.ar.borrow_mut().load(Binary::from(1).into());
        bus.memory.write(value);

        let value = Binary::from(69);
        bus.ar.borrow_mut().load(Binary::from(2).into());
        bus.memory.write(value);
        //set programe;
        //0xA; MOV [0x1], [0x2]
        let value = Binary::from(0b0_00001_0000000000000001_0000000000000010u64);
        bus.ar.borrow_mut().load(Binary::from(10).into());
        bus.memory.write(value);

        // Start: 0xA
        bus.pc.load(Binary::from(10).into());

        // fetch
        bus.execute();
        let mut bus = bus.next_step();
        // IR <- *AR, PC++
        bus.execute();
        let mut bus = bus.next_step();
        // Decode
        bus.execute();
        let mut bus = bus.next_step();
        // Is it indirect
        bus.execute();
        let mut bus = bus.next_step();
        bus.execute();
        let bus = bus.reset();

        bus.ar.borrow_mut().load(Binary::from(1).into());
        let read_from_1 = bus.memory.read();
        println!("{}", read_from_1);
        bus.ar.borrow_mut().load(Binary::from(2).into());
        let read_from_2 = bus.memory.read();
        println!("{}", read_from_2);
        assert_eq!(read_from_1, read_from_2);
    }
}
