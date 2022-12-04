use std::collections::HashSet;

use aoc_harness::*;
use utils::span::Span;

aoc_main!(2022 day 4, part1 [p1], part2 [p2], example part1 EG => 2);

// 7-50,8-33
// 76-83,77-87
// 68-73,55-68
// 13-37,12-25
// 7-7,12-96

const EG: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
fn p1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (one, two) = l.split_once(",").unwrap();
            let (one_a, one_b) = one.split_once("-").unwrap();
            let (two_a, two_b) = two.split_once("-").unwrap();
            let oai: usize = one_a.parse().unwrap();
            let obi: usize = one_b.parse().unwrap();
            let tai: usize = two_a.parse().unwrap();
            let tbi: usize = two_b.parse().unwrap();
            let r1 = Span::new(oai, obi + 1);
            let r2 = Span::new(tai, tbi + 1);
            let x = match r1.collide_with(&r2) {
                utils::span::CollisionType::Equal => true,
                utils::span::CollisionType::Before(_) => false,
                utils::span::CollisionType::OverlapsStart(_, _, _) => false,
                utils::span::CollisionType::StrictlyBigger(_, _, _) => true,
                utils::span::CollisionType::StrictlySmaller(_, _, _) => true,
                utils::span::CollisionType::OverlapsEnd(_, _, _) => false,
                utils::span::CollisionType::After(_) => false,
            };
            let y: HashSet<usize> = (oai..=obi).collect();
            let z: HashSet<usize> = (tai..=tbi).collect();
            let slow = y.is_subset(&z) || z.is_subset(&y);
            assert_eq!(x, slow);
            x
        })
        .count()
}
//382, 361 wrong, 465 wrong.

fn p2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (one, two) = l.split_once(",").unwrap();
            let (one_a, one_b) = one.split_once("-").unwrap();
            let (two_a, two_b) = two.split_once("-").unwrap();
            let oai: usize = one_a.parse().unwrap();
            let obi: usize = one_b.parse().unwrap();
            let tai: usize = two_a.parse().unwrap();
            let tbi: usize = two_b.parse().unwrap();
            let r1 = Span::new(oai, obi + 1);
            let r2 = Span::new(tai, tbi + 1);
            let x = match r1.collide_with(&r2) {
                utils::span::CollisionType::Equal => true,
                utils::span::CollisionType::Before(_) => false,
                utils::span::CollisionType::OverlapsStart(_, _, _) => true,
                utils::span::CollisionType::StrictlyBigger(_, _, _) => true,
                utils::span::CollisionType::StrictlySmaller(_, _, _) => true,
                utils::span::CollisionType::OverlapsEnd(_, _, _) => true,
                utils::span::CollisionType::After(_) => false,
            };
            let y: HashSet<usize> = (oai..=obi).collect();
            let z: HashSet<usize> = (tai..=tbi).collect();
            let slow = !y.is_disjoint(&z);
            dbg!(l);
            assert_eq!(x, slow);
            // dbg!(l, one, two, one_a, two_a, r1, r2, x);
            x
        })
        .count()
}
