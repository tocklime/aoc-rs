use itertools::Itertools;
use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<usize> {
    let mut a: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    a.push(0);
    a.sort_unstable();
    a.push(a.last().unwrap() + 3);
    a
}

#[aoc(day10, part1)]
pub fn p1(input: &[usize]) -> usize {
    let (a, b) = input
        .iter()
        .tuple_windows()
        .fold((0, 0), |(a, b), (c, d)| match d - c {
            1 => (a + 1, b),
            3 => (a, b + 1),
            _ => panic!("no"), //(a, b),
        });
    a * b
}

pub fn count_routes(
    memo: &mut HashMap<usize, usize>,
    map: &HashSet<usize>,
    from: usize,
    to: usize,
) -> usize {
    match (memo.get(&from), from.cmp(&to)) {
        (Some(&x), _) => x,
        (_, Ordering::Greater) => 0,
        (_, Ordering::Equal) => 1,
        _ => {
            let a = (from + 1..=from + 3)
                .filter_map(|m| map.get(&m).map(|&x| count_routes(memo, map, x, to)))
                .sum();
            memo.insert(from, a);
            a
        }
    }
}

#[aoc(day10, part2)]
pub fn p2(input: &[usize]) -> usize {
    let mut xs: HashSet<usize> = input.iter().copied().collect();
    let target = xs.iter().max().unwrap() + 3_usize;
    xs.insert(target);
    count_routes(&mut HashMap::new(), &xs, 0, target)
}

#[aoc(day10, part2, dp)]
pub fn p2_dp(input: &[usize]) -> usize {
    let mut dp = vec![0; input.len()];
    //the value in dp[ix] is the number of ways to get to input[ix].
    dp[0] = 1;
    for (ix, val) in input.iter().enumerate() {
        //now at index ix, and there's dp[ix] ways to get here.
        //for each of the next 3 values <= val + 3, add dp[ix] to the ways to get there.
        for offset in (1..=3).take_while(|&offset| input.get(ix + offset).map_or(false, |&x| x <= val + 3))
        {
            dp[ix + offset] += dp[ix];
        }
    }
    //answer is at the end of dp.
    *dp.last().unwrap()
}

#[aoc(day10, part2, dp_deque)]
pub fn p2_dp_deque(input: &[usize]) -> usize {
    let mut dp : VecDeque<usize> = [1,0,0,0].iter().copied().collect();
    //the values in dp are the number of ways to get to the next 3 indexes from here.
    let mut last = 1;
    for (ix, val) in input.iter().enumerate() {
        //now at index ix, and there's dp[ix] ways to get here.
        //for each of the next 3 values <= val + 3, add dp[ix] to the ways to get there.
        last = dp.pop_front().unwrap();
        dp.push_back(0);
        for offset in (0..3).take_while(|&offset| input.get(ix + 1 + offset).map_or(false, |&x| x <= val + 3))
        {
            dp[offset] += last;
        }
    }
    //answer is at the end of dp.
    last
}
