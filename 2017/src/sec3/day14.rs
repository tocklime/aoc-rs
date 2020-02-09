use crate::utils::knot_hash::KnotHash;
use crate::utils::cartesian::Point;
use std::collections::HashSet;
use bitvec::vec::BitVec;
use bitvec::order::Msb0;
use std::convert::TryInto;

#[aoc(day14, part1)]
fn p1(input: &str) -> u64 {
    (0..128).map::<u64, _>(|l| KnotHash::from_str(&format!("{}-{}", input, l)).set_bit_count().into()).sum()
}

#[aoc(day14, part2)]
fn p2(input: &str) -> usize {
    let mut set: HashSet<Point<u8>> = HashSet::new();
    for l in 0..128 {
        let v = KnotHash::from_str(&format!("{}-{}", input, l)).bit_arr();
        for (ix, x) in v.into_iter().flat_map(|x| BitVec::<Msb0, u8>::from_element(x)).enumerate() {
            if x {
                set.insert(Point::new(ix.try_into().unwrap(), l));
            }
        }
    }
    let mut set_count = 0;
    while !set.is_empty() {
        let mut fringe = HashSet::new();
        fringe.insert(*set.iter().nth(0).unwrap());
        while !fringe.is_empty() {
            fringe.iter().for_each(|x|{set.remove(x);});
            fringe = fringe.iter()
                .flat_map(|x| x.neighbours().to_vec().into_iter())
                .filter(|x| set.contains(x))
                .collect();
        }
        set_count+=1;
    }
    set_count
}