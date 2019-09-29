use std::{
    cmp::{Eq, PartialEq},
    fmt,
    iter::{self, FromIterator},
    marker::PhantomData,
    mem,
};

use crate::TIndex;

type Frame = u64;

const FRAME_SIZE: usize = mem::size_of::<Frame>() * 8;

pub struct TBitSet<I> {
    _marker: PhantomData<fn(I)>,
    inner: Vec<Frame>,
}

impl<I> fmt::Debug for TBitSet<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.inner.iter().map(|frame| format!("{:#b}", frame)))
            .finish()
    }
}

impl<I> Clone for TBitSet<I> {
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

impl<I> PartialEq for TBitSet<I> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.frame_count() < rhs.frame_count() {
            self.inner
                .iter()
                .copied()
                .chain(iter::repeat(0))
                .zip(rhs.inner.iter().copied())
                .all(|(a, b)| a == b)
        } else {
            self.inner
                .iter()
                .copied()
                .zip(rhs.inner.iter().copied().chain(iter::repeat(0)))
                .all(|(a, b)| a == b)
        }
    }
}

impl<I> Eq for TBitSet<I> {}

impl<I> TBitSet<I> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            inner: Vec::new(),
        }
    }

    pub fn frame_count(&self) -> usize {
        self.inner.len()
    }

    pub fn element_count(&self) -> usize {
        self.inner
            .iter()
            .fold(0, |sum, elem| sum + elem.count_ones() as usize)
    }

    pub fn shrink_to_fit(&mut self) {
        while self.inner.last().map_or(false, |&l| l == 0) {
            self.inner.pop();
        }
    }
}

impl<I: TIndex> TBitSet<I> {
    #[inline]
    fn set_usize(&mut self, idx: usize, value: bool) {
        let frame_offset = idx / FRAME_SIZE;
        if frame_offset >= self.inner.len() {
            if value {
                self.inner.resize(frame_offset + 1, 0);
                self.inner[frame_offset] |= 1 << idx - frame_offset * FRAME_SIZE;
            }
        } else {
            if value {
                self.inner[frame_offset] |= 1 << idx - frame_offset * FRAME_SIZE;
            } else {
                self.inner[frame_offset] &= !(1 << idx - frame_offset * FRAME_SIZE);
            }
        }
    }

    pub fn set(&mut self, idx: I, value: bool) {
        self.set_usize(idx.as_index(), value)
    }

    pub fn add(&mut self, idx: I) {
        self.set_usize(idx.as_index(), true)
    }

    pub fn remove(&mut self, idx: I) {
        self.set_usize(idx.as_index(), false)
    }

    fn flip_usize(&mut self, idx: usize) {
        let frame_offset = idx / FRAME_SIZE;
        if frame_offset >= self.inner.len() {
            self.inner.resize(frame_offset + 1, 0);
        }

        self.inner[frame_offset] ^= 1 << idx - frame_offset * FRAME_SIZE;
    }

    pub fn flip(&mut self, idx: I) {
        self.flip_usize(idx.as_index())
    }

    #[inline]
    fn get_usize(&self, idx: usize) -> bool {
        let frame_offset = idx / FRAME_SIZE;
        self.inner
            .get(frame_offset)
            .map_or(false, |v| v & (1 << idx - frame_offset * FRAME_SIZE) != 0)
    }

    pub fn get(&self, idx: I) -> bool {
        self.get_usize(idx.as_index())
    }

    pub fn iter(&self) -> Iter<I> {
        Iter {
            inner: self,
            pos: 0,
        }
    }
}

impl<I: TIndex> FromIterator<I> for TBitSet<I> {
    #[inline]
    fn from_iter<U: IntoIterator<Item = I>>(iter: U) -> TBitSet<I> {
        let mut set = TBitSet::new();
        for idx in iter {
            set.add(idx);
        }
        set
    }
}

impl<I: TIndex> IntoIterator for TBitSet<I> {
    type Item = I;
    type IntoIter = IntoIter<I>;

