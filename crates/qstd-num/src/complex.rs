use core::fmt;

use crate::{
    consts::{one, zero},
    float::{DefaultFloat, Float},
    real::Real,
};

pub type Pair<T> = (T, T);

pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const UNIT: char = 'i';

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct Complex<F: Float = DefaultFloat> {
    pub re: Real<F>,
    pub im: Real<F>,
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

impl<F: Float> Complex<F> {
    pub const fn with(re: Real<F>, im: Real<F>) -> Self {
        Self { re, im }
    }

    pub const fn re(self) -> Real<F> {
        self.re
    }

    pub const fn im(self) -> Real<F> {
        self.im
    }

    pub const fn pair(self) -> Pair<Real<F>> {
        (self.re, self.im)
    }

    pub const fn get_re_no_assert(self) -> F {
        self.re().get_no_assert()
    }

    pub const fn get_im_no_assert(self) -> F {
        self.im().get_no_assert()
    }

    pub const fn get_no_assert(self) -> Pair<F> {
        (self.get_re_no_assert(), self.get_im_no_assert())
    }

    pub fn get_re(self) -> F {
        self.re().get()
    }

    pub fn get_im(self) -> F {
        self.im().get()
    }

    pub fn get(self) -> Pair<F> {
        (self.get_re(), self.get_im())
    }

    pub const ZERO: Self = Self::with(zero(), zero());
    pub const ONE: Self = Self::with(one(), zero());
    pub const I: Self = Self::with(zero(), one());
}
