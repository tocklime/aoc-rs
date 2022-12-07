use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::u32,
    combinator::{map, value},
    sequence::{preceded, terminated, tuple},
    IResult,
};

aoc_main!(2022 day 7, both [solve], example both EG => (95437, 24933642));

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
            tuple((u32, tag(" "), take_until("\n"))),
            |(size, _, name)| Output::File(size, name),
        ),
    ))(input)
}
fn parse_command(input: &str) -> IResult<&str, Command> {
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
    terminated(
        alt((
        map(parse_command, Line::Command),
        map(parse_output, Line::Output),
    )),
    tag("\n"))
    (input)
}

#[derive(Debug, Default)]
struct Dir {
    files: u32,
    dirs: Vec<Dir>,
}
impl Dir {
    fn size(&self) -> u32 {
        self.files
            + self
                .dirs
                .iter()
                .map(|v| v.size())
                .sum::<u32>()
    }
    fn all_sizes(&self) -> Vec<u32> {
        let mut ans = Vec::new();
        ans.push(self.size());
        for d in &self.dirs {
            ans.extend(d.all_sizes());
        }
        ans
    }
}

fn solve(input: &str) -> (u32, u32) {
    let mut rest = input;
    let mut dir_stack : Vec<Dir> = vec![Default::default()];
    while !rest.is_empty() {
        let (input, l) = parse_line(rest).unwrap();
        rest = input;
        match l {
            Line::Command(Command::CdUp) => {
                //combine dir into parent.
                let ch = dir_stack.pop().unwrap();
                dir_stack.last_mut().unwrap().dirs.push(ch);
            }
            Line::Command(Command::Ls) => (),
            Line::Command(Command::CdDown(_)) => {
                dir_stack.push(Default::default());
            }
            Line::Output(Output::Dir(_)) => (),
            Line::Output(Output::File(size, _)) => {
                let b = dir_stack.last_mut().unwrap();
                b.files += size;
            }
        }
    }

    while dir_stack.len() > 1 {
        let ch = dir_stack.pop().unwrap();
        dir_stack.last_mut().unwrap().dirs.push(ch);
    }
    let top = dir_stack.pop().unwrap();
    let part1 = top
        .all_sizes()
        .into_iter()
        .filter(|&x| x < 100000)
        .sum::<u32>();
    let total = 70000000;
    let required_free = 30000000;
    let max_used = total - required_free;
    let current = top.size();
    let required_to_free = current - max_used;
    //want the smallest dir > required_to_free.
    let part2 = top
        .all_sizes()
        .into_iter()
        .filter(|&x| x >= required_to_free)
        .min()
        .unwrap();

    (part1, part2)
}
