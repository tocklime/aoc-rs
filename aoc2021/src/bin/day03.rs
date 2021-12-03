use aoc_harness::*;
use utils::nums::NumBitExt;

aoc_main!(2021 day 3, generator gen, [p1] => 4001724, [p2] => 587895);

struct Day3 {
    width: usize,
    nums: Vec<usize>,
}
fn gen(input: &str) -> Day3 {
    let nums = input
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();
    let width = input.lines().next().unwrap().chars().count();
    Day3 { nums, width }
}
fn p1(input: &Day3) -> usize {
    let mut ones_counts = vec![0; input.width];
    for x in &input.nums {
        for (ix, count) in ones_counts.iter_mut().enumerate() {
            *count += x.get_bit(ix) as usize;
        }
    }
    let mut epsilon = 0;
    for (ix, &ones) in ones_counts.iter().enumerate() {
        epsilon.set_bit(ix, 2 * ones > input.nums.len());
    }
    let gamma = !epsilon & ((1 << input.width) - 1);
    epsilon * gamma
}
fn filter_on(input: &[usize], width: usize, prefer_ones: bool) -> usize {
    let mut list = input.to_vec();
    for ix in 0..=width {
        if list.len() <= 1 {
            return list[0];
        }
        let bit_ix = width - 1 - ix;
        let pos_1s = list.iter().filter(|&&x| x.get_bit(bit_ix)).count();
        let target_bit_value = prefer_ones ^ (2 * pos_1s < list.len());
        list.retain(|x| x.get_bit(bit_ix) == target_bit_value);
    }
    unreachable!()
}
fn p2(input: &Day3) -> usize {
    let oxy = filter_on(&input.nums, input.width, true);
    let co2 = filter_on(&input.nums, input.width, false);
    co2 * oxy
}

#[cfg(test)]
mod tests {
    use super::*;

    const EG: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn tp1() {
        assert_eq!(p1(&gen(EG)), 198);
    }
    #[test]
    pub fn tp2() {
        assert_eq!(p2(&gen(EG)), 230);
    }
}
