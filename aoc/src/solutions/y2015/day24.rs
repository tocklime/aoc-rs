use aoc_harness::aoc_main;

aoc_main!(2015 day 24, part1 [p1], part2 [p2]);
use itertools::Itertools;

fn can_split_to(items: &[usize], target: usize) -> bool {
    if target == 0 {
        true
    } else if items.is_empty() {
        false
    } else {
        let first = items[0];

        first <= target && can_split_to(&items[1..], target - first)
            || can_split_to(&items[1..], target)
    }
}

fn can_split_to_2(items: &[usize], target_1: usize, target_2: usize) -> bool {
    if target_1 == 0 && target_2 == 0 {
        true
    } else if items.is_empty() {
        false
    } else {
        let first = items[0];

        first <= target_1 && can_split_to_2(&items[1..], target_1 - first, target_2)
            || first <= target_2 && can_split_to_2(&items[1..], target_1, target_2 - first)
            || can_split_to_2(&items[1..], target_1, target_2)
    }
}

fn p1(input: &str) -> Option<usize> {
    let weights = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();
    let total: usize = weights.iter().sum();
    assert_eq!(total % 3, 0);
    let group_weight = total / 3;
    (0..weights.len())
        .filter_map(|size_1| {
            weights
                .iter()
                .permutations(size_1)
                .filter_map(|g| {
                    if g.iter().cloned().sum::<usize>() == group_weight {
                        //weight is right, check g23 splittable.
                        let rest = weights
                            .iter()
                            .cloned()
                            .filter(|n| !g.contains(&n))
                            .collect_vec();
                        if can_split_to(&rest, group_weight) {
                            Some(g.iter().cloned().product::<usize>())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
}

fn p2(input: &str) -> Option<usize> {
    let weights = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();
    let total: usize = weights.iter().sum();
    assert_eq!(total % 4, 0);
    let group_weight = total / 4;
    (0..weights.len())
        .filter_map(|size_1| {
            weights
                .iter()
                .permutations(size_1)
                .filter_map(|g| {
                    if g.iter().cloned().sum::<usize>() == group_weight {
                        //weight is right, check g23 splittable.
                        let rest = weights
                            .iter()
                            .cloned()
                            .filter(|n| !g.contains(&n))
                            .collect_vec();
                        if can_split_to_2(&rest, group_weight, group_weight) {
                            Some(g.iter().cloned().product::<usize>())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .min()
        })
        .next()
}
