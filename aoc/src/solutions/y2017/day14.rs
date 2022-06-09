use aoc_harness::aoc_main;

aoc_main!(2017 day 14, part1 [p1], part2 [p2]);
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use std::collections::HashSet;
use std::convert::TryInto;
use std::str::FromStr;
use utils::cartesian::Point;
use utils::knot_hash::KnotHash;

fn p1(input: &str) -> u64 {
    (0..128)
        .map::<u64, _>(|l| {
            KnotHash::from_str(&format!("{}-{}", input, l))
                .unwrap()
                .set_bit_count()
                .into()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let mut set: HashSet<Point<u8>> = HashSet::new();
    for l in 0..128 {
        let v: Vec<u8> = KnotHash::from_str(&format!("{}-{}", input, l))
            .unwrap()
            .bit_arr();
        for (ix, x) in BitVec::<u8, Msb0>::from_slice(&v).into_iter().enumerate() {
            if x {
                set.insert(Point::new(ix.try_into().unwrap(), l));
            }
        }
    }
    let mut set_count = 0;
    while !set.is_empty() {
        let mut fringe = HashSet::new();
        fringe.insert(*set.iter().next().unwrap());
        #[allow(clippy::unnecessary_to_owned)]
        while !fringe.is_empty() {
            for x in &fringe {
                set.remove(x);
            }
            fringe = fringe
                .iter()
                .flat_map(|x| x.neighbours().to_vec().into_iter())
                .filter(|x| set.contains(x))
                .collect();
        }
        set_count += 1;
    }
    set_count
}
