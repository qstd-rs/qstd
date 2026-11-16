use core::fmt::Debug;

// TRAITS

mod sealed {
    pub trait Sealed {}
}

pub trait Strictness: Copy + Debug + sealed::Sealed {
    const STRICT: bool;

    type Inverse: Strictness<Inverse = Self>;

    fn new() -> Self;
}

// DEFAULT

pub type DefaultStrictness = Lenient;

// TYPES

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lenient;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Strict;

impl Default for Lenient {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Strict {
    fn default() -> Self {
        Self::new()
    }
}

impl Lenient {
    pub const fn new() -> Self {
        Self
    }
}

impl Strict {
    pub const fn new() -> Self {
        Self
    }
}

// IMPLEMENTATION

impl sealed::Sealed for Lenient {}

impl sealed::Sealed for Strict {}

impl Strictness for Lenient {
    const STRICT: bool = false;

    type Inverse = Strict;

    fn new() -> Self {
        Self::new()
    }
}

impl Strictness for Strict {
    const STRICT: bool = true;

    type Inverse = Lenient;

    fn new() -> Self {
        Self::new()
    }
}

// INVERSION

pub type Invert<S> = <S as Strictness>::Inverse;

// FUNCTIONS

pub const fn strict<S: Strictness>() -> bool {
    S::STRICT
}

pub const fn not_strict<S: Strictness>() -> bool {
    strict::<Invert<S>>()
}

// ASSERTIONS

const fn assert_involution<S: Strictness>(strictness: S) -> Invert<Invert<S>> {
    strictness
}

macro_rules! const_assert_involution {
    ($($type: ty),+ $(,)?) => {
        $(
            const _: () = {
                let strictness = <$type>::new();

                $crate::strictness::assert_involution(strictness);
            };
        )+
    };
}

macro_rules! const_assert_negation {
    ($($type: ty),+ $(,)?) => {
        $(
            $crate::const_assert_ne_in_place!(
                $crate::strictness::strict::<$type>(),
                $crate::strictness::not_strict::<$type>(),
                "negation assertion failed"
            );
        )+
    }
}

// ASSERTION CALLS

const_assert_involution!(Lenient, Strict);
const_assert_negation!(Lenient, Strict);
