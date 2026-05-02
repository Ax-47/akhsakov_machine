use std::fmt::{Debug, Display};

use num_traits::ToPrimitive;

use crate::types::Address;
#[derive(Copy, Clone)]
pub(crate) struct Binary(pub(crate) [bool; 38]);
impl Binary {
    pub(crate) fn zero() -> Self {
        Self([false; 38])
    }

    pub fn to_int(&self) -> u64 {
        let mut int: u64 = 0;
        for (idx, &bit) in self.0.iter().rev().enumerate() {
            if bit {
                int |= 1u64 << idx;
            }
        }
        int
    }
}
impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for bit in self.0 {
            str.push(if bit { '1' } else { '0' });
        }
        write!(f, "0b{}", str)
    }
}
impl Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(38);

        for &bit in self.0.iter() {
            s.push(if bit { '1' } else { '0' });
        }
        write!(f, "0b{}", s)
    }
}

impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0).all(|(a, b)| *a == b)
    }
}
impl<T> From<T> for Binary
where
    T: num_traits::PrimInt,
{
    fn from(value: T) -> Self {
        let mut data = [false; 38];
        let bits = std::mem::size_of::<T>() * 8;
        for (idx, bit) in data.iter_mut().enumerate().take(bits) {
            *bit = (value >> idx) & T::one() == T::one();
        }
        data.reverse();
        Self(data)
    }
}

impl Into<Address> for Binary {
    fn into(self) -> Address {
        let adr = self.to_int().to_u16().unwrap_or_default();
        Address(adr)
    }
}
