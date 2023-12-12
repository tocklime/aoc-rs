use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    character::complete::{space1, self, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair, combinator::map,
};
use nom_supreme::tag::complete::tag;
use utils::{nom::NomError, nums::NumBitExt, collections::Intersections};

aoc_harness::aoc_main!(2023 day 12, part1 [p1], part2 [p2], example both EG => (21,525152), example part1 EG1 => 10);

fn to_spring(c: char) -> Option<SpringState> {
    match c {
        '?' => None,
        '.' => Some(SpringState::Working),
        '#' => Some(SpringState::Broken),
        _ => panic!("Not a spring: {c}")
    }
}

#[derive(Debug,PartialEq, Eq,Copy,Clone)]
enum SpringState {
    Working,Broken
}
fn draw(choices: u32, input: &[Option<SpringState>]) {
    let mut rem = choices;
    for x in input {
        let c = match x{
            Some(SpringState::Broken) => '#',
            Some(SpringState::Working) => '.',
            None => {
                let a = if rem.get_bit(0) {
                    '#'
                } else {
                    '.'
                };
                rem >>= 1;
                a
            }
        };
        print!("{c}");
    }
}
fn count_possibles(input: &[Option<SpringState>], correct: &[usize]) -> usize {
    let count_of_unknowns = input.iter().filter(|x| x.is_none()).count();
    dbg!(count_of_unknowns);
    
    (0..(1u64 << count_of_unknowns)).filter(|x| {        let mut choices = *x;
        let mut correct_next = correct.iter().peekable();
        let mut current_run_count = 0;
        // print!("{correct:?} ");
        // draw(*x, input);
        for &i in input {
            let target = correct_next.peek().copied().copied().unwrap_or_default();
            let here = i.unwrap_or_else(|| {let a = choices.get_bit(0); choices >>= 1; if a { SpringState::Broken} else {SpringState::Working}});
            match here {
                SpringState::Broken => {
                    current_run_count += 1;
                }
                SpringState::Working => {
                    if current_run_count > 0 {
                        if current_run_count != target {
                            // println!(" ==> run mismatch");
                            return false;
                        } else {
                            current_run_count = 0;
                            correct_next.next();
                        }
                    }
                }
            }
            if current_run_count > target {
                // println!(" ==> run too long");
                return false;
            }
        }
        let target = correct_next.next().copied().unwrap_or_default();
        if correct_next.peek().is_some() {
            // println!("Bad Remaining");
            return false;
        }
        // if target == current_run_count {
        //     println!(" ==> OK");
        // } else {
        //     println!(" ==> last item wrong");
        // }
        target == current_run_count

    }).count()
}

// fn check_counts(input: &[SpringState], counts: &[usize]) -> bool {
//     let mut next_count = counts.iter();
//     for (key, group) in &input.iter().group_by(|x| **x) {
//         if key == SpringState::Broken {
//             match (group.count(), next_count.next()) {
//                 (_, None) => return false,
//                 (a, Some(b)) if a != *b => return false,
//                 _ => ()
//             };
//         }
//     }
//     next_count.next().is_none()
// }

type Memo<'a> = HashMap<(&'a[char], u32, &'a [u32]), usize>;

fn count_arrangements<'a>(memo: &mut Memo<'a>, input: &'a [char], current_run: u32, runs: &'a [u32]) -> usize 
{
    let key = (input, current_run, runs);
    if let Some(x) = memo.get(&key) {
        return *x
    }
    
    if runs.is_empty() {
        let a = if input.iter().all(|&x| x != '#') {1} else {0};
        memo.insert(key, a);
        return a;
    }
    if current_run > runs[0] {
        memo.insert(key,0);
        return 0;
    }
    if input.is_empty() {
        let a = if current_run == runs[0] && runs.len() == 1 {
            1
        } else {
            0
        };
        memo.insert(key, a);
        return a;
    }
    let a = match input[0] {
        '#' => {
            count_arrangements(memo, &input[1..], current_run+1, runs)
        }
        '.' => {
            if current_run == 0 {
                count_arrangements(memo, &input[1..], 0, runs)
            } else if current_run == runs[0] {
                count_arrangements(memo, &input[1..], 0, &runs[1..])
            } else {
                0
            }
        }
        '?' => {
            count_arrangements(memo, &input[1..], current_run+1, runs) + //'#' case.
            if current_run == 0 {
                count_arrangements(memo, &input[1..], 0, runs)
            } else if current_run == runs[0] {
                count_arrangements(memo, &input[1..], 0, &runs[1..])
            } else {
                0
            }
        }
        _ => panic!("Unknown char {input:?}")
    };
    memo.insert(key, a);
    a
}

fn solve_line(input: &str) -> usize {
    let (input, (springs, counts)) = separated_pair(
        many1( one_of::<_,_,NomError>("#.?")),
        space1,
        separated_list1(tag(","), complete::u32)
    )(input).expect("parse");
    assert_eq!(input, "");
    let mut memo = HashMap::new();
    count_arrangements(&mut memo, &springs, 0, &counts)
}
fn solve_line_2(input: &str) -> usize {
    let (input, (springs, counts)) = separated_pair(
        many1( one_of::<_,_,NomError>("#.?")),
        space1,
        separated_list1(tag(","), complete::u32)
    )(input).expect("parse");
    assert_eq!(input, "");
    let sep = vec!['?'];
    let springs : Vec<char> = [&springs, &sep, &springs, &sep, &springs, &sep, &springs, &sep, &springs].into_iter().flatten().copied().collect();
    let counts: Vec<u32> = [&counts, &counts, &counts, &counts, &counts].iter().flat_map(|x| x.iter()).copied().collect();
    let mut memo = HashMap::new();
    count_arrangements(&mut memo, &springs, 0, &counts)
}
fn p1(input: &str) -> usize {
    input.lines().map(|x| {let a = solve_line(x); println!("{x:?} => {a}"); a}).sum()
}
fn p2(input: &str) -> usize {
    input.lines().map(|x| {let a = solve_line_2(x); println!("{x:?} => {a}"); a}).sum()
}


const EG1: &str = "?###???????? 3,2,1";
const EG: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
