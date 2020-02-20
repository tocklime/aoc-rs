use std::collections::HashMap;
use std::ops::AddAssign;
use std::hash::Hash;

pub trait ToLookup<K,V> : Iterator {
    fn collect_lookup(&mut self) -> HashMap<K,Vec<V>>;
}

impl<K,V, I : Iterator<Item=(K,V)>> ToLookup<K,V> for I
    where K : Hash + Eq
{
    fn collect_lookup(&mut self) -> HashMap<K,Vec<V>>
    {
        let mut ans = HashMap::new();
        for (k,v) in self {
            ans.entry(k).or_insert_with( Vec::new).push(v);
        }
        ans
    }
}

pub fn de_prefixsum<T: AddAssign + Default + Copy>(input: &[T]) -> Vec<T> {
    let mut total: T = Default::default();
    let mut ans = Vec::with_capacity(input.len());
    for i in input {
        total += *i;
        ans.push(total);
    }
    return ans;
}
