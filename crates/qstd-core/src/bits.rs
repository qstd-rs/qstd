use core::fmt::Debug;

use trait_aliases::trait_aliases;

trait_aliases! {
    pub trait Bit = Copy + Debug + Send + Sync;
}

pub trait Immediate: Bit {
    fn get(self) -> bool;
}

impl Immediate for bool {
    fn get(self) -> bool {
        self
    }
}
