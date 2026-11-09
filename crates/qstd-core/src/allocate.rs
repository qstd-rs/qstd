use core::array;

use crate::{
    aliases::Size,
    machine::{Machine, Register},
};

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{machine::VecRegister, vec};

pub trait Allocate: Machine {
    fn qubit(&mut self) -> Self::Qubit;

    fn register<const N: Size>(&mut self) -> Register<Self, N> {
        array::from_fn(|_| self.qubit())
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn vec_register(&mut self, size: Size) -> VecRegister<Self> {
        vec::from_fn(|_| self.qubit(), size)
    }
}
