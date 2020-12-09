use crate::utils::collections::{de_prefixsum, minmax};

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x|x.parse().unwrap()).collect()
}

#[aoc(day9,part1)]
pub fn p1(is: &[usize]) -> usize {
    is.windows(25).enumerate().find_map(|(ix,w)| {
        let target = is[25+ix];
        if w.iter().all(|x| !w.contains(&(target-x))) {
            Some(is[ix+25])
        }else {
            None
        }
    }).unwrap()
}

#[aoc(day9,part2)]
pub fn p2(is: &[usize]) -> usize {
    let target = p1(is);
    let ps = de_prefixsum(is);
    for ix_1 in 0..is.len() {
        if let Some(ix_2) = (ix_1..is.len())
                .take_while(|&ix| ps[ix] - ps[ix_1] <= target)
                .find(|&ix| ps[ix] - ps[ix_1] == target) {
            let (&a,&b) = minmax(is[ix_1+1..=ix_2].iter()).unwrap();
            return a+b;
        }
    }
    0
}