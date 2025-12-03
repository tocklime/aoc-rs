use std::{collections::BTreeSet, iter::from_fn};

use nom::{
    Parser, bytes::complete::tag, character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};
use num_traits::Euclid;
use utils::nom::NomError;

aoc_harness::aoc_main!(2025 day 2, generator parse, part1 [p1] => 15_873_079_081, part2 [p2, p2a] => 22_617_871_034, example part1 EG => 1_227_775_554, example part2 "1-99" => 495, example part2 EG => 4_174_379_265);

fn parse(input: &str) -> Vec<(u64, u64)> {
    all_consuming::<_, NomError, _>(separated_list1(
        tag(","),
        separated_pair(complete::u64, tag("-"), complete::u64),
    ))
    .parse(input.trim_end())
    .unwrap()
    .1
}

fn p1(pairs: &[(u64, u64)]) -> u64 {
    let mut total = 0;
    for &(a, b) in pairs {
        let a_len = a.ilog10() + 1;
        let b_len = b.ilog10() + 1;
        for candidate_len in a_len.div_ceil(2)..=b_len / 2 {
            let code_low = a / 10u64.pow(candidate_len);
            let code_hi = b / 10u64.pow(candidate_len);
            for code_candidate in code_low.max(1)..=code_hi {
                let code = code_candidate * 10u64.pow(code_candidate.ilog10() + 1) + code_candidate;
                if (a..=b).contains(&code) {
                    total += code;
                }
            }
        }
    }
    total
}
fn p2a(pairs: &[(u64, u64)]) -> u64 {
    let mut invalid_ids = BTreeSet::new();
    for &(a, b) in pairs {
        let a_len = a.ilog10() + 1;
        let b_len = b.ilog10() + 1;
        for solution_length in a_len..=b_len {
            for group_count in 2..=solution_length {
                //want group_count groups.
                let (repeat_len, m) = solution_length.div_rem_euclid(&group_count);
                if m == 0 {
                    //want group_count groups of length repeat_len.
                    let divisor = 10u64.pow(a_len - repeat_len);
                    let rep_min: u64 = a / divisor;
                    let rep_max: u64 = b / divisor;
                    for r in rep_min.max(1)..=rep_max {
                        let r = r / (10u64.pow(r.ilog10() + 1 - repeat_len));
                        let mut cand = r;
                        let cand_len = r.ilog10() + 1;
                        let multiplier = 10u64.pow(cand_len);
                        let cands = from_fn(move || {
                            cand = cand * multiplier + r;
                            Some(cand)
                        });
                        invalid_ids.extend(cands.skip_while(|&x| x < a).take_while(|&x| x <= b));
                    }
                }
            }
        }
    }
    invalid_ids.into_iter().sum::<u64>()
}

fn p2(pairs: &[(u64, u64)]) -> u64 {
    let mut invalid_ids = BTreeSet::new();
    for &(a, b) in pairs {
        let a_len = a.ilog10() + 1;
        for repeat_len in 1..=a_len.div_ceil(2) {
            // println!("Doing repeats of len {repeat_len} between {a} and {b}");
            let divisor = 10u64.pow(a_len - repeat_len);
            let rep_min: u64 = a / divisor;
            let rep_max: u64 = b / divisor;
            // println!("That will be {rep_min} to {rep_max}");
            for r in rep_min.max(1)..=rep_max {
                let r = r / (10u64.pow(r.ilog10() + 1 - repeat_len));
                let mut cand = r;
                let cand_len = r.ilog10() + 1;
                let multiplier = 10u64.pow(cand_len);
                let cands = from_fn(move || {
                    cand = cand * multiplier + r;
                    Some(cand)
                });
                invalid_ids.extend(cands.skip_while(|&x| x < a).take_while(|&x| x <= b));
            }
        }
    }
    invalid_ids.into_iter().sum::<u64>()
}

const EG: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
