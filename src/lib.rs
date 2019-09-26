use std::{
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
    fmt,
};

pub trait TIndex: From<usize> {
    fn as_index(self) -> usize;
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

impl<I, T> TSlice<I, T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.inner.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        self.inner.iter_mut()
    }
}

impl<I: TIndex, T> Index<I> for TSlice<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &T {
        &self.inner[index.as_index()]
    }
}

impl<I: TIndex, T> IndexMut<I> for TSlice<I, T> {
    fn index_mut(&mut self, index: I) -> &mut T {
        &mut self.inner[index.as_index()]
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

impl<I, T> TVec<I, T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            inner: Vec::new(),
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

    pub fn last(&self) -> Option<&T> {
        self.inner.last()
    }
}

impl<I: TIndex, T> TVec<I, T> {
    pub fn push(&mut self, item: T) -> I {
        let idx = self.inner.len();
        self.inner.push(item);
        idx.into()
    }

    pub fn last_id(&self) -> Option<I> {
        if self.inner.is_empty() {
            None
        } else {
            Some((self.inner.len() - 1).into())
        }
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
        unsafe {
            // SAFETY: as `TSlice` is `#[repr(transparent)]`
            // casting from `[T]` to `TSlice<I, T>` is safe
            let ptr = self.inner.deref() as *const _;
            let cast = ptr as *const TSlice<I, T>;
            &*cast
        }
    }
}

impl<I, T> DerefMut for TVec<I, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut TSlice<I, T> {
        unsafe {
            // SAFETY: as `TSlice` is `#[repr(transparent)]`
            // casting from `[T]` to `TSlice<I, T>` is safe
            let ptr = self.inner.deref_mut() as *mut _;
            let cast = ptr as *mut TSlice<I, T>;
            &mut *cast
        }
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