use crate::{registers::squence_counter::sequence::T0, types::Binary};
use num_traits::FromPrimitive;

#[derive(Clone, Copy)]
pub enum SequenceValue {
    T0,
    T1,
}
pub(crate) mod sequence {
    use concat_idents::concat_idents;
    macro_rules! create_whole_sequence {

        ($($x:expr),+) => {
            $(
            concat_idents!(StructName = "T", $x {
                #[derive(Clone, Copy)]
                pub(crate) struct StructName;
            });
            )*
        };

    }

    create_whole_sequence!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
}
#[derive(Clone, Copy)]
pub(crate) struct SquenceCounterRegister<T> {
    _marker: std::marker::PhantomData<T>,
}

impl SquenceCounterRegister<sequence::T0> {
    pub(crate) fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<T> SquenceCounterRegister<T> {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
    pub(crate) fn reset(self) -> SquenceCounterRegister<sequence::T0> {
        SquenceCounterRegister::default()
    }
}

macro_rules! create_increase {

    ($first: ty) =>{};
    ($first: ty,$second: ty) => {
        impl SquenceCounterRegister<$first> {
            pub(crate) fn increase(self) -> SquenceCounterRegister<$second> {
                SquenceCounterRegister::<$second>::new()
            }
        }
    };
    ($first: ty,$second: ty $(,$rest:ty)*) => {
        impl SquenceCounterRegister<$first> {
            pub(crate) fn increase(self) -> SquenceCounterRegister<$second> {
                SquenceCounterRegister::<$second>::new()
            }
        }
        create_increase!($second $(, $rest)*);
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
