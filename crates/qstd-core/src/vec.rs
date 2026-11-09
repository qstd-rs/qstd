use crate::{
    aliases::{Dyn, Index, MeasureFn, Size},
    bits::Bit,
};

cfg_select! {
    feature = "std" => {
        pub use std::vec::Vec;
    }
    feature = "alloc" => {
        pub use alloc::vec::Vec;
    }
    _ => {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

pub fn from_fn<T, F: FnMut(Index) -> T>(function: F, length: Size) -> Vec<T> {
    (0..length).map(function).collect()
}

pub trait DynMeasure<Q> {
    fn measure_with<B: Bit, M: MeasureFn<Q, B>>(&mut self, measure: M) -> Vec<B>;
}

impl<Q> DynMeasure<Q> for Dyn<Q> {
    fn measure_with<B: Bit, M: MeasureFn<Q, B>>(&mut self, measure: M) -> Vec<B> {
        self.iter_mut().map(measure).collect()
    }
}
