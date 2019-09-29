use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{TIndex, TSlice};

mod private {
    use super::*;

    pub trait Sealed {}

    impl<I: TIndex> Sealed for I {}
    impl<I: TIndex> Sealed for Range<I> {}
    impl<I: TIndex> Sealed for RangeFrom<I> {}
    impl Sealed for RangeFull {}
    impl<I: TIndex> Sealed for RangeInclusive<I> {}
    impl<I: TIndex> Sealed for RangeTo<I> {}
    impl<I: TIndex> Sealed for RangeToInclusive<I> {}
}

pub trait TSliceIndex<T: ?Sized>: private::Sealed {
    /// The output type returned by methods.
    type Output: ?Sized;

    /// Returns a shared reference to the output at this location, if in
    /// bounds.
    fn get(self, slice: &T) -> Option<&Self::Output>;

    /// Returns a mutable reference to the output at this location, if in
    /// bounds.
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    /// Returns a shared reference to the output at this location, panicking
    /// if out of bounds.
    fn index(self, slice: &T) -> &Self::Output;

    /// Returns a mutable reference to the output at this location, panicking
    /// if out of bounds.
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for I {
    type Output = T;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice.inner.get(self.as_index())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice.inner.get_mut(self.as_index())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        &slice.inner[self.as_index()]
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        &mut slice.inner[self.as_index()]
    }
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for Range<I> {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice
            .inner
            .get(Range {
                start: self.start.as_index(),
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice
            .inner
            .get_mut(Range {
                start: self.start.as_index(),
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[Range {
            start: self.start.as_index(),
            end: self.end.as_index(),
        }]
        .into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[Range {
            start: self.start.as_index(),
            end: self.end.as_index(),
        }])
            .into()
    }
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for RangeFrom<I> {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice
            .inner
            .get(RangeFrom {
                start: self.start.as_index(),
            })
            .map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice
            .inner
            .get_mut(RangeFrom {
                start: self.start.as_index(),
            })
            .map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[RangeFrom {
            start: self.start.as_index(),
        }]
        .into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[RangeFrom {
            start: self.start.as_index(),
        }])
            .into()
    }
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for RangeFull {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice.inner.get(self).map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice.inner.get_mut(self).map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[self].into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[self]).into()
    }
}

fn range_inclusive<I: TIndex>(range: RangeInclusive<I>) -> RangeInclusive<usize> {
    let (start, end) = range.into_inner();
    RangeInclusive::new(start.as_index(), end.as_index())
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for RangeInclusive<I> {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice.inner.get(range_inclusive(self)).map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice.inner.get_mut(range_inclusive(self)).map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[range_inclusive(self)].into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[range_inclusive(self)]).into()
    }
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for RangeTo<I> {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice
            .inner
            .get(RangeTo {
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice
            .inner
            .get_mut(RangeTo {
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[RangeTo {
            end: self.end.as_index(),
        }]
        .into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[RangeTo {
            end: self.end.as_index(),
        }])
            .into()
    }
}

impl<I: TIndex, T> TSliceIndex<TSlice<I, T>> for RangeToInclusive<I> {
    type Output = TSlice<I, T>;

    fn get(self, slice: &TSlice<I, T>) -> Option<&Self::Output> {
        slice
            .inner
            .get(RangeToInclusive {
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn get_mut(self, slice: &mut TSlice<I, T>) -> Option<&mut Self::Output> {
        slice
            .inner
            .get_mut(RangeToInclusive {
                end: self.end.as_index(),
            })
            .map(|s| s.into())
    }

    fn index(self, slice: &TSlice<I, T>) -> &Self::Output {
        slice.inner[RangeToInclusive {
            end: self.end.as_index(),
        }]
        .into()
    }

    fn index_mut(self, slice: &mut TSlice<I, T>) -> &mut Self::Output {
        (&mut slice.inner[RangeToInclusive {
            end: self.end.as_index(),
        }])
            .into()
    }
}