    fn into_iter(self) -> IntoIter<I> {
        IntoIter {
            inner: self,
            pos: 0,
        }
    }
}

pub struct Iter<'a, I> {
    inner: &'a TBitSet<I>,
    pos: usize,
}

impl<'a, I: TIndex> Iterator for Iter<'a, I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        while self.pos < self.inner.frame_count() * FRAME_SIZE {
            let pos = self.pos;
            self.pos += 1;
            if self.inner.get_usize(pos) {
                return Some(pos.into());
            }
        }

        None
    }
}

pub struct IntoIter<I> {
    inner: TBitSet<I>,
    pos: usize,
}

impl<I: TIndex> Iterator for IntoIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        while self.pos < self.inner.frame_count() * FRAME_SIZE {
            let pos = self.pos;
            self.pos += 1;
            if self.inner.get_usize(pos) {
                return Some(pos.into());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut set = TBitSet::new();
        assert_eq!(set.element_count(), 0);
        assert_eq!(set.get(1000000), false);
        assert_eq!(set.frame_count(), 0);
        set.add(3);
        assert_eq!(set.frame_count(), 1);
        assert_eq!(set.get(3), true);
        assert_eq!(set.get(4), false);
        set.add(5);
        assert_eq!(set.element_count(), 2);
        assert_eq!(set.get(5), true);
        set.add(FRAME_SIZE + 2);
        assert_eq!(set.frame_count(), 2);
        assert_eq!(set.get(FRAME_SIZE + 2), true);
        assert_eq!(set.get(FRAME_SIZE + 1), false);
        set.flip(FRAME_SIZE + 4);
        assert_eq!(set.get(FRAME_SIZE), false);
        assert_eq!(set.get(FRAME_SIZE + 2), true);
        assert_eq!(set.get(FRAME_SIZE + 4), true);
        set.flip(FRAME_SIZE + 4);
        assert_eq!(set.get(FRAME_SIZE + 4), false);
        set.flip(FRAME_SIZE * 2 + 1);
        assert_eq!(set.frame_count(), 3);
        assert_eq!(set.get(FRAME_SIZE * 2 + 1), true);
        assert_eq!(set.get(FRAME_SIZE * 2 + 3), false);
        set.remove(FRAME_SIZE * 2 + 1);
        assert_eq!(set.get(FRAME_SIZE * 2 + 1), false);
        set.remove(FRAME_SIZE * 2 + 1);
        assert_eq!(set.get(FRAME_SIZE * 2 + 1), false);
        set.remove(FRAME_SIZE * 100);
        assert_eq!(set.frame_count(), 3);
        assert_eq!(set.element_count(), 3);
    }

    #[test]
    fn eq() {
        let mut a = TBitSet::new();
        let mut b = TBitSet::new();
        a.add(FRAME_SIZE * 2);
        assert_ne!(a, b);
        b.add(FRAME_SIZE * 2);
        assert_eq!(a, b);
        a.add(FRAME_SIZE * 3);
        assert_ne!(a, b);
        a.remove(FRAME_SIZE * 3);
        assert_ne!(a.frame_count(), b.frame_count());
        assert_eq!(a, b);
        b.add(FRAME_SIZE * 4);
        assert_ne!(a, b);
        b.remove(FRAME_SIZE * 4);
        assert_ne!(a.frame_count(), b.frame_count());
        assert_eq!(a, b);
    }

    #[test]
    fn iter() {
        let set: TBitSet<_> = [7, 4, 3, 4, 1, 1000].iter().copied().collect();
        assert_eq!(set.get(1), true);
        assert_eq!(set.get(2), false);
        assert_eq!(set.get(4), true);
        assert_eq!(set.get(7), true);
        assert_eq!(set.get(99), false);
        assert_eq!(set.get(1000), true);
        dbg!(&set);

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(1000));
        assert_eq!(iter.next(), None);

        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(1000));
        assert_eq!(iter.next(), None);
    }
}
