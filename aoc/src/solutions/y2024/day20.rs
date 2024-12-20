use std::collections::{BTreeMap, HashSet};

use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2024 day 20, part1 [solve::<2>] => 1389, part2 [solve::<20>] => 1_005_068);

fn solve<const RANGE: usize>(input: &str) -> usize {
    get_cheats(input, RANGE)
        .into_iter()
        .filter_map(|x| (x.0 >= 100).then_some(x.1))
        .sum()
}

fn get_cheats(input: &str, range: usize) -> Vec<(usize, usize)> {
    let g = Grid2d::from_str_as_char(input);
    // let start = g.find(|x| x == &'S').unwrap().0;
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
    let mut shortcut_counts: BTreeMap<usize, usize> = BTreeMap::new();

    for (p, cost) in costs_g.indexed_iter() {
        //spot shortcuts from p.
        if let &Some(cost) = cost {
            costs_g
                .nearby_within_range(p, range)
                .filter_map(|x| {
                    costs_g[x].and_then(|y| cost.checked_sub(y + p.manhattan_unsigned(&x)))
                })
                .for_each(|c| *shortcut_counts.entry(c).or_default() += 1);
        }
    }
    shortcut_counts.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::get_cheats;

    #[test]
    fn example() {
        assert_eq!(
            super::get_cheats(EG, 2),
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
        let a = get_cheats(EG, 20);
        let filtered: Vec<(usize, usize)> = a
            .into_iter()
            .filter_map(|x| (x.0 >= 50).then_some((x.1, x.0)))
            .collect();
        assert_eq!(
            filtered,
            [
                (32, 50),
                (31, 52),
                (29, 54),
                (39, 56),
                (25, 58),
                (23, 60),
                (20, 62),
                (19, 64),
                (12, 66),
                (14, 68),
                (12, 70),
                (22, 72),
                (4, 74),
                (3, 76),
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
