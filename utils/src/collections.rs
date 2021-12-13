use std::hash::Hash;
use std::ops::AddAssign;
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
