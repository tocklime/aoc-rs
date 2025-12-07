use itertools::Itertools;
use nom::{
    Parser,
    branch::alt,
    character::complete::{self, newline, one_of, space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
};
use utils::{grid2d::Grid2d, nom::NomError};

aoc_harness::aoc_main!(2025 day 6, part1 [p1] => 5_667_835_681_547, part2 [p2] => 9_434_900_032_651, both [both], example part1 EG => 4_277_556, example part2 EG => 3_263_827);

#[derive(Debug, Copy, Clone)]
enum ParseItem {
    Number(u64),
    Operator(char),
}

impl ParseItem {
    fn as_num_or_panic(self) -> u64 {
        match self {
            ParseItem::Number(n) => n,
            ParseItem::Operator(o) => panic!("Not a number: '{o}'"),
        }
    }
}

fn p1(input: &str) -> u64 {
    let ns = all_consuming(separated_list1(
        (space0, newline::<_, NomError>, space0),
        separated_list1(
            space1,
            alt((
                complete::u64.map(ParseItem::Number),
                one_of("*+").map(ParseItem::Operator),
            )),
        ),
    ))
    .parse(input.trim())
    .unwrap()
    .1;
    let wid = ns[0].len();
    (0..wid)
        .map(|ix| {
            let mut prob = ns.iter().map(move |x| x[ix]).collect::<Vec<_>>();
            match prob.pop().unwrap() {
                ParseItem::Operator('+') => prob
                    .into_iter()
                    .map(ParseItem::as_num_or_panic)
                    .sum::<u64>(),
                ParseItem::Operator('*') => prob
                    .into_iter()
                    .map(ParseItem::as_num_or_panic)
                    .product::<u64>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn p2(input: &str) -> u64 {
    let mut lines = input.trim().lines().collect::<Vec<&str>>();
    let op_line = lines.pop().unwrap();
    let mut op = None;
    let mut total = 0;
    let mut pending = Vec::new();
    for ix in 0..lines[0].len() {
        let str : String = lines.iter().map(|l| l.chars().nth(ix).unwrap()).collect();
        // println!("{ix}: {str}");
        if str.trim().is_empty() {
            //blank.
            match op.take() {
                Some('+') => total += pending.iter().copied().sum::<u64>(),
                Some('*') => total += pending.iter().copied().product::<u64>(),
                _ => unreachable!()
            }
            pending.clear();
        } else {
            let op_char = op_line.chars().nth(ix);
            if op_char.is_some() && op_char != Some(' ') {
                op = op_char;
            }
            let n : u64 = str.trim().parse().unwrap();
            pending.push(n);
        }
    }
            match op.take() {
                Some('+') => total += pending.iter().copied().sum::<u64>(),
                Some('*') => total += pending.iter().copied().product::<u64>(),
                _ => unreachable!()
            }
            pending.clear();
    total
}

fn both(input: &str) -> (u64, u64) {
    let (final_newline, _) = input
        .trim_end()
        .as_bytes()
        .iter()
        .enumerate()
        .rev()
        .find(|x| x.1 == &b'\n')
        .unwrap();
    let (nums, ops) = input.split_at(final_newline);
    let ops: Vec<char> = ops.chars().filter(|c| "*+".contains(*c)).collect();
    let num_grid = Grid2d::from_str(nums, |x| x as u8);
    let gaps: Vec<usize> = (0..num_grid.dim().x)
        .filter(|x| num_grid.get_col(*x).all(|c| c == &b' '))
        .map(|x| x + 1)
        .collect();
    let mut p1 = 0;
    let mut p2 = 0;
    for ((from, to), op) in [0]
        .into_iter()
        .chain(gaps)
        .chain([num_grid.dim().x+1])
        .tuple_windows()
        .zip(ops.into_iter())
    {
        let sub_grid = Grid2d::from_fn((num_grid.dim().y, to - from - 1), |p| {
            num_grid[(p.y, p.x + from)]
        });
        // println!(
        //     "{:?}\n{}",
        //     sub_grid.dim(),
        //     sub_grid.to_string_with(|x| (*x as char).to_string())
        // );
        match op {
            '+' => {
                p1 += (0..sub_grid.dim().y)
                    .map(|x| {
                        (str::from_utf8(sub_grid.get_row(x)).unwrap())
                            .trim()
                            .parse::<u64>()
                            .unwrap()
                    })
                    .sum::<u64>() ;
                p2 += (0..sub_grid.dim().x)
                    .map(|x| {
                        let vec = sub_grid.get_col(x).copied().collect::<Vec<u8>>();
                        let str = str::from_utf8(&vec).unwrap().trim();
                        str.parse::<u64>().unwrap()
                    })
                    .sum::<u64>() ;
            }
            '*' => {
                p1 += (0..sub_grid.dim().y)
                    .map(|x| {
                        (str::from_utf8(sub_grid.get_row(x)).unwrap())
                            .trim()
                            .parse::<u64>()
                            .unwrap()
                    })
                    .product::<u64>() ;
                p2 += (0..sub_grid.dim().x)
                    .map(|x| {
                        let vec = sub_grid.get_col(x).copied().collect::<Vec<u8>>();
                        let str = str::from_utf8(&vec).unwrap().trim();
                        str.parse::<u64>().unwrap()
                    })
                    .product::<u64>() ;
            }
            _ => unreachable!(),
        }
    }
    (p1, p2)
}

const EG: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
