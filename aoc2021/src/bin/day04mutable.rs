use aoc_harness::*;
use std::str::FromStr;
use utils::numset::NumSet;

aoc_main!(2021 day 4, generator whole_input_is::<Day04>, [p1] => 60368, [p2] => 17435);

#[derive(Clone, Debug)]
struct Board {
    grid: [u8; 25],
    locs: [Option<usize>; 100],
}
struct Marks {
    set: NumSet<u32>,
    row_counts: [u8; 5],
    col_counts: [u8; 5],
    won: bool,
}
impl Marks {
    pub fn new() -> Self {
        Self {
            set: NumSet::new(),
            row_counts: [0; 5],
            col_counts: [0; 5],
            won: false,
        }
    }
}

impl Board {
    fn mark_off(&self, num: u8, marks: &mut Marks) -> bool {
        if let Some(p) = self.locs[num as usize]
        {
            marks.set.insert(p);
            let r = &mut marks.row_counts[p/5];
            *r += 1;
            let c = &mut marks.col_counts[p%5];
            *c += 1;
            marks.won = *c == 5 || *r == 5;
        }
        marks.won
    }
    fn score(&self, marks: &NumSet<u32>) -> usize {
        (0..25)
            .filter(|&ix| !marks.contains(ix))
            .map(|ix| self.grid[ix] as usize)
            .sum::<usize>()
    }
}
#[derive(Clone, Debug)]
struct Day04 {
    num_seq: Vec<u8>,
    boards: Vec<Board>,
}
impl FromStr for Day04 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nums, boards) = s.split_once("\n\n").unwrap();
        let num_seq = nums.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
        let boards = boards
            .split("\n\n")
            .map(|b| b.parse::<Board>().unwrap())
            .collect();
        Ok(Self { num_seq, boards })
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u8> = s
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let mut locs = [None; 100];
        let mut grid = [0;25];
        for (ix, &n) in nums.iter().enumerate() {
            locs[n as usize] = Some(ix);
            grid[ix] = n;
        }

        Ok(Self { grid, locs })
    }
}
fn p1(input: &Day04) -> usize {
    let mut marks: Vec<(&Board, Marks)> = input.boards.iter().map(|x| (x, Marks::new())).collect();
    for &x in &input.num_seq {
        for (b, m) in marks.iter_mut() {
            if b.mark_off(x, m) {
                return (x as usize) * b.score(&m.set);
            }
        }
    }
    0
}

fn p2(input: &Day04) -> usize {
    let mut marks: Vec<(&Board, Marks)> = input.boards.iter().map(|x| (x, Marks::new())).collect();
    let mut nums = input.num_seq.iter();
    let mut x = 0;
    while marks.len() > 1 {
        x = *nums.next().unwrap();
        for (b, m) in marks.iter_mut() {
            b.mark_off(x, m);
        }
        marks.retain(|x| !x.1.won);
    }
    let (b, mut m) = marks.pop().unwrap();
    while !m.won {
        x = *nums.next().unwrap();
        b.mark_off(x, &mut m);
    }
    b.score(&m.set) * (x as usize)
}

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

    #[test]
    fn t2() {
        assert_eq!(p2(&whole_input_is(EG)), 1924);
    }
}
