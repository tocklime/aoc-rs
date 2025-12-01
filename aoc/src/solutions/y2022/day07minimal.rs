use std::str::FromStr;

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{newline, u32},
    combinator::{eof, value},
    error::{Error, ErrorKind},
    multi::{fold_many0, many0},
    sequence::terminated,
    Err, Finish, IResult, Parser,
};

aoc_harness::aoc_main!(2022 day 7, generator whole_input_is::<Dirs>, part1 [p1] => 1_644_735, part2 [p2] => 1_300_850, example both EG => (95437, 24_933_642));

const EG: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
#[derive(Debug, Default)]
struct Dirs {
    my_size: u32,
    sizes: Vec<u32>,
}
impl FromStr for Dirs {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_dir(s).finish() {
            Ok((_, dir)) => Ok(dir),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn rest_of_line(input: &str) -> IResult<&str, &str> {
    terminated(take_until("\n"), newline).parse(input)
}
fn parse_dir(input: &str) -> IResult<&str, Dirs> {
    let (input, (_, dir, _)) = (tag("$ cd "), rest_of_line, rest_of_line).parse(input)?;
    if dir == ".." {
        return Err(Err::Failure(Error {
            input,
            code: ErrorKind::Fail,
        }));
    }
    let (input, files) = fold_many0(
        alt((
            value(0, (tag("dir "), rest_of_line)),
            terminated(u32, rest_of_line),
        )),
        || 0,
        |a, b| a + b,
    )
    .parse(input)?;
    let (input, children) = many0(parse_dir).parse(input)?;
    let (my_size, mut sizes) = children
        .into_iter()
        .fold((files, Vec::new()), |mut acc, i| {
            acc.0 += i.my_size;
            acc.1.extend(i.sizes);
            acc
        });
    sizes.push(my_size);
    let (input, _) = alt((eof, tag("$ cd ..\n"))).parse(input)?;
    Ok((input, Dirs { my_size, sizes }))
}

fn p1(top: &Dirs) -> u32 {
    top.sizes.iter().filter(|&&x| x < 100_000).sum::<u32>()
}

fn p2(top: &Dirs) -> u32 {
    const TOTAL: u32 = 70_000_000;
    const REQUIRED_FREE: u32 = 30_000_000;
    const MAX_USED: u32 = TOTAL - REQUIRED_FREE;
    let required_to_free = top.my_size - MAX_USED;
    //want the smallest dir > required_to_free.
    *top.sizes
        .iter()
        .filter(|&&x| x >= required_to_free)
        .min()
        .unwrap()
}
