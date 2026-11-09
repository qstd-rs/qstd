use trait_aliases::trait_aliases;

pub type Index = usize;
pub type Size = usize;

pub type Dyn<T> = [T];

pub type Array<T, const N: Size> = [T; N];

trait_aliases! {
    pub trait MeasureFn<Q, B> = FnMut(&mut Q) -> B;
}
