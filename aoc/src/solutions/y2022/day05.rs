use std::{cmp::Ordering, str::FromStr};

use aoc_harness::*;
use nom::{bytes::complete::tag, character::complete::u8, multi::separated_list1, sequence::tuple};

aoc_main!(2022 day 5, generator whole_input_is::<X>, part1 [solve::<false>] => "GFTNRBZPF", part2 [solve::<true>] => "VRQWPDSGP", example both EG => ("CMZ","MCD"));

const EG: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

struct X {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Command>,
}
#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_line(input: &str) -> nom::IResult<&str, Command> {
    let (input, (_, count, _, from, _, to)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;
    assert_ne!(from, to);
    Ok((
        input,
        Command {
            count: count.into(),
            from: (from - 1).into(),
            to: (to - 1).into(),
        },
    ))
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, instrs) = s.split_once("\n\n").unwrap();
        let (_, instructions) =
            separated_list1(tag("\n"), parse_line)(instrs).map_err(|e| e.to_string())?;

        let w = map.lines().map(|x| x.len()).max().unwrap();
        let stack_count = (w + 1) / 4;
        let mut stacks = vec![vec![]; stack_count];
        for l in map.lines().rev().skip(1) {
            for (ix, c) in l.chars().enumerate() {
                if c.is_alphabetic() {
                    stacks[(ix - 1) / 4].push(c);
                }
            }
        }

        Ok(X {
            stacks,
            instructions,
        })
    }
}
fn read_tops(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|x| x.last().unwrap()).collect()
}
fn borrow_mut_twice<T>(arr: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    match a.cmp(&b) {
        Ordering::Less => {
            let (arr_a, arr_b) = arr.split_at_mut(b);
            (&mut arr_a[a], &mut arr_b[0])
        }
        Ordering::Greater => {
            let (arr_b, arr_a) = arr.split_at_mut(a);
            (&mut arr_a[0], &mut arr_b[b])
        }
        Ordering::Equal => panic!("Can't borrow twice from the same index"),
    }
}

fn solve<const CARRY_MANY: bool>(input: &X) -> String {
    let mut stacks = input.stacks.clone();
    for c in &input.instructions {
        let len = stacks[c.from].len();
        let (from, to) = borrow_mut_twice(&mut stacks, c.from, c.to);
        let carry = from[len - c.count..].iter();
        if !CARRY_MANY {
            to.extend(carry.rev())
        } else {
            to.extend(carry);
        }
        from.truncate(len-c.count);
    }
    read_tops(&stacks)
}

// fn solve_both(input: &X) -> (String, String) {
//     let mut stacks_p1 = input.stacks.clone();
//     let mut stacks_p2 = input.stacks.clone();
//     for c in &input.instructions {
//         let len = stacks_p1[c.from].len();
//         let carry = stacks_p1[c.from].split_off(len - c.count);
//         stacks_p1[c.to].extend(carry.into_iter().rev());
//         let len = stacks_p2[c.from].len();
//         let carry = stacks_p2[c.from].split_off(len - c.count);
//         stacks_p2[c.to].extend(carry);
//     }
//     (read_tops(&stacks_p1), read_tops(&stacks_p2))
// }
