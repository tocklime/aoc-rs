use aoc_harness::*;
use ndarray::Array2;
use std::str::FromStr;

aoc_main!(2021 day 4, generator whole_input_is::<Day04>, [p1] => 60368, [p2] => 17435);

const MARKED: u8 = 255;
#[derive(Clone, Debug)]
struct Board {
    grid: Array2<u8>, //255 == marked.
    locs: [Option<(usize, usize)>; 100],
}
impl Board {
    fn is_won(&self) -> bool {
        self.grid
            .rows()
            .into_iter()
            .any(|l| l.iter().all(|c| *c == MARKED))
            || self
                .grid
                .columns()
                .into_iter()
                .any(|l| l.iter().all(|c| *c == MARKED))
    }
    fn mark_off(&mut self, num: u8) {
        if let Some(p) = self.locs[num as usize] {
            *self.grid.get_mut(p).unwrap() = MARKED;
        }
    }
    fn score(&self) -> usize {
        self.grid
            .iter()
            .filter(|c| **c != MARKED)
            .map(|&x| x as usize)
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
        for (ix, &n) in nums.iter().enumerate() {
            locs[n as usize] = Some((ix / 5, ix % 5));
        }
        let grid = Array2::from_shape_vec((5, 5), nums).unwrap();
        Ok(Self { grid, locs })
    }
}
fn p1(input: &Day04) -> usize {
    let mut input: Day04 = input.clone();
    for &x in &input.num_seq {
        for b in input.boards.iter_mut() {
            b.mark_off(x);
            if b.is_won() {
                return (x as usize) * b.score();
            }
        }
    }
    0
}

fn p2(input: &Day04) -> usize {
    let mut input: Day04 = input.clone();
    for &x in &input.num_seq {
        for b in input.boards.iter_mut() {
            b.mark_off(x);
        }

        match &mut input.boards[..] {
            [only_board] => {
                if only_board.is_won() {
                    return (x as usize) * only_board.score();
                }
            }
            _ => {
                input.boards.retain(|b| !b.is_won());
            }
        }
    }
    0
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

    const COL_WIN: &str = "0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
0 0 0 0 1
";
    const ROW_WIN: &str = "1 1 1 1 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";
    #[test]
    fn t2() {
        let mut cw_b = COL_WIN.parse::<Board>().unwrap();
        cw_b.mark_off(1);
        assert!(cw_b.is_won());
        let mut rw_b = ROW_WIN.parse::<Board>().unwrap();
        rw_b.mark_off(1);
        assert!(rw_b.is_won());
        assert_eq!(p2(&whole_input_is(EG)), 1924);
    }
}
