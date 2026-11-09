use core::fmt;

use crate::invariant::PhantomInvariantLifetime;

pub const INVARIANT: &str = "invariant";
pub const UNIQUE: &str = "unique";
pub const ID: &str = "'id";

pub fn write_id(formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(formatter, "#[{name}] {ID}")
}

/// [`PhantomInvariantLifetime<'id>`]-like structure.
///
/// Used to manipulate and store the unique invariant lifetime `'id` obtained from
/// the [`Guard<'id>`] structure via calling [`id`].
///
/// Holding [`Self`] indicates that the lifetime `'id` is a trusted brand.
/// `'id` will not unify with another trusted brand lifetime unless it comes
/// from the same original brand (i.e. the same invocation of [`guard!`] or [`with_guard`]).
///
/// [`id`]: Guard::id
/// [`guard!`]: crate::guard
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id<'id> {
    invariant_lifetime: PhantomInvariantLifetime<'id>,
}

impl Id<'_> {
    /// Constructs [`Self`] with an unbound lifetime.
    ///
    /// You should not need to use this function; use [`guard!`] or [`with_guard`] instead.
    ///
    /// # Safety
    ///
    /// [`Self`] holds an invariant lifetime that must be derived from another generative brand.
    /// Using this function directly is the "I know what I'm doing" version
    /// of [`guard!`] or [`with_guard`] followed by calling [`id`]; restrict the lifetime
    /// to a known brand immediately to avoid introducing potentially unsound behavior.
    ///
    /// [`guard!`]: crate::guard
    /// [`id`]: Guard::id
    pub const unsafe fn new() -> Self {
        let invariant_lifetime = PhantomInvariantLifetime::new();

        Self { invariant_lifetime }
    }
}

impl fmt::Debug for Id<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_id(formatter, INVARIANT)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Guard<'id> {
    id: Id<'id>,
}

impl fmt::Debug for Guard<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_id(formatter, UNIQUE)
    }
}

impl<'id> Guard<'id> {
    /// Constructs [`Self`] from the given [`Id`] with an unbound lifetime.
    ///
    /// You should not need to use this function; use [`guard!`] or [`with_guard`] instead.
    ///
    /// # Safety
    ///
    /// [`Self`] holds an invariant lifetime that must be an unused generative brand.
    /// Using this function directly is the "I know what I'm doing" version
    /// of [`guard!`] or [`with_guard`]; restrict the lifetime to a known brand immediately
    /// to avoid introducing potentially unsound behavior.
    ///
    /// [`guard!`]: crate::guard
    pub const unsafe fn new(id: Id<'id>) -> Self {
        Self { id }
    }

    /// Returns the contained [`Id`] with the same lifetime as [`Self`].
    ///
    /// Use this function to obtain the [`Id`] for use in other APIs that require generative brands.
    pub const fn id(&self) -> Id<'id> {
        self.id
    }
}

#[doc(hidden)]
pub mod private {
    use core::marker::PhantomData;

    use crate::generative::Id;

    pub struct Anchor<'a, 'id> {
        place: PhantomData<&'a Id<'id>>,
    }

    impl<'a, 'id> Anchor<'a, 'id> {
        pub const fn new(_place: &'a Id<'id>) -> Self
        where
            'a: 'id,
        {
            Self { place: PhantomData }
        }
    }

    impl Drop for Anchor<'_, '_> {
        #[inline(always)]
        fn drop(&mut self) {}
    }
}

#[macro_export]
macro_rules! guard {
    ($name: ident) => {
        let place = unsafe { $crate::generative::Id::new() };
        let _anchor = $crate::generative::private::Anchor::new(&place);
        let $name = unsafe { $crate::generative::Guard::new(place) };
    };
}

pub fn with_guard<F, T>(function: F) -> T
where
    F: for<'id> FnOnce(Guard<'id>) -> T,
{
    function(unsafe { Guard::new(Id::new()) })
}
