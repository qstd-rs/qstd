use std::{fmt, thread::panicking};

use qstd_brand::brands::{Brand, Branded, DefaultBrand};
use qstd_core::{aliases::Index, by::By};

use crate::released::{is_active, is_released};

pub const QUBIT: &str = "Qubit";
pub const Q: char = 'q';

pub const RELEASED: Index = Index::MAX;

#[must_use = "dropped qubits stay allocated (and possibly entangled)"]
pub struct Qubit<B: Brand = DefaultBrand> {
    index: Index,
    brand: B,
}

impl<B: Brand> fmt::Debug for Qubit<B> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple(QUBIT)
            .field(self.index().by_ref())
            .finish()
    }
}

impl<B: Brand> fmt::Display for Qubit<B> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{Q}{index}", index = self.index())
    }
}

pub fn not_panicking() -> bool {
    !panicking()
}

impl<B: Brand> Drop for Qubit<B> {
    fn drop(&mut self) {
        assert!(
            panicking() || self.is_released(),
            "implicitly dropped qubit {self}"
        )
    }
}

impl<B: Brand> Branded for Qubit<B> {
    type Brand = B;

    fn brand(&self) -> B {
        self.brand
    }
}

impl<B: Brand> Qubit<B> {
    pub const unsafe fn try_new(index: Index, brand: B) -> Option<Self> {
        if is_released(index) {
            None
        } else {
            let qubit = unsafe { Self::new_unchecked(index, brand) };

            Some(qubit)
        }
    }

    pub const unsafe fn new(index: Index, brand: B) -> Self {
        assert!(
            is_active(index),
            "attempt to initialize qubit in released state"
        );

        unsafe { Self::new_unchecked(index, brand) }
    }

    pub const unsafe fn new_unchecked(index: Index, brand: B) -> Self {
        Self { index, brand }
    }

    pub const fn is_released(&self) -> bool {
        is_released(self.index())
    }

    pub const fn index(&self) -> Index {
        self.index
    }
}
