use core::{f32::consts as const_f32, f64::consts as const_f64};

pub unsafe trait Zero {
    const ZERO: Self;
}

pub unsafe trait One {
    const ONE: Self;
}

pub unsafe trait Const {
    const FRAC_1_SQRT_2: Self;
}

pub const fn zero<T: Zero>() -> T {
    T::ZERO
}

pub const fn one<T: One>() -> T {
    T::ONE
}

pub const fn frac_1_sqrt_2<T: Const>() -> T {
    T::FRAC_1_SQRT_2
}

unsafe impl Zero for f32 {
    const ZERO: Self = 0.0;
}

unsafe impl Zero for f64 {
    const ZERO: Self = 0.0;
}

unsafe impl One for f32 {
    const ONE: Self = 1.0;
}

unsafe impl One for f64 {
    const ONE: Self = 1.0;
}

unsafe impl Const for f32 {
    const FRAC_1_SQRT_2: Self = const_f32::FRAC_1_SQRT_2;
}

unsafe impl Const for f64 {
    const FRAC_1_SQRT_2: Self = const_f64::FRAC_1_SQRT_2;
}
