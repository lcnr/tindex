use std::{
    iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator},
    marker::PhantomData,
    ops::Range,
};

use crate::TIndex;

#[derive(Debug)]
pub struct IndexIter<I> {
    _marker: PhantomData<I>,
    inner: Range<usize>,
}

impl<I> IndexIter<I> {
    pub(crate) fn new(end: usize) -> Self {
        Self {
            _marker: PhantomData,
            inner: 0..end,
        }
    }
}

impl<I> Clone for IndexIter<I> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            inner: self.inner.clone(),
        }
    }
}

impl<I: TIndex> Iterator for IndexIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        self.inner.next().map(I::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I: TIndex> DoubleEndedIterator for IndexIter<I> {
    #[inline]
    fn next_back(&mut self) -> Option<I> {
        self.inner.next_back().map(I::from)
    }
}

impl<I: TIndex> ExactSizeIterator for IndexIter<I> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<I: TIndex> FusedIterator for IndexIter<I> {}
