use core::{
    fmt,
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

/// The `runtime` literal.
pub const RUNTIME: &str = "runtime";

/// Represents non-atomic identifiers.
pub type NonAtomicId = u64;

/// Represents non-zero identifiers (non-atomic).
pub type NonZeroId = NonZeroU64;

/// Represents atomic identifiers.
pub type AtomicId = AtomicU64;

/// Represents the initial [`NonAtomicId`].
pub const ONE: NonAtomicId = 1;

/// Represents runtime brand identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RuntimeId {
    inner: NonZeroId,
}

impl fmt::Display for RuntimeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "#[{RUNTIME}({inner})]", inner = self.get())
    }
}

static NEXT: AtomicId = AtomicId::new(ONE);

fn increment(id: NonAtomicId) -> Option<NonAtomicId> {
    id.checked_add(ONE)
}

fn next() -> NonZeroId {
    let id = NEXT
        .try_update(Ordering::Relaxed, Ordering::Relaxed, increment)
        .expect("ran out of brands");

    NonZeroId::new(id).expect("always non-zero and never wraps")
}

impl RuntimeId {
    /// Constructs [`Self`] from the given [`NonZeroId`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given [`NonZeroId`] *uniquely* identifies
    /// *exactly one* specific runtime brand.
    pub const unsafe fn new(inner: NonZeroId) -> Self {
        Self { inner }
    }

    /// Returns the inner [`NonZeroId`].
    pub const fn get_non_zero(self) -> NonZeroId {
        self.inner
    }

    /// Returns the inner [`NonAtomicId`].
    ///
    /// Prefer [`get_non_zero`] if you need to use the inner identifier
    /// in the context where the value is required to be non-zero.
    ///
    /// ```
    /// use qstd_brand::prelude::RuntimeId;
    ///
    /// // obtain the next runtime brand identifier
    /// let id = RuntimeId::next();
    ///
    /// // obtain the inner `Id`
    /// let inner = id.get();
    ///
    /// // which is equivalent to this
    /// let other = id.get_non_zero().get();
    ///
    /// assert_eq!(inner, other);
    /// ```
    ///
    /// [`get_non_zero`]: Self::get_non_zero
    pub const fn get(self) -> NonAtomicId {
        self.get_non_zero().get()
    }

    /// Returns the next available runtime brand identifier.
    ///
    /// # Panics
    ///
    /// Panics if the next available identifier would overflow [`NonZeroId`]
    /// (or, equivalently, [`NonAtomicId`]), with the `ran out of brands` message.
    pub fn next() -> Self {
        let id = next();

        // SAFETY: `next` uses checked increments and never wraps
        unsafe { Self::new(id) }
    }
}

/// Shorthand for calling [`next`] in [`RuntimeId`].
///
/// # Panics
///
/// Panics are propagated from [`next`].
///
/// [`next`]: RuntimeId::next
pub fn runtime_id() -> RuntimeId {
    RuntimeId::next()
}
