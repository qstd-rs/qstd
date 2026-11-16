use core::{f32::consts as const_f32, f64::consts as const_f64};

pub unsafe trait Zero {
    const ZERO: Self;
}

pub unsafe trait One {
    const ONE: Self;
}

pub unsafe trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

pub unsafe trait Two {
    const TWO: Self;
}

pub unsafe trait Half {
    const HALF: Self;
}

pub unsafe trait Const {
    const FRAC_1_SQRT_2: Self;

    const FRAC_PI_4: Self;
}

pub unsafe trait Epsilon {
    const COMPARE: Self;
    const UNITARY: Self;
}

pub const fn zero<T: Zero>() -> T {
    T::ZERO
}

pub const fn one<T: One>() -> T {
    T::ONE
}

pub const fn two<T: Two>() -> T {
    T::TWO
}

pub const fn negative_one<T: NegativeOne>() -> T {
    T::NEGATIVE_ONE
}

pub const fn half<T: Half>() -> T {
    T::HALF
}

pub const fn frac_1_sqrt_2<T: Const>() -> T {
    T::FRAC_1_SQRT_2
}

pub const fn frac_pi_4<T: Const>() -> T {
    T::FRAC_PI_4
}

pub const fn compare_epsilon<T: Epsilon>() -> T {
    T::COMPARE
}

pub const fn unitary_epsilon<T: Epsilon>() -> T {
    T::UNITARY
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

unsafe impl NegativeOne for f32 {
    const NEGATIVE_ONE: Self = -1.0;
}

unsafe impl NegativeOne for f64 {
    const NEGATIVE_ONE: Self = -1.0;
}

unsafe impl Two for f32 {
    const TWO: Self = 2.0;
}

unsafe impl Two for f64 {
    const TWO: Self = 2.0;
}

unsafe impl Half for f32 {
    const HALF: Self = 0.5;
}

unsafe impl Half for f64 {
    const HALF: Self = 0.5;
}

unsafe impl Const for f32 {
    const FRAC_1_SQRT_2: Self = const_f32::FRAC_1_SQRT_2;
    const FRAC_PI_4: Self = const_f32::FRAC_PI_4;
}

unsafe impl Const for f64 {
    const FRAC_1_SQRT_2: Self = const_f64::FRAC_1_SQRT_2;
    const FRAC_PI_4: Self = const_f64::FRAC_PI_4;
}

// XXX: adjust these if deemed appropriate

unsafe impl Epsilon for f32 {
    const COMPARE: Self = 1e-4;
    const UNITARY: Self = 1e-5;
}

unsafe impl Epsilon for f64 {
    const COMPARE: Self = 1e-10;
    const UNITARY: Self = 1e-12;
}
