use crate::{
    bus::buss::Bus, registers::squence_counter::SquenceCounterRegister,
    registers::squence_counter::sequence,
};

pub(crate) trait Increase<T> {
    type Next;
    fn next_step(self) -> Self::Next;
}

macro_rules! create_increase {

    ($t:ty  ) => { };
    ($t:ty ,$nt:ty ) => {
        impl Increase<$t> for Bus<$t> {
            type Next = Bus<$nt>;
            fn next_step(self) -> Self::Next {
                Bus::<$nt>::new(
                    self.g_rax,
                    self.ac,
                    self.ins_r,
                    self.pc,
                    self.sc.increase(),
                    self.ar,
                    self.memory,
                )
            }
        }
    };
    ($t:ty ,$nt:ty $(, $rest:ty)*) => {
        impl Increase<$t> for Bus<$t> {
            type Next = Bus<$nt>;
            fn next_step(self) -> Self::Next {
                Bus::<$nt>::new(
                    self.g_rax,
                    self.ac,
                    self.ins_r,
                    self.pc,
                    self.sc.increase(),
                    self.ar,
                    self.memory,
                )
            }
        }
        create_increase!($nt $(, $rest)*);
    };
}
create_increase! {
    sequence::T0,
    sequence::T1,
    sequence::T2,
    sequence::T3,
    sequence::T4,
    sequence::T5,
    sequence::T6,
    sequence::T7,
    sequence::T8,
    sequence::T9,
    sequence::T10,
    sequence::T11,
    sequence::T12,
    sequence::T13,
    sequence::T14,
    sequence::T15
}
