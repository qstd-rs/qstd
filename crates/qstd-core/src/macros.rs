#[macro_export]
macro_rules! const_assert {
    ($condition: expr, $message: expr) => {
        const { assert!($condition, $message) }
    };
}

#[macro_export]
macro_rules! const_assert_eq {
    ($left: expr, $right: expr, $message: expr) => {
        $crate::const_assert!($left == $right, $message)
    };
}

#[macro_export]
macro_rules! const_assert_ne {
    ($left: expr, $right: expr, $message: expr) => {
        $crate::const_assert!($left != $right, $message)
    };
}

#[macro_export]
macro_rules! const_assert_sum {
    ($left: expr, $right: expr, $target: expr, $message: expr) => {
        $crate::const_assert_eq!($left + $right, $target, $message)
    };
}

#[macro_export]
macro_rules! const_assert_in_place {
    ($condition: expr, $message: expr) => {
        const _: () = assert!($condition, $message);
    };
}

#[macro_export]
macro_rules! const_assert_eq_in_place {
    ($left: expr, $right: expr, $message: expr) => {
        $crate::const_assert_in_place!($left == $right, $message);
    };
}

#[macro_export]
macro_rules! const_assert_ne_in_place {
    ($left: expr, $right: expr, $message: expr) => {
        $crate::const_assert_in_place!($left != $right, $message);
    };
}
