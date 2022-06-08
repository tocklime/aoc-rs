use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Block<T>(Vec<Vec<T>>);

impl Block<bool> {
    fn from_str(input: &str) -> Self {
        Block(input.split("/").map(|l| l.chars().map(|x| x == '#').collect()).collect())
    }
    fn to_str(&self) -> String {
        self.0.iter().map(|l| l.iter().map(|c| if *c { '#' } else { '.' }).collect::<String>()).join("\n")
    }
}

impl<T> Block<T> {
    fn map<F>(&self, f: F) -> Block<T>
        where F: Fn(&T) -> T {
        Block(self.0.iter().map(move |l| l.iter().map(|c| f(c)).collect()).collect())
    }
    fn sub_grid(&self, size: usize, x: usize, y: usize) -> Self
        where T: Clone {
        let vec = (y..y + size).map(|y_1| self.0[y_1][x..x + size].to_vec()).collect();
        Block(vec)
    }
    fn split(self, size: usize) -> Block<Self>
        where T: Clone {
        assert_eq!(self.0.len() % size, 0);
        let mut ans = Vec::with_capacity(self.0.len() / size);
        for y in (0..self.0.len()).step_by(size) {
            let v = (0..self.0[0].len())
                .step_by(size)
                .map(|x| self.sub_grid(size, x, y)).collect();
            ans.push(v);
        }
        Block(ans)
    }
    fn mirror(self) -> Self
        where T: Clone {
        Block(self.0.clone().into_iter().rev().collect())
    }
    fn rotate(self) -> Self
        where T: Clone
    {
        let rot_and_tranposed = (0..self.0.len()).rev().map(|c|
            self.0.iter().map(|r| r[c].clone()).collect_vec()
        ).collect_vec();
        Block(rot_and_tranposed)
    }
}

impl<T> Block<Block<T>> {
    fn join(self) -> Block<T>
        where T: Clone {
        //#join grid of grids into single grid.
        let mut ans: Vec<Vec<T>> = Vec::new();
        for l in &self.0 {
            let h = l[0].0.len();
            //height of subblocks.
            for y in 0..h {
                //y is a row we want to extract from every item of l.
                let r = l.iter().map(|b| &b.0[y]).flatten().cloned().collect_vec();
                ans.push(r);
            }
        }
        Block(ans)
    }
}


fn p1(input: &str) -> usize {
    run(input, 5)
}


fn p2(input: &str) -> usize {
    run(input, 18)
}

fn run(input: &str, iter_count: usize) -> usize {
    let mut hm = HashMap::new();
    for l in input.lines() {
        let s = l.split(" => ").collect_vec();
        let mut a = Block::from_str(s[0]);
        let b = Block::from_str(s[1]);
        hm.insert(a.clone(), b.clone());
        let mut m = a.clone().mirror();
        hm.insert(m.clone(), b.clone());
        for _ in 0..3 {
            a = a.rotate();
            m = m.rotate();
            hm.insert(a.clone(), b.clone());
            hm.insert(m.clone(), b.clone());
        }
    }
    let mut block = Block::from_str(".#./..#/###");
    for _ in 0..iter_count {
        let s = block.0.len();
        let split_into = (2..=3).find(|n| s % n == 0).unwrap();
        let split = block.split(split_into);
        let mapped = split.map(|c| hm.get(c).unwrap().clone());
        block = mapped.join();
    }
    block.0.iter().map(|l| l.iter().filter(|x| **x).count()).sum()
}

