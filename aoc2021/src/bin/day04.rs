use aoc_harness::*;
use ndarray::Array2;
use std::str::FromStr;
use utils::nums::NumBitExt;

aoc_main!(2021 day 4, generator whole_input_is::<Day04>, [p1] => 60368, [p2] => 17435);

#[derive(Debug, Clone, Copy)]
struct NumSet {
    n: u128,
}
impl NumSet {
    pub fn new() -> Self {
        Self { n: 0 }
    }
    pub fn insert(&mut self, n: u8) {
        self.n.set_bit(n.into(), true)
    }
    pub fn is_subset(&self, other: &NumSet) -> bool {
        (self.n & other.n) == self.n
    }
    pub fn contains(&self, n: u8) -> bool {
        self.n.get_bit(n.into())
    }
}
impl FromIterator<u8> for NumSet {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x);
        }
        s
    }
}
#[derive(Clone, Debug)]
struct Board {
    grid: Array2<u8>,
    win_sets: Vec<NumSet>,
}
impl Board {
    fn is_won(&self, drawn: &NumSet) -> bool {
        self.win_sets.iter().any(|x| x.is_subset(drawn))
    }
    fn score(&self, drawn: &NumSet) -> usize {
        self.grid
            .iter()
            .filter(|c| !drawn.contains(**c))
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
        let nums = s
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let grid = Array2::from_shape_vec((5, 5), nums).unwrap();
        let mut win_sets: Vec<NumSet> = grid
            .rows()
            .into_iter()
            .map(|x| x.iter().cloned().collect())
            .collect();
        win_sets.extend(
            grid.columns()
                .into_iter()
                .map(|x| x.iter().cloned().collect::<NumSet>()),
        );

        Ok(Self { grid, win_sets })
    }
}
fn p1(input: &Day04) -> usize {
    let mut drawn = NumSet::new();
    for &x in &input.num_seq {
        drawn.insert(x);
        let won = input.boards.iter().filter(|x| x.is_won(&drawn)).next();
        if let Some(b) = won {
            return (x as usize) * b.score(&drawn);
        }
    }
    0
}

fn p2(input: &Day04) -> usize {
    let mut boards: Vec<&Board> = input.boards.iter().collect();
    let mut drawn = NumSet::new();
    for &x in &input.num_seq {
        drawn.insert(x);
        match boards[..] {
            [only_board] => {
                if only_board.is_won(&drawn) {
                    return (x as usize) * only_board.score(&drawn);
                }
            }
            _ => {
                boards.retain(|b| !b.is_won(&drawn));
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
        let rw_b = ROW_WIN.parse::<Board>().unwrap();
        let hs_1 = vec![1].into_iter().collect::<NumSet>();
        assert!(cw_b.is_won(&hs_1));
        assert!(rw_b.is_won(&hs_1));
        assert_eq!(p2(&whole_input_is(EG)), 1924);
    }
}
