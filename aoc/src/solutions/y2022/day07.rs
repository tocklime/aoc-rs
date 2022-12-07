use std::{cell::RefCell, collections::HashMap, rc::Rc};

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha1},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

aoc_main!(2022 day 7, both [p1], example both EG => (95437, 24933642));

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
#[derive(Debug, Clone)]
enum Command<'a> {
    CdDown(&'a str),
    CdUp,
    Ls,
}
#[derive(Debug, Clone)]
enum Output<'a> {
    Dir(&'a str),
    File(u32, &'a str),
}
#[derive(Debug, Clone)]
enum Line<'a> {
    Command(Command<'a>),
    Output(Output<'a>),
}
fn parse_output(input: &str) -> IResult<&str, Output> {
    alt((
        preceded(tag("dir "), map(take_until("\n"), Output::Dir)),
        map(
            tuple((complete::u32, tag(" "), take_until("\n"))),
            |(size, _, name)| Output::File(size, name),
        ),
    ))(input)
}
fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Command<'a>> {
    let (input, _) = tag("$ ")(input)?;
    alt((
        value(Command::CdUp, tag("cd ..")),
        value(Command::Ls, tag("ls")),
        map(preceded(tag("cd "), take_until("\n")), |d: &str| {
            Command::CdDown(d)
        }),
    ))(input)
}
fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_output, Line::Output),
    ))(input)
}

#[derive(Debug, Default)]
struct Dir<'a> {
    files: u32,
    dirs: HashMap<&'a str, Rc<RefCell<Dir<'a>>>>,
}
impl<'a> Dir<'a> {
    fn size(&self) -> u32 {
        self.files + self.dirs.iter().map(|(k,v)| v.borrow().size()).sum::<u32>()
    }
    fn all_sizes(&self) -> Vec<u32> {
        let mut ans = Vec::new();
        ans.push(self.size());
        for (_, d) in &self.dirs {
            ans.extend(d.borrow().all_sizes());
        }
        ans
    }
}

fn p1(input: &str) -> (u32,u32) {
    let (rest, val) = separated_list1(tag("\n"), parse_line)(input).unwrap();
    let top = Rc::new(RefCell::new(Dir::default()));
    let mut dir_stack = vec![Rc::clone(&top)];
    for l in &val[1..] {
        match l {
            Line::Command(Command::CdUp) => {
                dir_stack.pop();
            }
            Line::Command(Command::Ls) => (),
            Line::Command(Command::CdDown(n)) => {
                let b = dir_stack.last().unwrap().borrow();
                let target = Rc::clone(b.dirs.get(n).unwrap());
                drop(b);
                dir_stack.push(target);
            }
            Line::Output(Output::Dir(n)) => {
                let mut b = dir_stack.last().unwrap().borrow_mut();
                b.dirs.insert(n, Rc::new(RefCell::new(Dir::default())));
            }
            Line::Output(Output::File(size, _)) => {
                let mut b = dir_stack.last().unwrap().borrow_mut();
                b.files += size;
            }
        }
    }
    let part1 = top.borrow().all_sizes().into_iter().filter(|&x| x < 100000).sum::<u32>();
    let total = 70000000;
    let required_free = 30000000;
    let max_used = total - required_free;
    let current = top.borrow().size();
    let required_to_free = current - max_used;
    //want the smallest dir > required_to_free.
    let part2 = top.borrow().all_sizes().into_iter().filter(|&x| x >= required_to_free).min().unwrap();
    

    (part1, part2)
}
