use std::{
    borrow::{Borrow, BorrowMut, ToOwned},
    cmp::{Eq, Ordering, PartialEq},
    fmt,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

pub mod bitset;
pub mod slice_index;

use slice_index::TSliceIndex;

pub trait TIndex: From<usize> {
    fn as_index(self) -> usize;
}

impl TIndex for usize {
    fn as_index(self) -> usize {
        self
    }
}

#[repr(transparent)]
pub struct TSlice<I, T> {
    _marker: PhantomData<fn(I)>,
    inner: [T],
}

impl<I, T: fmt::Debug> fmt::Debug for TSlice<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<I, T: Clone> ToOwned for TSlice<I, T> {
    type Owned = TVec<I, T>;

    fn to_owned(&self) -> Self::Owned {
        self.inner.to_vec().into()
    }
}

impl<I, T: PartialEq> PartialEq for TSlice<I, T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            self.iter().zip(other.iter()).all(|(s, o)| s == o)
        } else {
            false
        }
    }
}

impl<I, T: Eq> Eq for TSlice<I, T> {}

impl<I, T> TSlice<I, T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn first(&self) -> Option<&T> {
        self.inner.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.inner.first_mut()
    }

    pub fn last(&self) -> Option<&T> {
        self.inner.last()
    }

    pub fn split_last(&self) -> Option<(&T, &Self)> {
        self.inner.split_last().map(|(t, slice)| (t, slice.into()))
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.inner.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        self.inner.iter_mut()
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort()
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.inner.sort_by(compare)
    }

    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.inner.sort_by_key(f)
    }

    pub fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.inner.sort_by_cached_key(f)
    }
}

impl<I: TIndex, T> TSlice<I, T> {
    pub fn get(&self, idx: I) -> Option<&T> {
        self.inner.get(idx.as_index())
    }

    pub fn get_mut(&mut self, idx: I) -> Option<&mut T> {
        self.inner.get_mut(idx.as_index())
    }

    pub fn swap(&mut self, a: I, b: I) {
        self.inner.swap(a.as_index(), b.as_index())
    }

    pub fn split_at(&self, mid: I) -> (&Self, &Self) {
        let (left, right) = self.inner.split_at(mid.as_index());
        (left.into(), right.into())
    }

    pub fn split_at_mut(&mut self, mid: I) -> (&mut Self, &mut Self) {
        let (left, right) = self.inner.split_at_mut(mid.as_index());
        (left.into(), right.into())
    }

    pub fn binary_search(&self, x: &T) -> Result<I, I>
    where
        T: Ord,
    {
        self.inner.binary_search(x).map(I::from).map_err(I::from)
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.inner.binary_search_by(f).map(I::from).map_err(I::from)
    }

    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.inner
            .binary_search_by_key(b, f)
            .map(I::from)
            .map_err(I::from)
    }
}

impl<'a, I, T> From<&'a [T]> for &'a TSlice<I, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        unsafe {
            // SAFETY: as `TSlice` is `#[repr(transparent)]`
            // casting from `[T]` to `TSlice<I, T>` is safe
            let ptr = slice as *const _;
            let cast = ptr as *const TSlice<I, T>;
            &*cast
        }
    }
}

impl<'a, I, T> From<&'a mut [T]> for &'a mut TSlice<I, T> {
    #[inline]
    fn from(slice: &'a mut [T]) -> Self {
        unsafe {
            // SAFETY: as `TSlice` is `#[repr(transparent)]`
            // casting from `[T]` to `TSlice<I, T>` is safe
            let ptr = slice as *mut _;
            let cast = ptr as *mut TSlice<I, T>;
            &mut *cast
        }
    }
}

impl<I: TIndex, S: TSliceIndex<TSlice<I, T>>, T> Index<S> for TSlice<I, T> {
    type Output = S::Output;

    fn index(&self, index: S) -> &S::Output {
        index.index(self)
    }
}

impl<I: TIndex, S: TSliceIndex<TSlice<I, T>>, T> IndexMut<S> for TSlice<I, T> {
    fn index_mut(&mut self, index: S) -> &mut S::Output {
        index.index_mut(self)
    }
}

