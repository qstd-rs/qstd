use crate::{
    approximate::{ApproxEq, ExtractEpsilon},
    consts::{Epsilon, compare_epsilon, unitary_epsilon},
    float::{DefaultFloat, Float},
    real::Real,
};

#[derive(Debug, Clone, Copy)]
pub struct Tolerance<F: Float = DefaultFloat> {
    epsilon: Real<F>,
}

pub type ExtractTolerance<L, R> = Tolerance<ExtractEpsilon<L, R>>;

pub type DefaultTolerance = Tolerance<DefaultFloat>;

unsafe impl<F: Float> Epsilon for Tolerance<F> {
    const COMPARE: Self = unsafe { Self::new_unchecked(compare_epsilon()) };
    const UNITARY: Self = unsafe { Self::new_unchecked(unitary_epsilon()) };
}

impl<F: Float> Tolerance<F> {
    pub fn new(epsilon: Real<F>) -> Option<Self> {
        (epsilon.get() > F::ZERO).then(|| unsafe { Self::new_unchecked(epsilon) })
    }

    pub const unsafe fn new_unchecked(epsilon: Real<F>) -> Self {
        Self { epsilon }
    }

    pub const fn epsilon(self) -> Real<F> {
        self.epsilon
    }

    pub const fn get(self) -> F {
        self.epsilon().get()
    }

    pub fn epsilon_squared(self) -> F {
        let epsilon = self.get();

        epsilon * epsilon
    }

    pub fn approx_eq<L, R>(self, left: L, right: R) -> bool
    where
        L: ApproxEq<R, Epsilon = F>,
    {
        left.approx_eq(right, self)
    }
}
