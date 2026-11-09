use crate::{
    aliases::Size,
    by::By,
    machine::{Machine, Measurements, Register},
    measure::Measure,
};

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::machine::{VecMeasurements, VecRegister};

pub trait Release: Machine {
    fn release(&mut self, qubit: Self::Qubit);

    fn release_n<const N: Size>(&mut self, register: Register<Self, N>);

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn release_vec(&mut self, register: VecRegister<Self>);

    fn take(&mut self, mut qubit: Self::Qubit) -> Self::Bit
    where
        Self: Measure,
    {
        let bit = self.measure(qubit.by_mut());

        self.release(qubit);

        bit
    }

    fn take_n<const N: Size>(&mut self, mut register: Register<Self, N>) -> Measurements<Self, N>
    where
        Self: Measure,
    {
        let measurements = self.measure_n(register.by_mut());

        self.release_n(register);

        measurements
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn take_vec(&mut self, mut register: VecRegister<Self>) -> VecMeasurements<Self>
    where
        Self: Measure,
    {
        let measurements = self.measure_dyn(register.by_mut());

        self.release_vec(register);

        measurements
    }
}
