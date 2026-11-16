use qstd_brand::brands::{Brand, DefaultBrand};
use qstd_core::{aliases::Size, machine::Machine};
use qstd_num::{
    complex::Complex,
    float::{DefaultFloat, Float},
    sample::Sample,
};

use crate::qubits::Qubit;

#[derive(Debug)]
pub struct StateVec<R: Sample<F>, B: Brand = DefaultBrand, F: Float = DefaultFloat> {
    amplitudes: Vec<Complex<F>>,
    count: Size,
    brand: B,
    sample: R,
}

pub type DefaultStateVec = StateVec<DefaultBrand, DefaultFloat>;

impl<R: Sample<F>, B: Brand, F: Float> Machine for StateVec<B, F> {
    type Qubit = Qubit<B>;
    type Bit = bool;
}
