use crate::{
    aliases::Size,
    arrays::ArrayMeasure,
    machine::{Machine, Measurements, Register},
};

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{
    machine::{DynRegister, VecMeasurements},
    vec::DynMeasure,
};

pub trait Measure: Machine {
    fn measure(&mut self, qubit: &mut Self::Qubit) -> Self::Bit;

    fn measure_into(&mut self, qubit: &mut Self::Qubit, bit: &mut Self::Bit) {
        *bit = self.measure(qubit);
    }

    fn measure_n<const N: Size>(
        &mut self,
        register: &mut Register<Self, N>,
    ) -> Measurements<Self, N> {
        register.measure_with(|qubit| self.measure(qubit))
    }

    fn measure_into_n<const N: Size>(
        &mut self,
        register: &mut Register<Self, N>,
        measurements: &mut Measurements<Self, N>,
    ) {
        *measurements = self.measure_n(register);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn measure_dyn(&mut self, register: &mut DynRegister<Self>) -> VecMeasurements<Self> {
        register.measure_with(|qubit| self.measure(qubit))
    }
}
