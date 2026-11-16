pub mod import {
    pub use core::mem::size_of;
}

macro_rules! is_zero_sized {
    ($type: ty) => {
        $crate::internals::import::size_of::<$type>() == 0
    };
}

pub(crate) use is_zero_sized;

macro_rules! const_expect_zero_sized {
    ($type: ty) => {
        concat!("`", stringify!($type), "` must be zero-sized")
    };
}

pub(crate) use const_expect_zero_sized;

macro_rules! const_assert_zero_sized {
    ($($type: ty),+ $(,)?) => {
        $(
            const _: () = assert!(
                $crate::internals::is_zero_sized!($type),
                $crate::internals::const_expect_zero_sized!($type),
            );
        )+
    };
}

pub(crate) use const_assert_zero_sized;
