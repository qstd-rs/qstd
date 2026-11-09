use core::array;

use crate::{
    aliases::{Array, Index, MeasureFn, Size},
    bits::Bit,
    internals::const_assert_sum,
};

pub type Pair<T> = (T, T);

pub trait ArrayMethods<T, const N: Size> {
    fn split_mut<const L: Size, const R: Size>(&mut self) -> (&mut Array<T, L>, &mut Array<T, R>);

    fn split<const L: Size, const R: Size>(self) -> (Array<T, L>, Array<T, R>);

    fn try_pair_mut(&mut self, one: Index, two: Index) -> Option<Pair<&mut T>>;

    fn pair_mut(&mut self, one: Index, two: Index) -> Pair<&mut T> {
        self.try_pair_mut(one, two)
            .expect("indices must be different and within bounds")
    }
}

impl<T, const N: Size> ArrayMethods<T, N> for Array<T, N> {
    fn split_mut<const L: Size, const R: Size>(&mut self) -> (&mut Array<T, L>, &mut Array<T, R>) {
        const_assert_sum!(L, R, N, "split sizes must add up to the array length");

        let (left, right) = self.split_at_mut(L);

        (
            left.try_into().expect("length checked by const assert"),
            right.try_into().expect("length checked by const assert"),
        )
    }

    fn split<const L: Size, const R: Size>(self) -> (Array<T, L>, Array<T, R>) {
        const_assert_sum!(L, R, N, "split sizes must add up to the array length");

        let mut iterator = self.into_iter();

        let left = array::from_fn(|_| iterator.next().expect("length checked by const assert"));
        let right = array::from_fn(|_| iterator.next().expect("length checked by const assert"));

        (left, right)
    }

    fn try_pair_mut(&mut self, one: Index, two: Index) -> Option<Pair<&mut T>> {
        let [one_value, two_value] = self.get_disjoint_mut([one, two]).ok()?;

        let pair = (one_value, two_value);

        Some(pair)
    }
}

pub trait ArrayMeasure<Q, const N: Size> {
    fn measure_with<B: Bit, M: MeasureFn<Q, B>>(&mut self, measure: M) -> Array<B, N>;
}

impl<Q, const N: Size> ArrayMeasure<Q, N> for Array<Q, N> {
    fn measure_with<B: Bit, M: MeasureFn<Q, B>>(&mut self, measure: M) -> Array<B, N> {
        let mut iterator = self.iter_mut().map(measure);

        array::from_fn(|_| iterator.next().expect("iterator yields exactly N items"))
    }
}
