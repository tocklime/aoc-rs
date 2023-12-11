use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{AddAssign, Index};
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

pub trait Intersections<K>: Iterator {
    fn intersections(&mut self) -> HashSet<K>;
}
impl<'a, K, I> Intersections<K> for I
where
    K: 'a + Eq + Hash + Clone,
    I: Iterator<Item = &'a HashSet<K>>,
{
    fn intersections(&mut self) -> HashSet<K> {
        let first = self
            .next()
            .expect("Can't intersect empty hashset iterator")
            .clone();
        self.fold(first, |i, a| i.intersection(a).cloned().collect())
    }
}
pub trait ToLookup<K, V>: Iterator {
    fn collect_lookup(&mut self) -> HashMap<K, Vec<V>>;
}

impl<K, V, I: Iterator<Item = (K, V)>> ToLookup<K, V> for I
where
    K: Hash + Eq,
{
    fn collect_lookup(&mut self) -> HashMap<K, Vec<V>> {
        let mut ans = HashMap::new();
        for (k, v) in self {
            ans.entry(k).or_insert_with(Vec::new).push(v);
        }
        ans
    }
}
pub trait ToLookupSet<K, V>: Iterator {
    fn collect_lookup_set(&mut self) -> HashMap<K, HashSet<V>>;
}

impl<K, V, I: Iterator<Item = (K, V)>> ToLookupSet<K, V> for I
where
    K: Hash + Eq,
    V: Eq + Hash,
{
    fn collect_lookup_set(&mut self) -> HashMap<K, HashSet<V>> {
        let mut ans = HashMap::new();
        for (k, v) in self {
            ans.entry(k).or_insert_with(HashSet::new).insert(v);
        }
        ans
    }
}

pub fn prefix_sum_vec<T: AddAssign + Default + Copy>(input: &[T]) -> Vec<T> {
    let mut total: T = Default::default();
    let mut ans = Vec::with_capacity(input.len());
    for i in input {
        total += *i;
        ans.push(total);
    }
    ans
}

pub fn prefix_sum<'a, T, I>(input: I) -> impl Iterator<Item = T> + 'a
where
    T: 'a + AddAssign + Default + Copy,
    I: 'a + IntoIterator<Item = &'a T>,
{
    input.into_iter().scan(T::default(), |acc, x| {
        *acc += *x;
        Some(*acc)
    })
}

pub fn minmax<'a, T, I: IntoIterator<Item = &'a T>>(input: I) -> Option<(&'a T, &'a T)>
where
    T: Ord,
{
    let mut i = input.into_iter();
    i.next()
        .map(|x| i.fold((x, x), |(min, max), c| (min.min(c), max.max(c))))
}

pub fn minmaxsum<'a, T, I: IntoIterator<Item = &'a T>>(input: I) -> Option<(&'a T, &'a T, T)>
where
    T: Ord + Add<Output = T> + Copy,
{
    let mut i = input.into_iter();
    i.next().map(|x| {
        i.fold((x, x, *x), |(min, max, sum), c| {
            (min.min(c), max.max(c), sum + *c)
        })
    })
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct VecLookup<T>(Vec<Option<T>>);

impl<T> VecLookup<T> {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn ensure_size(&mut self, size: usize) {
        let short = size.saturating_sub(self.0.len());
        self.0.extend((0..short).map(|_| None));
    }
    pub fn insert(&mut self, key: usize, value: T) {
        self.ensure_size(key + 1);
        self.0[key] = Some(value);
    }
    #[must_use]
    pub fn get(&self, key: usize) -> Option<&T> {
        self.0.get(key).and_then(|x| x.as_ref())
    }
    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.0.get_mut(key).and_then(|x| match x {
            Some(r) => Some(r),
            None => None,
        })
    }
    #[must_use]
    pub fn contains_key(&self, key: usize) -> bool {
        self.get(key).is_some()
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        VecLookupIter(self, 0)
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_k, v)| v)
    }
}
impl<'a, T> VecLookup<T> {
    pub fn entry(&'a mut self, key: usize) -> VecLookupEntry<'a, T> {
        if self.contains_key(key) {
            return VecLookupEntry::Occupied(key, self.get_mut(key).unwrap());
        }
        VecLookupEntry::Vacant(key, self)
    }
}
pub struct VecLookupIter<'a, T>(&'a VecLookup<T>, usize);
impl<'a, T> IntoIterator for &'a VecLookup<T> {
    type Item = (usize, &'a T);

    type IntoIter = VecLookupIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        VecLookupIter(self, 0)
    }
}
impl<'a, T> Iterator for VecLookupIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.1 < self.0 .0.len() {
            let here = self.1;
            self.1 += 1;
            match &self.0 .0[here] {
                None => (),
                Some(x) => {
                    return Some((here, x));
                }
            }
        }
        None //out of items in array.
    }
}
impl<T: Default> FromIterator<(usize, T)> for VecLookup<T> {
    fn from_iter<TI: IntoIterator<Item = (usize, T)>>(iter: TI) -> Self {
        let mut a = Self::default();
        for (ix, t) in iter {
            a.insert(ix, t);
        }
        a
    }
}
impl<T> Index<usize> for VecLookup<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0[index].as_ref().unwrap()
    }
}

pub enum VecLookupEntry<'a, T: 'a> {
    Occupied(usize, &'a mut T),
    Vacant(usize, &'a mut VecLookup<T>),
}
impl<'a, T: Default> VecLookupEntry<'a, T> {
    #[must_use]
    pub fn or_default(self) -> &'a mut T {
        match self {
            VecLookupEntry::Occupied(_, r) => r,
            VecLookupEntry::Vacant(k, v) => {
                v.insert(k, T::default());
                v.get_mut(k).unwrap()
            }
        }
    }
    pub fn or_insert_with<F>(self, f: F) -> &'a mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            VecLookupEntry::Occupied(_, r) => r,
            VecLookupEntry::Vacant(k, v) => {
                v.insert(k, f());
                v.get_mut(k).unwrap()
            }
        }
    }
}

/// return the index of the first element in sorted_haystack which is >= needle.
pub fn bisect<T: Ord>(sorted_haystack: &[T], needle: T) -> usize {
    let mut lo = 0;
    let mut hi = sorted_haystack.len();
    while hi > lo {
        let c = (hi + lo) / 2;
        match sorted_haystack[c].cmp(&needle) {
            Ordering::Less => lo = c + 1,
            Ordering::Equal => return c,
            Ordering::Greater => hi = c,
        }
    }
    lo
}
