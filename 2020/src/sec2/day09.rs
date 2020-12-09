use crate::utils::collections::{minmax, prefix_sum};

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}
const WIN_SIZE : usize = 25;

#[aoc(day9, part1)]
pub fn p1(is: &[usize]) -> Option<usize> {
    is.windows(WIN_SIZE).enumerate().find_map(|(ix, w)| {
        let target = is[WIN_SIZE + ix];
        if w.iter().all(|x| !w.contains(&(target - x))) {
            Some(target)
        } else {
            None
        }
    })
}

#[aoc(day9, part2)]
pub fn p2(is: &[usize]) -> Option<usize> {
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
                    let (&a, &b) = minmax(&is[ix_1 + 1..=ix_2]).unwrap();
                    Some(a + b)
                } else {
                    None
                }
            })
    })
}

#[aoc(day9, part2,other)]
pub fn p2_other(is: &[usize]) -> Option<usize> {
    let target = p1(is)?;
    let ps = prefix_sum(is).collect::<Vec<usize>>();
    //for each possible window size
    (2..=is.len()).find_map(|win_size| {
        //find a start index such that..
        (0..is.len() - win_size)
            // ..such that the difference on the prefix sum array ==target (that is, is[ix_1..=ix_2].sum() == target)
            .find_map(|ix_1| {
                if ps[ix_1+win_size] - ps[ix_1] == target {
                    let (&a, &b) = minmax(&is[ix_1 + 1..=ix_1 + win_size])?;
                    Some(a + b)
                } else {
                    None
                }
            })
    })
}
