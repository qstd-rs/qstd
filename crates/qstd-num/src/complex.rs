use core::{
    fmt,
    ops::{Add, Sub},
};

use crate::{
    approximate::ApproxEq,
    float::{DefaultFloat, Float},
    tolerance::Tolerance,
};

pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const UNIT: char = 'i';

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Complex<F: Float = DefaultFloat> {
    pub re: F,
    pub im: F,
}

impl<F: Float> ApproxEq for Complex<F> {
    type Epsilon = F;

    fn approx_eq(self, other: Self, tolerance: Tolerance<Self::Epsilon>) -> bool {
        (self - other).norm_squared() <= tolerance.epsilon_squared()
    }
}

pub type DefaultComplex = Complex<DefaultFloat>;

fn display<T: fmt::Display>(value: T, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    value.fmt(formatter)
}

fn sign_abs<F: Float>(x: F) -> (char, F) {
    if x.is_sign_negative() {
        (MINUS, -x)
    } else {
        (PLUS, x)
    }
}

impl<F: Float> fmt::Display for Complex<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (re, im) = self.get();

        let (sign, abs) = sign_abs(im);

        display(re, formatter)?;

        write!(formatter, "{sign}")?;

        display(abs, formatter)?;

        write!(formatter, "{UNIT}")?;

        Ok(())
    }
}

pub const fn zero<F: Float>() -> Complex<F> {
    Complex::new(F::ZERO, F::ZERO)
}

pub const fn one<F: Float>() -> Complex<F> {
    Complex::new(F::ONE, F::ZERO)
}

pub const fn i<F: Float>() -> Complex<F> {
    Complex::new(F::ZERO, F::ONE)
}

pub const fn minus_i<F: Float>() -> Complex<F> {
    Complex::new(F::ZERO, F::NEGATIVE_ONE)
}

impl<F: Float> Complex<F> {
    pub const fn new(re: F, im: F) -> Self {
        Self { re, im }
    }

    pub const fn re(self) -> F {
        self.re
    }

    pub const fn im(self) -> F {
        self.im
    }

    pub const fn set_re(&mut self, re: F) {
        self.re = re;
    }

    pub const fn set_im(&mut self, im: F) {
        self.im = im;
    }

    pub const fn get_mut(&mut self) -> (&mut F, &mut F) {
        (&mut self.re, &mut self.im)
    }

    pub const fn get(self) -> (F, F) {
        (self.re(), self.im())
    }

    pub fn conjugated(self) -> Self {
        let (re, im) = self.get();

        Self::new(re, -im)
    }

    pub fn conjugate(&mut self) {
        *self = self.conjugated();
    }

    pub fn scaled(self, factor: F) -> Self {
        let (re, im) = self.get();

        Self::new(re * factor, im * factor)
    }

    pub fn norm_squared(self) -> F {
        let (re, im) = self.get();

        re * re + im * im
    }

    pub fn scale(&mut self, factor: F) {
        *self = self.scaled(factor);
    }

    #[cfg(feature = "std")]
    pub fn norm(self) -> F {
        self.norm_squared().sqrt()
    }

    pub const ZERO: Self = zero();
    pub const ONE: Self = one();
    pub const I: Self = i();
    pub const MINUS_I: Self = minus_i();
}

impl<F: Float> Add for Complex<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = self.get();
        let (c, d) = other.get();

        Self::Output::new(a + c, b + d)
    }
}

impl<F: Float> Sub for Complex<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = self.get();
        let (c, d) = other.get();

        Self::Output::new(a - c, b - d)
    }
}
