use crate::{generative::Id, runtime::RuntimeId};

mod sealed {
    pub trait Sealed {}
}

pub trait Brand: Copy + Eq + sealed::Sealed {
    /// Asserts that `self` and `other` brands are equal, panicking if they are not.
    ///
    /// # Panics
    ///
    /// Panics if the brands are not equal. The panic message is implementation-defined.
    fn assert(self, other: Self);
}

impl sealed::Sealed for Id<'_> {}

impl Brand for Id<'_> {
    /// Does nothing since [`Id<'_>`] is zero-cost and does not require any runtime checks.
    ///
    /// The actual assertion is done at compile time by the Rust compiler,
    /// which ensures that the brands' lifetimes are correctly enforced.
    ///
    /// In the [`with_guard`] case, the compile error message will be
    /// `borrowed data escapes outside of closure`, code [`E0521`]:
    ///
    /// ```compile_fail
    /// use qstd_brand::prelude::{Brand, with_guard};
    ///
    /// with_guard(|brand| {
    ///     with_guard(|other| {
    ///         brand.id().assert(other.id()); // compile error!
    ///     });
    /// });
    /// ```
    ///
    /// In the [`guard!`] case, the compile error message will be
    /// `temporary value dropped while borrowed`, code [`E0716`]:
    ///
    /// ```compile_fail
    /// use qstd_brand::prelude::{Brand, guard};
    ///
    /// guard!(brand);
    /// guard!(other);
    ///
    /// brand.id().assert(other.id()); // compile error!
    /// ```
    ///
    /// [`with_guard`]: crate::generative::with_guard
    ///
    /// [`E0521`]: https://doc.rust-lang.org/error_codes/E0521.html
    ///
    /// [`guard!`]: crate::guard
    ///
    /// [`E0716`]: https://doc.rust-lang.org/error_codes/E0716.html
    #[inline(always)]
    fn assert(self, _other: Self) {}
}

impl sealed::Sealed for RuntimeId {}

impl Brand for RuntimeId {
    /// Asserts that `self` and `other` runtime brands are equal, panicking if they are not.
    ///
    /// # Panics
    ///
    /// Panics if the brands are not equal. The panic message is `runtime brand mismatch`.
    fn assert(self, other: Self) {
        assert_eq!(self, other, "runtime brand mismatch");
    }
}

/// Represents the default brand type.
pub type DefaultBrand = RuntimeId;

/// Asserts that `brand` and `other` brands are equal, panicking if they are not.
///
/// Calling this function with `brand` and `other` is the same as calling
/// [`assert`] on `brand` with `other`.
///
/// # Panics
///
/// Panics if the brands are not equal, that is, the panic from [`assert`] is propagated.
///
/// [`assert`]: Brand::assert
pub fn assert_brand<B: Brand>(brand: B, other: B) {
    brand.assert(other)
}

pub trait Branded {
    type Brand: Brand;

    fn brand(&self) -> Self::Brand;
}
