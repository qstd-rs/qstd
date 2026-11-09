#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use crate::{
    consts::{Const, One, Zero},
    float::{DefaultFloat, Float},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub struct Real<F: Float = DefaultFloat> {
    float: F,
}

pub type DefaultReal = Real<DefaultFloat>;

impl<F: Float> Real<F> {
    pub fn new(float: F) -> Option<Self> {
        float
            .is_finite()
            .then(|| unsafe { Self::new_unchecked(float) })
    }

    pub const unsafe fn new_unchecked(float: F) -> Self {
        Self { float }
    }

    pub const fn get_no_assert(self) -> F {
        self.float
    }

    pub fn get(self) -> F {
        let float = self.get_no_assert();

        #[cfg(feature = "unsafe-assert")]
        assert_unchecked(float.is_finite());

        float
    }
}

unsafe impl<F: Float> Zero for Real<F> {
    const ZERO: Self = unsafe { Self::new_unchecked(F::ZERO) };
}

unsafe impl<F: Float> One for Real<F> {
    const ONE: Self = unsafe { Self::new_unchecked(F::ONE) };
}

unsafe impl<F: Float> Const for Real<F> {
    const FRAC_1_SQRT_2: Self = unsafe { Self::new_unchecked(F::FRAC_1_SQRT_2) };
}

impl Real<f32> {
    pub const fn f32(float: f32) -> Option<Self> {
        if float.is_finite() {
            let real = unsafe { Self::new_unchecked(float) };

            Some(real)
        } else {
            None
        }
    }
}

impl Real<f64> {
    pub const fn f64(float: f64) -> Option<Self> {
        if float.is_finite() {
            let real = unsafe { Self::new_unchecked(float) };

            Some(real)
        } else {
            None
        }
    }
}
