use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::AddAssign;

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