#[repr(transparent)]
pub struct TVec<I, T> {
    _marker: PhantomData<fn(I)>,
    inner: Vec<T>,
}

#[macro_export]
macro_rules! tvec {
    ($elem:expr; $n:expr) => (
        $crate::TVec::from_vec(vec![$elem; $n])
    );
    ($($x:expr),*) => (
        $crate::TVec::from_vec(vec![$($x),*])
    );
    ($($x:expr,)*) => (tvec![$($x),*])
}

impl<I, T: fmt::Debug> fmt::Debug for TVec<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<I, T> Borrow<TSlice<I, T>> for TVec<I, T> {
    fn borrow(&self) -> &TSlice<I, T> {
        self.as_ref()
    }
}

impl<I, T> BorrowMut<TSlice<I, T>> for TVec<I, T> {
    fn borrow_mut(&mut self) -> &mut TSlice<I, T> {
        self.as_mut()
    }
}

impl<I, T: Clone> Clone for TVec<I, T> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

impl<I, T: PartialEq> PartialEq for TVec<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<I, T: Eq> Eq for TVec<I, T> {}

impl<I, T> TVec<I, T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            inner: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            _marker: PhantomData,
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            _marker: PhantomData,
            inner: vec,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }
}
impl<I, T: Clone> TVec<I, T> {
    pub fn extend_from_slice(&mut self, other: &TSlice<I, T>) {
        self.inner.extend_from_slice(&other.inner)
    }
}

impl<I: TIndex, T> TVec<I, T> {
    pub fn push(&mut self, item: T) -> I {
        let idx = self.inner.len();
        self.inner.push(item);
        idx.into()
    }

    pub fn remove(&mut self, id: I) -> T {
        self.inner.remove(id.as_index())
    }

    pub fn split_off(&mut self, at: I) -> Self {
        self.inner.split_off(at.as_index()).into()
    }

    pub fn last_id(&self) -> Option<I> {
        if self.inner.is_empty() {
            None
        } else {
            Some((self.inner.len() - 1).into())
        }
    }
}

impl<'a, I, T> From<&'a Vec<T>> for &'a TVec<I, T> {
    #[inline]
    fn from(vec: &'a Vec<T>) -> Self {
        unsafe {
            // SAFETY: as `TVec` is `#[repr(transparent)]`
            // casting from `Vec<T>` to `TSlice<I, T>` is safe
            let ptr = vec as *const _;
            let cast = ptr as *const TVec<I, T>;
            &*cast
        }
    }
}

impl<'a, I, T> From<&'a mut Vec<T>> for &'a mut TVec<I, T> {
    #[inline]
    fn from(vec: &'a mut Vec<T>) -> Self {
        unsafe {
            // SAFETY: as `TVec` is `#[repr(transparent)]`
            // casting from `Vec<T>` to `TSlice<I, T>` is safe
            let ptr = vec as *mut _;
            let cast = ptr as *mut TVec<I, T>;
            &mut *cast
        }
    }
}

impl<I, T> From<Vec<T>> for TVec<I, T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<I, T> IntoIterator for TVec<I, T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.inner.into_iter()
    }
}

impl<'a, I, T> IntoIterator for &'a TVec<I, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut TVec<I, T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<I, T> FromIterator<T> for TVec<I, T> {
    #[inline]
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> TVec<I, T> {
        Self {
            inner: iter.into_iter().collect(),
            _marker: PhantomData,
        }
    }
}

impl<I, T> Deref for TVec<I, T> {
    type Target = TSlice<I, T>;

    fn deref<'a>(&'a self) -> &'a TSlice<I, T> {
        self.inner.deref().into()
    }
}

impl<I, T> DerefMut for TVec<I, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut TSlice<I, T> {
        self.inner.deref_mut().into()
    }
}

impl<I, T> AsRef<TSlice<I, T>> for TVec<I, T> {
    fn as_ref(&self) -> &TSlice<I, T> {
        self
    }
}

impl<I, T> AsMut<TSlice<I, T>> for TVec<I, T> {
    fn as_mut(&mut self) -> &mut TSlice<I, T> {
        self
    }
}
