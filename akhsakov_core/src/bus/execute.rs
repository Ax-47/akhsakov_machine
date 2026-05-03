use crate::{
    bus::{buss::Bus, increase::Increase},
    instructions::instruction::{Instruction, Word},
    registers::squence_counter::sequence::*,
};

pub(crate) trait Execute<T> {
    fn execute(&mut self);
}

impl Execute<T1> for Bus<T0> {
    fn execute(&mut self) {
        self.ar.borrow_mut().load(self.pc.address);
        self.sc.increase();
    }
}

impl Execute<T1> for Bus<T1> {
    fn execute(&mut self) {
        let data = self.memory.read();
        self.ins_r.load(data);
        self.pc.increase();
        self.sc.increase();
    }
}

impl Execute<T2> for Bus<T2> {
    fn execute(&mut self) {
        self.ins_r.decode();
        self.sc.increase();
    }
}

impl Execute<T3> for Bus<T3> {
    fn execute(&mut self) {
        // ...
        self.sc.increase();
    }
}

impl Execute<T4> for Bus<T4> {
    fn execute(&mut self) {
        // ...
        match self.ins_r.execute() {
            Instruction::Null => {}
            Instruction::Move => {
                self.ar.borrow_mut().load(self.ins_r.instruction.address2);
                let data = self.memory.read();
                self.ar.borrow_mut().load(self.ins_r.instruction.address1);
                self.memory.write(data);
            }
            Instruction::Add => {}
            Instruction::Sub => {}
            Instruction::Jmp => {}
        }
        self.sc.increase();
    }
}
