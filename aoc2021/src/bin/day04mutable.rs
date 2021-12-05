use aoc_harness::*;
use ndarray::Array2;
use std::str::FromStr;
use utils::numset::NumSet;

aoc_main!(2021 day 4, generator whole_input_is::<Day04>, [p1] => 60368, [p2] => 17435);

#[derive(Clone, Debug)]
struct Board {
    grid: Array2<u8>,
    locs: [Option<(usize, usize)>; 100],
}

impl Board {
    fn is_won(&self, marks: &NumSet<u32>) -> bool {
        const WINNING_MARKS: &[u32; 10] = &[
            0b1111100000000000000000000,
            0b0000011111000000000000000,
            0b0000000000111110000000000,
            0b0000000000000001111100000,
            0b0000000000000000000011111,
            0b1000010000100001000010000,
            0b0100001000010000100001000,
            0b0010000100001000010000100,
            0b0001000010000100001000010,
            0b0000100001000010000100001,
        ];
        WINNING_MARKS
            .iter()
            .any(|&m| NumSet::from(m).is_subset(marks))
    }
    fn mark_off(&self, num: u8, marks: &mut NumSet<u32>) {
        if let Some(p) = self.find(num) {
            marks.insert(p.0 * 5 + p.1)
        }
    }
    fn find(&self, num: u8) -> Option<(usize, usize)> {
        self.locs[num as usize]
    }
    fn score(&self, marks: &NumSet<u32>) -> usize {
        (0..25)
            .filter(|&ix| !marks.contains(ix))
            .map(|ix| self.grid[(ix / 5, ix % 5)] as usize)
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
    let mut marks: Vec<(&Board, NumSet<u32>)> =
        input.boards.iter().map(|x| (x, NumSet::new())).collect();
    for &x in &input.num_seq {
        for (b, m) in marks.iter_mut() {
            b.mark_off(x, m);
            if b.is_won(m) {
                return (x as usize) * b.score(m);
            }
        }
    }
    0
}

fn p2(input: &Day04) -> usize {
    let mut marks: Vec<(&Board, NumSet<u32>)> =
        input.boards.iter().map(|x| (x, NumSet::new())).collect();
    for &x in &input.num_seq {
        for (b, m) in marks.iter_mut() {
            b.mark_off(x, m);
        }

        match &mut marks[..] {
            [(only_board, m)] => {
                if only_board.is_won(m) {
                    return (x as usize) * only_board.score(m);
                }
            }
            _ => {
                marks.retain(|(b, m)| !b.is_won(m));
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
        let cw_b = COL_WIN.parse::<Board>().unwrap();
        let mut marks = NumSet::new();
        cw_b.mark_off(1, &mut marks);
        assert!(cw_b.is_won(&marks));
        let rw_b = ROW_WIN.parse::<Board>().unwrap();
        let mut marks = NumSet::new();
        rw_b.mark_off(1, &mut marks);
        assert!(rw_b.is_won(&marks));
        assert_eq!(p2(&whole_input_is(EG)), 1924);
    }
}
