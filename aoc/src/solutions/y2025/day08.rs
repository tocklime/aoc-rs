use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::separated_list1,
};
use utils::nom::NomError;

aoc_harness::aoc_main!(2025 day 8, generator generate, both [both] => (96672, 22_517_595), example both EG => (40,25272));

fn dist2(a: &[u32;3], b: &[u32;3]) -> u64 {
   (0..3).map(|d| a[d].abs_diff(b[d]) as u64).map(|x| x*x).sum::<u64>()
}

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
    coords: Vec<[u32;3]>,
    dists: BTreeMap<u64, Vec<(usize,usize)>>
}

fn generate(input: &str) -> Problem {
    let iters : usize = if input == EG { 10 } else { 1000 };
    let coords = all_consuming(separated_list1(
        newline::<_, NomError>,
        separated_list1(tag(","), complete::u32).map(|v| [v[0], v[1], v[2]]),
    ))
    .parse(input.trim())
    .unwrap()
    .1;
    //compute distance of all pairs
    let mut dists : BTreeMap<u64, Vec<(usize,usize)>> = BTreeMap::default();
    for (a,b) in (0..coords.len()).cartesian_product(0..coords.len()) {
        if a < b {
            let d = dist2(&coords[a], &coords[b]);
            dists.entry(d).or_default().push((a,b));
        }
    }
    Problem { iters, coords, dists }
}

fn both(p: &Problem) -> (usize,u64) {
    let junction_box_count = p.coords.len();
    let dist_iter = p.dists.iter().flat_map(|(dist,v)| v.iter().map(move |p| (dist,p)));
    let mut joined : BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut owners: BTreeMap<usize, usize> = BTreeMap::new();
    let mut p1 = 0;
    let mut skipped_join = 0;
    for ((_d, &(a,b)), count) in dist_iter.zip(1..) {
        validate(&owners, &joined);
        if count == p.iters + 1 {
           p1 = joined.iter().map(|x| x.1.len() + 1).sorted_unstable().rev().take(3).product();
        }
        // println!("Joining {a} ({:?}) and {b} ({:?}) with dist2 {_d}", &coords[a], &coords[b]);
        //want a list of minimum-index to ordered list of nodes.
        //guarantee a < b.
        //but maybe a is already in something?
        let a_owner = *owners.get(&a).unwrap_or(&a);
        let b_owner = *owners.get(&b).unwrap_or(&b);
        // println!("a {a} -> {a_owner}, b {b} -> {b_owner}");
        assert!(!owners.contains_key(&a_owner));
        assert!(!owners.contains_key(&b_owner));
        // owners.insert(b,a);
        // joined.entry(a).or_default().insert(b);
        match a_owner.cmp(&b_owner) {
            std::cmp::Ordering::Less => {
                //a_owner now owns all.
                let take = joined.remove(&b_owner);
                owners.insert(b, a_owner);
                let rcv = joined.entry(a_owner).or_default();
                rcv.insert(b);
                rcv.insert(b_owner);
                owners.insert(b_owner, a_owner);
                for &ob in take.iter().flatten() {
                    rcv.insert(ob);
                    owners.insert(ob, a_owner);
                }
                // println!("{a_owner} takes all");
            }
            std::cmp::Ordering::Equal => {
                //already in same set.
                skipped_join += 1;
                // println!("No change");
            }
            std::cmp::Ordering::Greater => {
                //b_owner now owns all.
                let take = joined.remove(&a_owner);
                owners.insert(a, b_owner);
                let rcv = joined.entry(b_owner).or_default();
                rcv.insert(a);
                rcv.insert(a_owner);
                owners.insert(a_owner, b_owner);
                for &oa in take.iter().flatten() {
                    rcv.insert(oa);
                    owners.insert(oa, b_owner);
                }
                // println!("{b_owner} takes all");
            }
        }
        // println!("Done {count} joins, {skipped_join} of which were noops. That's {} real joins. Total box count is {junction_box_count}.", count - skipped_join);
        if count - skipped_join == junction_box_count - 1 { 
            // println!("Done.");
            let p2 = p.coords[a][0] as u64 * p.coords[b][0] as u64;
            return (p1, p2)
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
