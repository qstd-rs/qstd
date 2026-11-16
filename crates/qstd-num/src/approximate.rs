use crate::{
    consts::{compare_epsilon, unitary_epsilon},
    float::Float,
    tolerance::{ExtractTolerance, Tolerance},
};

pub trait ApproxEq<R = Self> {
    type Epsilon: Float;

    fn approx_eq(self, other: R, tolerance: Tolerance<Self::Epsilon>) -> bool;
}

pub type ExtractEpsilon<L, R> = <L as ApproxEq<R>>::Epsilon;

impl ApproxEq for f32 {
    type Epsilon = f32;

    fn approx_eq(self, other: Self, tolerance: Tolerance<Self::Epsilon>) -> bool {
        (self - other).abs() <= tolerance.get()
    }
}

impl ApproxEq for f64 {
    type Epsilon = f64;

    fn approx_eq(self, other: Self, tolerance: Tolerance<Self::Epsilon>) -> bool {
        (self - other).abs() <= tolerance.get()
    }
}

pub fn compare_approx_eq<L, R>(left: L, right: R) -> bool
where
    L: ApproxEq<R>,
{
    compare_tolerance::<L, R>().approx_eq(left, right)
}

pub fn unitary_approx_eq<L, R>(left: L, right: R) -> bool
where
    L: ApproxEq<R>,
{
    unitary_tolerance::<L, R>().approx_eq(left, right)
}

pub const fn compare_tolerance<L, R>() -> ExtractTolerance<L, R>
where
    L: ApproxEq<R>,
{
    compare_epsilon()
}

pub const fn unitary_tolerance<L, R>() -> ExtractTolerance<L, R>
where
    L: ApproxEq<R>,
{
    unitary_epsilon()
}
