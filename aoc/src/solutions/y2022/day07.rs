
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{newline, u32},
    combinator::{map, value},
    sequence::{preceded, terminated, tuple},
    IResult,
};

aoc_harness::aoc_main!(2022 day 7, both [solve] => (1_644_735, 1_300_850), example both EG => (95437, 24_933_642));

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
enum Line<'a> {
    CdDown(&'a str),
    CdUp,
    Ls,
    Dir(&'a str),
    File(u32, &'a str),
}
fn parse_line(input: &str) -> IResult<&str, Line> {
    terminated(
        alt((
            value(Line::CdUp, tag("$ cd ..")),
            value(Line::Ls, tag("$ ls")),
            preceded(tag("$ cd "), map(take_until("\n"), Line::CdDown)),
            preceded(tag("dir "), map(take_until("\n"), Line::Dir)),
            map(
                tuple((u32, tag(" "), take_until("\n"))),
                |(size, _, name)| Line::File(size, name),
            ),
        )),
        newline,
    )(input)
}

#[derive(Debug, Default)]
struct Dir {
    files: u32,
    dirs: Vec<Dir>,
}
impl Dir {
    fn size(&self) -> u32 {
        self.files + self.dirs.iter().map(Self::size).sum::<u32>()
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
    let mut dir_stack: Vec<Dir> = vec![Dir::default()];
    while !rest.is_empty() {
        let (input, l) = parse_line(rest).unwrap();
        rest = input;
        match l {
            Line::CdUp => {
                //combine dir into parent.
                let ch = dir_stack.pop().unwrap();
                dir_stack.last_mut().unwrap().dirs.push(ch);
            }
            Line::Ls | Line::Dir(_) => (),
            Line::CdDown(_) => {
                dir_stack.push(Dir::default());
            }
            Line::File(size, _) => {
                let b = dir_stack.last_mut().unwrap();
                b.files += size;
            }
        }
    }

    while dir_stack.len() > 1 {
        let ch = dir_stack.pop().expect("dir_stack not empty");
        dir_stack
            .last_mut()
            .expect("dir_stack not empty")
            .dirs
            .push(ch);
    }
    let top = dir_stack.pop().expect("dir_stack not empty");
    let all_sizes = top.all_sizes();
    let part1 = all_sizes.iter().filter(|&&x| x < 100_000).sum::<u32>();
    let total = 70_000_000;
    let required_free = 30_000_000;
    let max_used = total - required_free;
    let current = top.size();
    let required_to_free = current - max_used;
    //want the smallest dir > required_to_free.
    let part2 = all_sizes
        .iter()
        .filter(|&&x| x >= required_to_free)
        .min()
        .expect("size found");

    (part1, *part2)
}
