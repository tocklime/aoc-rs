use aoc_harness::aoc_main;

aoc_main!(2020 day 9, generator gen, part1 [p1], part2 [p2, p2_no_ps, p2_just_fast, p2_other]);

use itertools::Itertools;
use utils::collections::prefix_sum;

fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}
const WIN_SIZE: usize = 25;

fn p1(is: &[usize]) -> Option<usize> {
    is.windows(WIN_SIZE).enumerate().find_map(|(ix, w)| {
        let target = is[WIN_SIZE + ix];
        if w.iter().all(|&x| !w.contains(&target.wrapping_sub(x))) {
            Some(target)
        } else {
            None
        }
    })
}

fn p2(is: &[usize]) -> Option<usize> {
    let target = p1(is)?;
    let ps = prefix_sum(is).collect::<Vec<usize>>();
    //for each possible start index..
    (0..is.len()).find_map(|ix_1| {
        //find an end index such that..
        (ix_1..is.len())
            //(optimisation: give up once the difference is too big)
            .take_while(|&ix| ps[ix] - ps[ix_1] <= target)
            // ..such that the difference on the prefix sum array ==target (that is, is[ix_1..=ix_2].sum() == target)
            .find_map(|ix_2| {
                if ps[ix_2] - ps[ix_1] == target {
                    let (&a, &b) = is[ix_1 + 1..=ix_2].iter().minmax().into_option()?;
                    Some(a + b)
                } else {
                    None
                }
            })
    })
}

fn p2_other(is: &[usize]) -> Option<usize> {
    let target = p1(is)?;
    let ps = prefix_sum(is).collect::<Vec<usize>>();
    //for each possible window size
    (2..=is.len()).find_map(|win_size| {
        //find a start index such that..
        (0..is.len() - win_size)
            // ..such that the difference on the prefix sum array ==target (that is, is[ix_1..=ix_2].sum() == target)
            .find_map(|ix_1| {
                if ps[ix_1 + win_size] - ps[ix_1] == target {
                    let (&a, &b) = (is[ix_1 + 1..=ix_1 + win_size].iter())
                        .minmax()
                        .into_option()?;
                    Some(a + b)
                } else {
                    None
                }
            })
    })
}

fn p2_no_ps(is: &[usize]) -> Option<usize> {
    let target = p1(is)?;
    //for each possible window size
    let window = (2..=is.len()).find_map(|win_size|
        //find a window such that the sum equals the target.
        is.windows(win_size).find(|w| w.iter().sum::<usize>() == target))?;
    let (min, max) = window.iter().minmax().into_option()?;
    Some(min + max)
}

fn p2_just_fast(is: &[usize]) -> Option<usize> {
    let target = p1(is)?;
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    while sum != target {
        while sum < target {
            sum += is[end];
            end += 1;
        }
        while sum > target {
            sum -= is[start];
            start += 1;
        }
    }
    let (min, max) = is[start..end].iter().minmax().into_option()?;
    Some(min + max)
}
