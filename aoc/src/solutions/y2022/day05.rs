use std::str::FromStr;

use aoc_harness::*;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, sequence::tuple};

aoc_main!(2022 day 5, generator whole_input_is::<X>, part1 [solve::<true>] => "GFTNRBZPF", part2 [solve::<false>] => "VRQWPDSGP", example both EG => ("CMZ","MCD"));

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
    let (input, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        complete::u8,
        tag(" from "),
        complete::u8,
        tag(" to "),
        complete::u8,
    ))(input)?;
    Ok((
        input,
        Command {
            count: count as usize,
            from: (from - 1) as usize,
            to: (to - 1) as usize,
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

fn solve<const CARRY_MANY: bool>(input: &X) -> String {
    let mut stacks = input.stacks.clone();
    for c in &input.instructions {
        let len = stacks[c.from].len();
        let mut carry = stacks[c.from].split_off(len-c.count);
        if CARRY_MANY {
            carry.reverse();
        }
        stacks[c.to].extend(carry);
    }
    read_tops(&stacks)
}