macro_rules! const_assert {
    ($condition: expr, $message: expr) => {
        const { assert!($condition, $message) }
    };
}

pub(crate) use const_assert;

macro_rules! const_assert_eq {
    ($left: expr, $right: expr, $message: expr) => {
        $crate::internals::const_assert!($left == $right, $message)
    };
}

pub(crate) use const_assert_eq;

macro_rules! const_assert_sum {
    ($left: expr, $right: expr, $target: expr, $message: expr) => {
        $crate::internals::const_assert_eq!($left + $right, $target, $message)
    };
}

pub(crate) use const_assert_sum;
