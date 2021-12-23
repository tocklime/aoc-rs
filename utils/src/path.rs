use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use num::Zero;

pub fn run_dijkstra<N, C, FN, IN, FS>(start: &N, successors: &mut FN, stop: &mut FS) -> Option<C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see: BinaryHeap<SmallestHolder<C, N>> = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        elem: start.clone(),
    });
    //map of node -> index and smallest cost.
    let mut minima: HashMap<N, C> = HashMap::new();
    minima.insert(start.clone(), Zero::zero());
    while let Some(SmallestHolder { cost, elem }) = to_see.pop() {
        if stop(&elem) {
            return Some(cost);
        }
        let successors = {
            if let Some(c) = minima.get(&elem) {
                // We may have inserted a node several time into the binary heap if we found
                // a better way to access it. Ensure that we are currently dealing with the
                // best path and discard the others.
                if &cost > c {
                    continue;
                }
            }
            successors(&elem)
        };
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            match minima.entry(successor.clone()) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if e.get() > &new_cost {
                        e.insert(new_cost);
                    } else {
                        continue;
                    }
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(new_cost);
                }
            }

            to_see.push(SmallestHolder {
                cost: new_cost,
                elem: successor,
            });
        }
    }
    None
}

struct SmallestHolder<K, N> {
    cost: K,
    elem: N,
}

impl<K: PartialEq, N> PartialEq for SmallestHolder<K, N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<K: PartialEq, N> Eq for SmallestHolder<K, N> {}

impl<K: Ord, N> PartialOrd for SmallestHolder<K, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, N> Ord for SmallestHolder<K, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
