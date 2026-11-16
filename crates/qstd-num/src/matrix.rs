use crate::{
    complex::{Complex, one, zero},
    float::{DefaultFloat, Float},
};

pub type Size = usize;

pub type Array<T, const N: Size> = [T; N];

pub const SIZE: Size = 2;

pub type MatrixArray<T> = Array<Array<T, SIZE>, SIZE>;

pub type Quadruple<T> = (T, T, T, T);

/// ```text
/// [[a, b],
///  [c, d]]
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Matrix<F: Float = DefaultFloat> {
    pub a: Complex<F>,
    pub b: Complex<F>,
    pub c: Complex<F>,
    pub d: Complex<F>,
}

impl<F: Float> Matrix<F> {
    pub const fn new(a: Complex<F>, b: Complex<F>, c: Complex<F>, d: Complex<F>) -> Self {
        Self { a, b, c, d }
    }

    pub const fn get_mut(&mut self) -> Quadruple<&mut Complex<F>> {
        let Self { a, b, c, d } = self;

        (a, b, c, d)
    }

    pub const fn get(self) -> Quadruple<Complex<F>> {
        let Self { a, b, c, d } = self;

        (a, b, c, d)
    }

    pub const fn array(self) -> MatrixArray<Complex<F>> {
        let (a, b, c, d) = self.get();

        [[a, b], [c, d]]
    }

    pub fn adjoint(self) -> Self {
        let (a, b, c, d) = self.get();

        Self::new(
            a.conjugated(),
            c.conjugated(),
            b.conjugated(),
            d.conjugated(),
        )
    }

    pub fn adjoint_mut(&mut self) {
        *self = self.adjoint();
    }

    pub const IDENTITY: Self = Self::new(one(), zero(), zero(), one());
}
