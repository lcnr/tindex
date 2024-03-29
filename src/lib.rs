use std::{
    borrow::{Borrow, BorrowMut, ToOwned},
    cmp::{Eq, Ordering, PartialEq},
    fmt,
    hash::{Hash, Hasher},
    iter::FromIterator,
    marker::PhantomData,
    ops::{Bound, Deref, DerefMut, Index, IndexMut, RangeBounds},
    slice::{Iter, IterMut, Windows},
    vec::{IntoIter, Splice},
};

pub mod bitset;
pub mod iter;
pub mod slice_index;

pub use bitset::TBitSet;

use iter::IndexIter;
use slice_index::TSliceIndex;

pub trait TIndex: PartialEq + Eq + Clone + Copy {
    fn as_index(self) -> usize;

    fn from_index(index: usize) -> Self;
}

impl TIndex for usize {
    #[inline]
    fn as_index(self) -> usize {
        self
    }

    #[inline]
    fn from_index(index: usize) -> Self {
        index
    }
}

impl TIndex for u32 {
    #[inline]
    fn as_index(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_index(index: usize) -> Self {
        index as u32
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
        self.inner.eq(&other.inner)
    }
}

impl<I, T: Eq> Eq for TSlice<I, T> {}

impl<I, T: Hash> Hash for TSlice<I, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<I, T> TSlice<I, T> {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

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

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.inner.last_mut()
    }

    pub fn split_last(&self) -> Option<(&T, &Self)> {
        self.inner.split_last().map(|(t, slice)| (t, slice.into()))
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.inner.contains(item)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.inner.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        self.inner.iter_mut()
    }

    pub fn to_slice<'a>(&'a self) -> &'a [T] {
        &self.inner
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

    pub fn windows(&self, size: usize) -> Windows<T> {
        self.inner.windows(size)
    }
}

impl<I: TIndex, T> TSlice<I, T> {
    pub fn get(&self, idx: I) -> Option<&T> {
        self.inner.get(idx.as_index())
    }

    pub fn get_mut(&mut self, idx: I) -> Option<&mut T> {
        self.inner.get_mut(idx.as_index())
    }

    pub fn last_id(&self) -> Option<I> {
        if self.inner.is_empty() {
            None
        } else {
            Some(I::from_index(self.inner.len() - 1))
        }
    }

    pub fn range_start(&self) -> I {
        I::from_index(0)
    }

    pub fn range_end(&self) -> I {
        I::from_index(self.inner.len())
    }

    pub fn index_iter(&self) -> IndexIter<I> {
        IndexIter::new(self.inner.len())
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
        self.inner
            .binary_search(x)
            .map(I::from_index)
            .map_err(I::from_index)
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.inner
            .binary_search_by(f)
            .map(I::from_index)
            .map_err(I::from_index)
    }

    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.inner
            .binary_search_by_key(b, f)
            .map(I::from_index)
            .map_err(I::from_index)
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

impl<I, T: Hash> Hash for TVec<I, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<I, T> Default for TVec<I, T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            inner: Vec::new(),
        }
    }
}

impl<I, T> TVec<I, T> {
    pub fn new() -> Self {
        Default::default()
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

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.len();
        let mut del = 0;
        {
            let v = &mut self.inner;

            for i in 0..len {
                if !f(&mut v[i]) {
                    del += 1;
                } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        if del > 0 {
            self.inner.truncate(len - del);
        }
    }

    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.inner.resize(new_len, value)
    }

    pub fn extend_from_slice(&mut self, other: &TSlice<I, T>)
    where
        T: Clone,
    {
        self.inner.extend_from_slice(&other.inner)
    }

    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }
}

impl<I: TIndex, T> TVec<I, T> {
    pub fn push(&mut self, item: T) -> I {
        let idx = self.inner.len();
        self.inner.push(item);
        I::from_index(idx)
    }

    pub fn insert(&mut self, idx: I, elem: T) {
        self.inner.insert(idx.as_index(), elem)
    }

    pub fn remove(&mut self, id: I) -> T {
        self.inner.remove(id.as_index())
    }

    pub fn splice<R, E>(&mut self, range: R, replace_with: E) -> Splice<'_, E::IntoIter>
    where
        R: RangeBounds<I>,
        E: IntoIterator<Item = T>,
    {
        let start = match range.start_bound() {
            Bound::Included(v) => Bound::Included(v.as_index()),
            Bound::Excluded(v) => Bound::Excluded(v.as_index()),
            Bound::Unbounded => Bound::Unbounded,
        };

        let end = match range.end_bound() {
            Bound::Included(v) => Bound::Included(v.as_index()),
            Bound::Excluded(v) => Bound::Excluded(v.as_index()),
            Bound::Unbounded => Bound::Unbounded,
        };

        self.inner.splice((start, end), replace_with)
    }

    pub fn split_off(&mut self, at: I) -> Self {
        self.inner.split_off(at.as_index()).into()
    }
}

impl<'a, I, T: Clone> From<&'a TSlice<I, T>> for TVec<I, T> {
    #[inline]
    fn from(slice: &'a TSlice<I, T>) -> Self {
        slice.to_owned()
    }
}

impl<'a, I, T: Clone> From<&'a mut TSlice<I, T>> for TVec<I, T> {
    #[inline]
    fn from(slice: &'a mut TSlice<I, T>) -> Self {
        slice.to_owned()
    }
}

impl<'a, I, T: Clone> From<&'a [T]> for TVec<I, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        Self::from_vec(slice.into())
    }
}

impl<'a, I, T: Clone> From<&'a mut [T]> for TVec<I, T> {
    #[inline]
    fn from(slice: &'a mut [T]) -> Self {
        Self::from_vec(slice.into())
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

impl<I, T> Extend<T> for TVec<I, T> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        self.inner.extend(iter)
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
