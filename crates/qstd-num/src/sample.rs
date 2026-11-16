#[cfg(feature = "rand")]
use rand::{Rng, RngExt};

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use crate::{
    consts::{Half, One, Zero, half, one, zero},
    float::{DefaultFloat, Extracted, Float},
    real::Real,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Probability<F: Float> {
    value: Real<F>,
}

impl<F: Float> Probability<F> {
    pub fn new(value: Real<F>) -> Option<Self> {
        let float = value.get();

        (float >= F::ZERO && float <= F::ONE).then(|| unsafe { Self::new_unchecked(value) })
    }

    pub fn clamped(value: Real<F>) -> Self {
        let float = value.get();

        if float < F::ZERO {
            return Self::ZERO;
        }

        if float > F::ONE {
            return Self::ONE;
        }

        unsafe { Self::new_unchecked(value) }
    }

    pub const unsafe fn new_unchecked(value: Real<F>) -> Self {
        Self { value }
    }

    pub const fn value(self) -> Real<F> {
        self.value
    }

    pub const fn get(self) -> F {
        self.value().get()
    }
}

unsafe impl<F: Float> Zero for Probability<F> {
    const ZERO: Self = unsafe { Self::new_unchecked(zero()) };
}

unsafe impl<F: Float> One for Probability<F> {
    const ONE: Self = unsafe { Self::new_unchecked(one()) };
}

unsafe impl<F: Float> Half for Probability<F> {
    const HALF: Self = unsafe { Self::new_unchecked(half()) };
}

pub trait Sample<F: Float = DefaultFloat> {
    fn sample(&mut self, p: Probability<F>) -> bool;
}

pub const fn is_probability(p: Extracted) -> bool {
    p >= zero() && p <= one()
}

#[cfg(feature = "rand")]
impl<R: Rng + ?Sized, F: Float> Sample<F> for R {
    fn sample(&mut self, probability: Probability<F>) -> bool {
        let p = probability.get().extract();

        #[cfg(feature = "unsafe-assert")]
        unsafe {
            assert_unchecked(is_probability(p));
        }

        self.random_bool(p)
    }
}
