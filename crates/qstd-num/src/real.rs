use core::ops::Neg;

use crate::{
    approximate::ApproxEq,
    consts::{Const, Epsilon, Half, NegativeOne, One, Two, Zero},
    float::{DefaultFloat, Float},
    tolerance::Tolerance,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Real<F: Float = DefaultFloat> {
    float: F,
}

impl<F: Float> ApproxEq for Real<F> {
    type Epsilon = F;

    fn approx_eq(self, other: Self, tolerance: Tolerance<F>) -> bool {
        self.get().approx_eq(other.get(), tolerance)
    }
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

    pub const fn get(self) -> F {
        self.float
    }

    pub fn half(self) -> Self {
        let float = self.get();

        let half = float / F::TWO;

        unsafe { Self::new_unchecked(half) }
    }
}

impl<F: Float> Neg for Real<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let float = self.get();

        let negated = -float;

        unsafe { Self::new_unchecked(negated) }
    }
}

unsafe impl<F: Float> Zero for Real<F> {
    const ZERO: Self = unsafe { Self::new_unchecked(F::ZERO) };
}

unsafe impl<F: Float> One for Real<F> {
    const ONE: Self = unsafe { Self::new_unchecked(F::ONE) };
}

unsafe impl<F: Float> Half for Real<F> {
    const HALF: Self = unsafe { Self::new_unchecked(F::HALF) };
}

unsafe impl<F: Float> NegativeOne for Real<F> {
    const NEGATIVE_ONE: Self = unsafe { Self::new_unchecked(F::NEGATIVE_ONE) };
}

unsafe impl<F: Float> Two for Real<F> {
    const TWO: Self = unsafe { Self::new_unchecked(F::TWO) };
}

unsafe impl<F: Float> Epsilon for Real<F> {
    const COMPARE: Self = unsafe { Self::new_unchecked(F::COMPARE) };
    const UNITARY: Self = unsafe { Self::new_unchecked(F::UNITARY) };
}

unsafe impl<F: Float> Const for Real<F> {
    const FRAC_1_SQRT_2: Self = unsafe { Self::new_unchecked(F::FRAC_1_SQRT_2) };
    const FRAC_PI_4: Self = unsafe { Self::new_unchecked(F::FRAC_PI_4) };
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
