use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("#{id} @ {left},{top}: {width}x{height}")]
pub struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn right(&self) -> usize {
        self.left + self.width
    }
    fn bottom(&self) -> usize {
        self.top + self.height
    }
    fn intersects(&self, other: &Self) -> bool {
        let intersects_x = self.right() >= other.left && self.left <= other.right();
        let intersects_y = self.bottom() >= other.top && self.top <= other.bottom();
        intersects_x && intersects_y
    }
    fn squares(self: &Self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.left..self.right()).flat_map(move |x| (self.top..self.bottom()).map(move |y| (x, y)))
    }
}


fn gen(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|l| l.parse().expect("Bad line"))
        .collect()
}


fn part1(input: &[Claim]) -> usize {
    let mut grid = [[0 as u8; 1000]; 1000];
    let mut count = 0;
    for c in input {
        for (x, y) in c.squares() {
            if grid[x][y] == 1 {
                count += 1;
            }
            grid[x][y] += 1;
        }
    }
    count
}

fn part1_vec(input: &[Claim]) -> usize {
    let mut grid = vec![vec![0 as u8; 1000]; 1000];
    let mut count = 0;
    for c in input {
        for (x, y) in c.squares() {
            if grid[x][y] == 1 {
                count += 1;
            }
            grid[x][y] += 1;
        }
    }
    count
}

fn part1_hm(input: &[Claim]) -> usize {
    let mut grid = std::collections::HashMap::new();
    let mut count = 0;
    for c in input {
        for p in c.squares() {
            if grid.get(&p) == Some(&1) {
                count += 1;
            }
            *grid.entry(p).or_insert(0) += 1;
        }
    }
    count
}

use std::collections::HashSet;


fn part2(input: &[Claim]) -> usize {
    let mut candidates: HashSet<usize> = input.iter().map(|c| c.id).collect();
    for c1ix in 0..input.len() {
        for c2ix in c1ix + 1..input.len() {
            let c1 = &input[c1ix];
            let c2 = &input[c2ix];
            if c1.intersects(c2) {
                candidates.remove(&c1.id);
                candidates.remove(&c2.id);
            }
        }
    }
    *candidates.iter().next().expect("Not found")
}


fn part2_map(input: &[Claim]) -> usize {
    let mut grid = HashMap::new();
    let mut candidates: HashSet<usize> = input.iter().map(|c| c.id).collect();
    for c in input {
        for p in c.squares() {
            match grid.get(&p) {
                Some(o_id) => {
                    candidates.remove(o_id);
                    candidates.remove(&c.id);
                }
                None => {
                    grid.insert(p, c.id);
                }
            }
        }
    }
    *candidates.iter().next().expect("Not found")
}

#[cfg(test)]
const HINT_INPUT: &'static str = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;

#[test]
fn test_example() {
    assert_eq!(part1(&gen(HINT_INPUT.trim())), 4);
}
