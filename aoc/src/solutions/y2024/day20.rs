use std::{collections::HashSet, sync::atomic::AtomicUsize};

use utils::grid2d::Grid2d;
use rayon::prelude::*;

aoc_harness::aoc_main!(2024 day 20, generator Puzzle::from_str, part1 [solve::<2>] => 1389, part2 [solve::<20>] => 1_005_068);

fn solve<const RANGE: usize>(input: &Puzzle) -> usize {
    let ans = AtomicUsize::new(0);
    input.get_cheats(RANGE, &|x| {
        if x >= 100 {
            ans.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    });
    ans.into_inner()
}
struct Puzzle {
    g: Grid2d<Option<usize>>,
}
impl Puzzle {
    fn from_str(input: &str) -> Self {
        let g = Grid2d::from_str_as_char(input);
        let end = g.find(|x| x == &'E').unwrap().0;
        let mut costs_g: Grid2d<Option<usize>> = Grid2d::from_elem(g.dim(), None);
        let mut fringe = HashSet::new();
        fringe.insert(end);
        for step in 0usize.. {
            let mut new_fringe = HashSet::new();
            for &p in &fringe {
                if costs_g[p].is_none() {
                    costs_g[p] = Some(step);
                    new_fringe.extend(g.neighbours(p).filter(|&x| g[x] != '#'));
                }
            }
            if new_fringe.is_empty() {
                break;
            }
            fringe = new_fringe;
        }
        Self { g: costs_g }
    }
    fn get_cheats<F>(&self, range: usize, f: &F)
    where
        F: Fn(usize) + Sync,
    {
        let in_par = self.g.indexes().par_bridge();
        in_par.for_each(|p| {
            //spot shortcuts from p.
            if let Some(cost) = self.g[p] {
                for c in (1..=range)
                    .flat_map(|r| self.g.cells_at_range(p, r))
                    .filter_map(|x| {
                        self.g[x].and_then(|y| cost.checked_sub(1 + y + p.manhattan_unsigned(&x)))
                    })
                {
                    f(c+1);
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, sync::Mutex};

    #[test]
    fn example() {
        let ans = Mutex::new(BTreeMap::new());
        super::Puzzle::from_str(EG).get_cheats(2, &|c| *ans.lock().unwrap().entry(c).or_default() += 1);
        let ans: Vec<_> = ans.into_inner().unwrap().into_iter().collect();
        assert_eq!(
            ans,
            [
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1),
            ]
        );
    }
    #[test]
    fn example_part_2() {
        let ans = Mutex::new(BTreeMap::new());
        super::Puzzle::from_str(EG).get_cheats(20, &|c| {
            if c >= 50 {
                *ans.lock().unwrap().entry(c).or_default() += 1;
            }
        });
        let ans: Vec<_> = ans.into_inner().unwrap().into_iter().collect();
        assert_eq!(
            ans,
            [
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ]
        );
    }
    const EG: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
}
