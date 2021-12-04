use std::{collections::HashSet, str::FromStr};

use aoc_harness::*;

aoc_main!(2021 day 4, [p1], [p2]);

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<u8>>,
}
impl Board {
    fn is_won(&self, drawn: &HashSet<u8>) -> bool {
        let line_win = self
            .grid
            .iter()
            .any(|l| l.iter().all(|c| drawn.contains(c)));
        let col_win = (0..=4).any(|c_ix| {
            self.grid
                .iter()
                .map(|l| l[c_ix])
                .all(|c| drawn.contains(&c))
        });
        line_win || col_win
    }
    fn score(&self, drawn: &HashSet<u8>) -> usize {
        let left = self
            .grid
            .iter()
            .flat_map(|c| c.iter())
            .filter(|c| !drawn.contains(c))
            .map(|&x| x as usize)
            .collect::<Vec<_>>();
        dbg!(&left);
        left.into_iter().sum::<usize>()
    }
}
#[derive(Debug)]
struct Day04 {
    num_seq: Vec<u8>,
    boards: Vec<Board>,
}
impl FromStr for Day04 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.lines();
        let num_seq = i
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        i.next().unwrap(); //blank line
        let boards = parse_boards(i);
        Ok(Self { num_seq, boards })
    }
}

fn parse_boards(i: std::str::Lines) -> Vec<Board> {
    i.chunks(6)
        .into_iter()
        .map(|b| {
            let mut iter = b.into_iter();
            Board {
                grid: vec![
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                ],
            }
        })
        .collect::<Vec<_>>()
}
fn p1(input: &str) -> usize {
    let d: Day04 = input.parse().unwrap();
    let mut drawn = HashSet::new();
    for x in d.num_seq {
        drawn.insert(x);
        let won = d.boards.iter().filter(|x| x.is_won(&drawn)).next();
        if let Some(b) = won {
            return (x as usize) * b.score(&drawn);
        }
    }
    0
}

fn p2(input: &str) -> usize {
    let mut d: Day04 = input.parse().unwrap();
    let mut drawn = HashSet::new();
    for x in d.num_seq {
        drawn.insert(x);
        if d.boards.len() == 1 {
            let only_board = &d.boards[0];
            if only_board.is_won(&drawn) {
                dbg!(x, &drawn, &only_board);
                return (x as usize) * only_board.score(&drawn);
            }
        } else {
            d.boards.retain(|b| !b.is_won(&drawn));
        }
    }
    0
}
//16620 is wrong

#[cfg(test)]
mod tests {
    use super::*;
    const EG: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const col_win: &str = "1

0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
";
    const row_win: &str = "1

1 1 1 1 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";
    #[test]
    fn t2() {
        let cw_b = col_win.parse::<Day04>().unwrap();
        let rw_b = row_win.parse::<Day04>().unwrap();
        let hs_1 = vec![1].into_iter().collect::<HashSet<_>>();
        assert!(cw_b.boards[0].is_won(&hs_1));
        assert!(rw_b.boards[0].is_won(&hs_1));
        assert_eq!(p2(EG), 1924);
    }
}
