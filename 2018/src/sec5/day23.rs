use itertools::Itertools;
use nom::lib::std::collections::BinaryHeap;
use nom::lib::std::hash::Hash;
use num::abs;
use regex::Regex;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};

type N = i64;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Nanobot {
    pos: [N; 3],
    r: N,
}

impl Nanobot {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"^pos=<([-\d]+),([-\d]+),([-\d]+)>, r=(\d+)$").expect("bad regex");
        let c = re.captures(input.trim()).expect("no capture");
        Self {
            pos: [
                c[1].parse().unwrap(),
                c[2].parse().unwrap(),
                c[3].parse().unwrap(),
            ],
            r: c[4].parse().unwrap(),
        }
    }
    fn distance_to(&self, other: &Self) -> N {
        (0..=2).map(|i| abs(self.pos[i] - other.pos[i])).sum()
    }
    fn distance_to_origin(&self) -> N {
        self.pos.iter().map(|x| abs(*x)).sum()
    }
    fn can_see(&self, other: &Self) -> bool {
        self.distance_to(other) <= self.r
    }
    fn shares_space_with(&self, other: &Self) -> bool {
        self.distance_to(other) <= self.r + other.r
    }
}

#[aoc(day23, part1)]
fn p1(input: &str) -> usize {
    let bots = input.lines().map(|x| Nanobot::parse(x)).collect_vec();
    let best = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter().filter(|&b| best.can_see(b)).count()
}

fn bron_kerbosch<'a, T>(neighbours: &HashMap<&'a T, HashSet<&'a T>>) -> HashSet<&'a T>
where
    T: Eq + Hash,
{
    let mut best: Cell<HashSet<&T>> = Cell::new(HashSet::new());
    fn recur<'a, 'b, T>(
        neighbours: &'a HashMap<&'b T, HashSet<&'b T>>,
        best: &'a mut Cell<HashSet<&'b T>>,
        p: HashSet<&'b T>,
        r: HashSet<&'b T>,
        x: HashSet<&'b T>,
    ) where
        T: Eq + Hash,
    {
        if p.is_empty() && x.is_empty() {
            //r is maximal... is it maximum?
            let best_set = best.get_mut();
            if r.len() > best_set.len() {
                *best_set = r;
            }
        } else {
            let most_neighbours_of_p_and_x =
                p.union(&x).max_by_key(|&&x| neighbours[x].len()).unwrap();
            for n in p.difference(&neighbours[most_neighbours_of_p_and_x]) {
                let ns = &neighbours[n];
                let mut new_r = r.clone();
                new_r.insert(*n);
                recur(
                    neighbours,
                    best,
                    p.intersection(ns).copied().collect(),
                    new_r,
                    x.intersection(ns).copied().collect(),
                );
            }
        }
    }
    recur(
        neighbours,
        &mut best,
        neighbours.keys().copied().collect(),
        HashSet::new(),
        HashSet::new(),
    );
    best.into_inner()
}

#[aoc(day23, part2, wrong_but_i_dont_know_why)]
fn p2(input: &str) -> N {
    let bots = input.lines().map(|x| Nanobot::parse(x)).collect_vec();
    let mut shares_space_with: HashMap<&Nanobot, HashSet<&Nanobot>> = HashMap::new();
    for v in bots.iter().combinations(2) {
        let a = v[0];
        let b = v[1];
        if a.shares_space_with(b) {
            shares_space_with
                .entry(&a)
                .or_insert_with(HashSet::new)
                .insert(b);
            shares_space_with
                .entry(&b)
                .or_insert_with(HashSet::new)
                .insert(a);
        }
    }
    let maximum_clique = bron_kerbosch(&shares_space_with);
    maximum_clique
        .iter()
        .map(|x| x.distance_to_origin() - x.r)
        .max()
        .unwrap()
}

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug)]
struct Cube {
    lo: Vec<N>,
    hi: Vec<N>,
}
impl Cube {
    fn distance_to_point(&self, p: &[N]) -> N {
        assert_eq!(p.len(), 3);
        (0..=2)
            .map(|i| {
                let target = p[i];
                let cube_lo = self.lo[i];
                let cube_hi = self.hi[i];
                if target < cube_lo {
                    cube_lo - target
                } else if target > cube_hi {
                    target - cube_hi
                } else {
                    0
                }
            })
            .sum()
    }
    fn distance_to_origin(&self) -> N {
        self.distance_to_point(&[0, 0, 0])
    }
    fn intersects(&self, n: &Nanobot) -> bool {
        self.distance_to_point(&n.pos) <= n.r
    }
    fn new_cube_about_origin(half_width: N) -> Self {
        Cube {
            lo: vec![-half_width, -half_width, -half_width],
            hi: vec![half_width, half_width, half_width],
        }
    }
    fn size(&self) -> N {
        self.hi[0] - self.lo[0] + 1
    }
    fn octants(&self) -> Vec<Self> {
        let new_w = self.size() / 2;
        (0..3)
            .map(|_| (0..=1))
            .multi_cartesian_product()
            .map(|v| {
                let new_lo: Vec<i64> = self
                    .lo
                    .iter()
                    .zip(v.into_iter())
                    .map(|(l, d)| l + d * new_w)
                    .collect();
                let new_hi = new_lo.iter().map(|x| x + new_w - 1).collect();
                Self {
                    hi: new_hi,
                    lo: new_lo,
                }
            })
            .collect()
    }
}

#[aoc(day23, part2, oct_search)]
fn p2b(input: &str) -> N {
    let bots = input.lines().map(|x| Nanobot::parse(x)).collect_vec();
    let max_coord = *bots.iter().flat_map(|x| x.pos.iter()).max().unwrap();
    let mut power_2_bound = 1;
    while power_2_bound <= max_coord {
        power_2_bound *= 2;
    }
    let init_cube = Cube::new_cube_about_origin(power_2_bound);
    let mut work = BinaryHeap::new(); //MAX heap.
                                      //sort by MOST Bots, then BIGGEST size, then smallest distance to origin.
    work.push((
        bots.len(),
        init_cube.size(),
        -init_cube.distance_to_origin(),
        init_cube,
    ));
    while !work.is_empty() {
        let (_, sz, _, c) = work.pop().unwrap();
        if sz <= 1 {
            return c.distance_to_origin();
        }
        for oct in c.octants() {
            let reach = bots.iter().filter(|b| oct.intersects(&b)).count();
            work.push((reach, oct.size(), -oct.distance_to_origin(), oct));
        }
    }
    0
}
