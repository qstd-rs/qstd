use core::{
    any::type_name,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub const LIFETIME: &str = "'_";

#[repr(transparent)]
pub struct PhantomInvariant<T: ?Sized> {
    invariant: PhantomData<fn(T) -> T>,
}

impl<T: ?Sized> Default for PhantomInvariant<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> PhantomInvariant<T> {
    pub const fn new() -> Self {
        Self {
            invariant: PhantomData,
        }
    }
}

impl<T: ?Sized> fmt::Debug for PhantomInvariant<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = stringify!(PhantomInvariant);

        let type_name = type_name::<T>();

        write!(formatter, "{name}<{type_name}>")
    }
}

impl<T: ?Sized> Clone for PhantomInvariant<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for PhantomInvariant<T> {}

impl<T: ?Sized> PartialEq for PhantomInvariant<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: ?Sized> Eq for PhantomInvariant<T> {}

impl<T: ?Sized> PartialOrd for PhantomInvariant<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Ord for PhantomInvariant<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: ?Sized> Hash for PhantomInvariant<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhantomInvariantLifetime<'a> {
    lifetime: PhantomInvariant<&'a ()>,
}

impl PhantomInvariantLifetime<'_> {
    pub const fn new() -> Self {
        let lifetime = PhantomInvariant::new();

        Self { lifetime }
    }
}

impl fmt::Debug for PhantomInvariantLifetime<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = stringify!(PhantomInvariantLifetime);

        write!(formatter, "{name}<{LIFETIME}>")
    }
}
