use crate::{
    aliases::{Array, Dyn, Size},
    bits::Bit,
};

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::vec::Vec;

pub trait Machine {
    type Qubit;

    type Bit: Bit;
}

pub type Register<M, const N: Size> = Array<<M as Machine>::Qubit, N>;

pub type DynRegister<M> = Dyn<<M as Machine>::Qubit>;

#[cfg(any(feature = "std", feature = "alloc"))]
pub type VecRegister<M> = Vec<<M as Machine>::Qubit>;

pub type Measurements<M, const N: Size> = Array<<M as Machine>::Bit, N>;

pub type DynMeasurements<M> = Dyn<<M as Machine>::Bit>;

#[cfg(any(feature = "std", feature = "alloc"))]
pub type VecMeasurements<M> = Vec<<M as Machine>::Bit>;
