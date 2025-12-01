use itertools::Itertools;
use num::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{inputs::parse_input_from_str_sep_by, nums};

aoc_harness::aoc_main!(2024 day 7, 
    generator gen, 
    part1 [integer_bits::<1>, cartesian_product::<1>, pathfinding_dfs::<1>, manual_dfs::<1>, manual_dfs_backward::<1>] => 2_299_996_598_890, 
    part2 [integer_bits::<2>, cartesian_product::<2>, pathfinding_dfs::<2>, manual_dfs::<2>,manual_dfs_backward::<2>] => 362_646_859_298_554, 
    example part1 EG => 3749, example part2 EG => 11387);

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}
impl Op {
    fn from_bit(n: u64) -> Self {
        match n {
            0 => Self::Add,
            1 => Self::Mul,
            2 => Self::Concat,
            _ => unimplemented!(),
        }
    }
    fn ap(self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Concat => {
                let m = 10u64.pow(nums::digit_count(b) as u32);
                a * m + b
            }
        }
    }
    fn ap_rev(self, a: u64, b: u64) -> Option<u64> {
        match self {
            Op::Add => a.checked_sub(b),
            Op::Mul => a.is_multiple_of(b).then_some(a / b),
            Op::Concat => {
                let m = 10u64.pow(nums::digit_count(b) as u32);
                let c = a.checked_sub(b)?;
                (c % m == 0).then_some(c / m)
            }
        }
    }
}

fn check_p1(target: u64, nums: &[u64], operator_choice: u64, base: u64) -> bool {
    let mut n_i = nums.iter();
    let mut total = *n_i.next().unwrap();
    let mut choices = operator_choice;
    for &n in n_i {
        let (d, m) = choices.div_rem(&base);
        let op = Op::from_bit(m);
        choices = d;
        total = op.ap(total, n);
    }
    total == target
}
fn check_p2(target: u64, nums: &[u64], ops: &[&Op]) -> bool {
    let mut n_i = nums.iter();
    let start = *n_i.next().unwrap();
    let total = n_i.zip(ops.iter()).fold(start, |a, (e, o)| o.ap(a, *e));
    total == target
}

fn integer_bits<const PART: u64>(input: &[(u64, Vec<u64>)]) -> u64 {
    let choices = PART + 1;

    input
        .iter()
        .filter_map(|(target, nums)| {
            let gap_count = nums.len() - 1;
            let max_n = choices.pow(gap_count as u32);
            (0..max_n)
                .any(|n| check_p1(*target, nums, n, choices))
                .then_some(target)
        })
        .sum()
}
const OPS: [Op; 2] = [Op::Add, Op::Mul];
const OPS2: [Op; 3] = [Op::Add, Op::Mul, Op::Concat];
fn gen(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (target, nums) = l.split_once(": ").unwrap();
            let target: u64 = target.parse().unwrap();
            let nums: Vec<u64> = parse_input_from_str_sep_by(nums, " ");
            (target, nums)
        })
        .collect()
}
fn cartesian_product<const PART: u8>(input: &[(u64, Vec<u64>)]) -> u64 {
    let ops: &[Op] = match PART {
        1 => &OPS,
        2 => &OPS2,
        _ => unreachable!(),
    };
    input
        .iter()
        .filter_map(|(target, nums)| {
            let gap_count = nums.len() - 1;
            std::iter::repeat_n(ops, gap_count)
                .multi_cartesian_product()
                .any(|s| check_p2(*target, nums, &s))
                .then_some(target)
        })
        .sum()
}
fn find_solution(target: u64, nums: &[u64], max_op: u64) -> bool {
    let path = pathfinding::directed::dfs::dfs(
        (nums[0], 1),
        |&(total, ix)| {
            (0..=max_op)
                .map(move |n| (Op::from_bit(n).ap(total, nums[ix]), ix + 1))
                .filter(|&(total, ix)| {
                    if ix == nums.len() {
                        total == target
                    } else {
                        total <= target
                    }
                })
        },
        |&(_, pos)| pos == nums.len(),
    );
    path.is_some()
}
fn pathfinding_dfs<const PART: u64>(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        .filter_map(|(target, nums)| find_solution(*target, nums, PART).then_some(target))
        .sum()
}
fn manual_dfs<const PART: u64>(input: &[(u64, Vec<u64>)]) -> u64 {
    let ops: &[Op] = match PART {
        1 => &OPS,
        2 => &OPS2,
        _ => unreachable!(),
    };
    input
        .par_iter()
        .filter_map(|(target, nums)| {
            let mut stack = vec![(nums[0], 1)];
            while let Some((total, ix)) = stack.pop() {
                if ix == nums.len() {
                    if *target == total {
                        return Some(target);
                    }
                } else if total <= *target {
                    stack.extend(ops.iter().map(|o| (o.ap(total, nums[ix]), ix + 1)));
                }
            }
            None
        })
        .sum()
}
fn manual_dfs_backward<const PART: u64>(input: &[(u64, Vec<u64>)]) -> u64 {
    let ops: &[Op] = match PART {
        1 => &OPS,
        2 => &OPS2,
        _ => unreachable!(),
    };
    input
        .par_iter()
        .filter_map(|(target, nums)| {
            let mut stack = vec![(*target, nums.len() - 1)];
            while let Some((total, ix)) = stack.pop() {
                if ix == 0 {
                    if total == nums[0] {
                        return Some(target);
                    }
                } else {
                    stack.extend(
                        ops.iter()
                            .filter_map(|o| o.ap_rev(total, nums[ix]).map(|n| (n, ix - 1))),
                    );
                }
            }
            None
        })
        .sum()
}

const EG: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
