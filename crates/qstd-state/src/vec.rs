use qstd_brand::brands::{Brand, DefaultBrand};
use qstd_core::{aliases::Size, machine::Machine};
use qstd_num::{
    complex::Complex,
    float::{DefaultFloat, Float},
};

use crate::qubits::Qubit;

#[derive(Debug, Clone)]
pub struct StateVec<B: Brand = DefaultBrand, F: Float = DefaultFloat> {
    amplitudes: Vec<Complex<F>>,
    count: Size,
    brand: B,
}

pub type DefaultStateVec = StateVec<DefaultBrand, DefaultFloat>;

impl<B: Brand, F: Float> Machine for StateVec<B, F> {
    type Qubit = Qubit<B>;
    type Bit = bool;
}
