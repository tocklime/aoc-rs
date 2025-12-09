use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::separated_list1,
};
use utils::{kdtree::KdTree, nom::NomError};

aoc_harness::aoc_main!(2025 day 8, generator generate, both [both] => (96672, 22_517_595), example both EG => (40,25272));

#[allow(dead_code)]
fn dist2(a: &[i64; 3], b: &[i64; 3]) -> u64 {
    (0..3)
        .map(|d| a[d].abs_diff(b[d]))
        .map(|x| x * x)
        .sum::<u64>()
}

#[allow(dead_code)]
fn validate(owners: &BTreeMap<usize,usize>, joined: &BTreeMap<usize,BTreeSet<usize>>) {
    for (k,v) in joined {
        for child in v {
            assert_eq!(owners[child], *k);
        }
    }
    for (child, owner) in owners {
        assert!(joined[owner].contains(child));
    }
}

struct Problem {
    iters: usize,
    coords: Vec<[i64; 3]>,
    kd_tree: KdTree<3, usize>,
    // dists: BTreeMap<u64, Vec<(usize, usize)>>,
}

fn generate(input: &str) -> Problem {
    let iters: usize = if input == EG { 10 } else { 1000 };
    let coords = all_consuming(separated_list1(
        newline::<_, NomError>,
        separated_list1(tag(","), complete::i64).map(|v| [v[0], v[1], v[2]]),
    ))
    .parse(input.trim())
    .unwrap()
    .1;
    let coords_with_ixs = coords.iter().copied().zip(0..).collect_vec();
    let kd_tree = KdTree::from(&coords_with_ixs);
    //compute distance of all pairs
    // let mut dists: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::default();
    // for a in 0..coords.len() {
    //     for b in a + 1..coords.len() {
    //         let d = dist2(&coords[a], &coords[b]);
    //         dists.entry(d).or_default().push((a, b));
    //     }
    // }
    Problem {
        iters,
        kd_tree,
        coords,
        // dists,
    }
}

struct Nearests<'tree> {
    base_point: [i64;3],
    base_ix: usize,
    kd_tree: &'tree KdTree<3, usize>,
    done_so_far: HashSet<[i64;3]>
}
impl<'tree> Nearests<'tree> {
    fn new(tree: &'tree KdTree<3, usize>, point: [i64;3], ix: usize) -> Self {
        Self {
            base_point: point,
            base_ix: ix,
            kd_tree: tree,
            done_so_far: [point].into_iter().collect()
        }
    }
}

impl<'tree> Iterator for Nearests<'tree> {
    type Item = (i64, [i64;3], usize);

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.kd_tree.find_nearest_to(self.base_point, &self.done_so_far);
        if let Some((_,x, _)) = ans {
            self.done_so_far.insert(x);
        }
        ans
    }
}


struct IgnoreOrder<T>(T);
impl<T> PartialEq for IgnoreOrder<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl<T> PartialOrd for IgnoreOrder<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Eq for IgnoreOrder<T> {

}
impl<T> Ord for IgnoreOrder<T> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}


fn both(prob: &Problem) -> (usize, u64) {
    let junction_box_count = prob.coords.len();
    let mut nearests : BTreeSet<_> = prob.coords.iter().enumerate().map(|(ix, point)| {
        let mut ns = Nearests::new(&prob.kd_tree, *point, ix);
        let first = ns.next().unwrap();
        (first, ns.base_ix, IgnoreOrder(ns))
    }).collect();
    // let dist_iter = p
    //     .dists
    //     .iter()
    //     .flat_map(|(dist, v)| v.iter().map(move |p| (dist, p)));
    let mut joined: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut owners: BTreeMap<usize, usize> = BTreeMap::new();
    let mut p1 = 0;
    let mut skipped_join = 0;
    let mut count = 0;
    // println!("Nearests: ");
    // for ((dist, point, a), b, ns) in nearests.iter() {
    //     println!("  dist {dist} point {point:?} ix {a}-->{b} {}", ns.0.base_ix);
        

    // }
    while let Some(((_dist, _p, ix), ix_b, IgnoreOrder(mut ns))) = nearests.pop_first() {
        let a = ix;
        assert_eq!(ns.base_ix, ix_b);
        let b = ix_b;
        if let Some(next) = ns.next() {
            //put this back in the heap for the next nearest.
            nearests.insert((next, ix_b, IgnoreOrder(ns)));
        }
        if a < b {
            //ignore when a >= b. We'll catch it the other way.
            continue;
        }
        // println!("Nearests: ");
        // for ((dist, point, a), b, ns) in nearests.iter().take(5) {
        //     println!("  dist {dist} point {point:?} ix {a}-->{b} {}", ns.0.base_ix);
            

        // }
        count += 1;
        // validate(&owners, &joined);
        if count == prob.iters + 1 {
            p1 = joined
                .iter()
                .map(|x| x.1.len())
                .sorted_unstable()
                .rev()
                .take(3)
                .product();
            // println!("at count == {count}, p1 ans: {p1}");
        }
        // println!("Joining {a} ({:?}) and {b} ({:?}) with dist2 {_dist}", &prob.coords[a], &prob.coords[b]);
        //want a list of minimum-index to ordered list of nodes.
        //guarantee a < b.
        //but maybe a is already in something?
        let a_owner = *owners.get(&a).unwrap_or(&a);
        let b_owner = *owners.get(&b).unwrap_or(&b);
        // println!("a {a} -> {a_owner}, b {b} -> {b_owner}");
        // assert!(!owners.contains_key(&a_owner));
        // assert!(!owners.contains_key(&b_owner));
        if a_owner == b_owner {
            skipped_join += 1;
            // println!("Same owner, skip");
            continue;
        }
        let owner_min = a_owner.min(b_owner);
        let owner_max = a_owner.max(b_owner);
        let take = joined.remove(&owner_max);
        owners.insert(b, owner_min);
        owners.insert(a, owner_min);
        let rcv = joined.entry(owner_min).or_default();
        rcv.insert(a);
        rcv.insert(b);
        rcv.insert(owner_max);
        owners.insert(owner_max, owner_min);
        for &ob in take.iter().flatten() {
            rcv.insert(ob);
            owners.insert(ob, owner_min);
        }
        // println!("Now {owner_min} owns all of that");
        // println!("Done {count} joins, {skipped_join} of which were noops. That's {} real joins. Total box count is {junction_box_count}.", count - skipped_join);
        // println!("owners: {owners:?}");
        // println!("joined: {joined:?}");
        if count - skipped_join == junction_box_count - 1 {
            // println!("Done.");
            let p2 = prob.coords[a][0] as u64 * prob.coords[b][0] as u64;
            return (p1, p2);
        }
    }
    unreachable!()
}

const EG: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

//6528 too low.
