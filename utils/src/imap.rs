use std::ops::{Index, IndexMut};

#[derive(Default, Clone)]
pub struct IMap<T> {
    pos: Vec<T>,
    neg: Vec<T>,
}

impl<T: Copy> IMap<T> {
    pub fn with_elem(n: usize, t: T) -> Self {
        Self {
            pos: vec![t; n],
            neg: vec![t; n - 1],
        }
    }
}
impl<T: Default> IMap<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            pos: Vec::with_capacity(n),
            neg: Vec::with_capacity(n),
        }
    }
    fn to_ix_and_arr(&self, index: isize) -> (usize, &Vec<T>) {
        if index < 0 {
            let ix = usize::try_from(-index).unwrap() - 1;
            (ix, &self.neg)
        } else {
            let ix = usize::try_from(index).unwrap();
            (ix, &self.pos)
        }
    }
    fn to_ix_and_arr_mut(&mut self, index: isize) -> (usize, &mut Vec<T>) {
        if index < 0 {
            let ix = usize::try_from(-index).unwrap() - 1;
            (ix, &mut self.neg)
        } else {
            let ix = usize::try_from(index).unwrap();
            (ix, &mut self.pos)
        }
    }

    #[must_use]
    pub fn get(&self, index: isize) -> Option<&T> {
        let (ix, v) = self.to_ix_and_arr(index);
        v.get(ix)
    }

    pub fn insert(&mut self, index: isize, t: T) {
        let (ix, v) = self.to_ix_and_arr_mut(index);
        v.extend((v.len()..=ix).map(|_| Default::default()));
        v[ix] = t;
    }
}
impl<T: Default> Index<isize> for IMap<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let (ix, v) = self.to_ix_and_arr(index);
        &v[ix]
    }
}

impl<T: Default> IndexMut<isize> for IMap<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let (ix, v) = self.to_ix_and_arr_mut(index);
        v.extend((v.len()..=ix).map(|_| Default::default()));
        &mut v[ix]
    }
}
