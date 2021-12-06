use aoc_harness::*;
use utils::nums::NumBitExt;

aoc_main!(2021 day 3, generator gen, part1 [p1] => 4001724, part2 [p2] => 587895,
          example both EG => (198, 230));

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
fn ones_count_at_pos(nums: &[usize], pos: usize) -> usize {
    nums.iter().filter(|x| x.get_bit(pos)).count()
}
fn p1(input: &Day3) -> usize {
    let epsilon = (0..input.width).fold(0, |epsilon, ix| {
        let ones_count = ones_count_at_pos(&input.nums, ix);
        //there's more ones than zeroes iff 2 * ones_count is bigger than the list.
        epsilon.with_set_bit(ix, 2 * ones_count > input.nums.len())
    });
    let gamma = !epsilon & ((1 << input.width) - 1);
    epsilon * gamma
}
fn filter_on(input: &[usize], width: usize, prefer_ones: bool) -> usize {
    let mut list = input.to_vec();
    let mut ix = width;
    while list.len() > 1 {
        ix -= 1;
        let pos_1s = ones_count_at_pos(&list, ix);
        let target_bit_value = prefer_ones ^ (2 * pos_1s < list.len());
        list.retain(|x| x.get_bit(ix) == target_bit_value);
    }
    list[0]
}

fn p2(input: &Day3) -> usize {
    let oxy = filter_on(&input.nums, input.width, true);
    let co2 = filter_on(&input.nums, input.width, false);
    co2 * oxy
}
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
