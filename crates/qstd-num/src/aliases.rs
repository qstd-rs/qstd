use core::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use trait_aliases::trait_aliases;

use crate::consts::{Const, One, Zero};

trait_aliases! {
    #[trait_alias(N)]
    pub trait OutputOps<R = Self, T = Self> =
        Add<R, Output = T> + Sub<R, Output = T> + Mul<R, Output = T> + Div<R, Output = T>;

    #[trait_alias(N)]
    pub trait AssignOps<R = Self> = AddAssign<R> + SubAssign<R> + MulAssign<R> + DivAssign<R>;

    #[trait_alias(N)]
    pub trait Ops<R = Self, T = Self> = OutputOps<R, T> + AssignOps<R>;

    #[trait_alias(N)]
    pub trait SelfOps = Sized + OutputOps + AssignOps;

    #[trait_alias(N)]
    pub trait Negate<T = Self> = Neg<Output = T>;

    #[trait_alias(N)]
    pub trait Fold<T = Self> = Sum<T> + Product<T>;

    #[trait_alias(N)]
    pub trait Number = PartialEq + SelfOps + Zero + One;

    #[trait_alias(N)]
    pub trait Ordered = PartialOrd + Number;

    #[trait_alias(N)]
    pub trait Signed = Ordered + Negate;

    #[trait_alias(T)]
    pub trait Thread = Send + Sync;

    #[trait_alias(T)]
    pub trait Format = Debug + Display;

    #[trait_alias(R)]
    pub trait Traits = Copy + Default + Thread + Format + Fold + Signed + Const;
}
