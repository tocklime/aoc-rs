

aoc_harness::aoc_main!(2016 day 15, part1 [p1] => 376_777, part2 [p2] => 3_903_937);
use reformation::Reformation;
use itertools::Itertools;

//Disc #2 has 19 positions; at time=0, it is at position 10.
#[derive(Debug, Reformation, PartialEq, Eq)]
#[reformation("Disc #{id} has {positions} positions; at time=0, it is at position {offset}.")]
struct Disc {
    id: usize,
    positions: usize,
    offset: usize,
}

impl Disc {
    fn can_pass_at(&self, t: usize) -> bool {
        (self.offset + t + self.id) % self.positions == 0
    }
}


fn p1(input: &str) -> usize {
    let discs = input.lines().map(|x| Disc::parse(x).unwrap()).collect_vec();
    (0..).find(|&n| discs.iter().all(|d| d.can_pass_at(n))).unwrap()
}


fn p2(input: &str) -> usize {
    let mut discs = input.lines().map(|x| Disc::parse(x).unwrap()).collect_vec();
    discs.push(Disc {id: 7, positions: 11, offset: 0});
    (0..).find(|&n| discs.iter().all(|d| d.can_pass_at(n))).unwrap()
}
