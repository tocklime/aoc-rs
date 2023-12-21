use std::collections::HashSet;

// use progress_bar::inc_progress_bar;
use utils::{
    cartesian::Point,
    grid2d::{Grid2d, ICoord},
    polynomials::PolynomialDetector,
};

aoc_harness::aoc_main!(2023 day 21, part1 [p1] => 3858, part2 [p2_from_consts, p2::<26_501_365>] => 636_350_496_972_143, example part1 EG => 42);

#[allow(dead_code)]
fn draw(plain: &Grid2d<char>, reach: &Grid2d<bool>) {
    let mut ans = String::new();
    for (p, t) in reach.indexed_iter() {
        if *t {
            ans.push('O');
        } else {
            ans.push(plain[p]);
        }
        if p.x == reach.dim().x - 1 {
            ans.push('\n');
        }
    }
    println!("{}", ans);
}
#[allow(dead_code)]
fn draw2(plain: &Grid2d<char>, reach: &HashSet<ICoord>, seen: &HashSet<ICoord>) {
    let mut ans = String::new();
    for (p, t) in plain.indexed_iter() {
        let ip = Point::new(p.x as isize, p.y as isize);
        if reach.contains(&ip) {
            ans.push('O');
        } else if seen.contains(&ip) {
            ans.push('o');
        } else {
            ans.push(*t);
        }
        if p.x == plain.dim().x - 1 {
            ans.push('\n');
        }
    }
    println!("{}", ans);
}
fn step(rocks: &Grid2d<char>, reach: &Grid2d<bool>) -> Grid2d<bool> {
    Grid2d::from_fn(reach.dim(), |p| {
        rocks[p] != '#' && p.neighbours().iter().any(|n| reach.get(*n) == Some(&true))
    })
}
fn p1(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let mut reach = Grid2d::from_fn(g.dim(), |x| g[x] == 'S');
    // println!("{g}");
    // draw(&g, &reach);
    for _ in 0..64 {
        reach = step(&g, &reach);
    }
    // draw(&g, &reach);
    reach.iter().filter(|x| **x).count()
}
fn p2_from_consts(_input: &str) -> usize {
    let mut pd = PolynomialDetector::default();
    pd.add(3943);
    pd.add(97407);
    pd.add(315_263);
    pd.add(657_511);
    let r = pd.get_equation();
    r.evaluate(101_151)
}
fn p2<const N: usize>(input: &str) -> i64 {
    let g = Grid2d::from_str(input, |x| x);
    let mut reach: HashSet<ICoord> = g
        .indexed_iter()
        .filter_map(|(p, v)| {
            if v == &'S' {
                Some(Point::new(p.x as isize, p.y as isize))
            } else {
                None
            }
        })
        .collect();
    let mut pd = PolynomialDetector::default();
    let mid = reach.iter().next().unwrap().x as usize; //65;
    let width = 2 * g.dim().x; //262;

    let d1 = g.dim();
    let d = Point::new(d1.x as isize, d1.y as isize);
    let mut seen = [reach.clone(), HashSet::new()];
    for iteration in 0..N {
        // if iteration%width > (mid - 3) && iteration%width < (mid + 3) {
        //     let n = seen[iteration%2].len();
        //     println!("{iteration}: {n}");
        // }
        if iteration % width == mid {
            let n = seen[iteration % 2].len();
            // let n = seen[iteration%2].len() + reach.len();
            pd.add(n as i64);
            // println!("{iteration}: {n}");
            if pd.get_certainty_and_power().certainty > 1 {
                let n = (N - mid) / width; //101_150.
                return pd.get_equation().evaluate(n as i64 + 1);
            }
        }
        let new: HashSet<ICoord> = reach
            .iter()
            .flat_map(|o| {
                o.neighbours().into_iter().filter(|n| {
                    let project = Point::new(
                        usize::try_from(n.x.rem_euclid(d.x)).unwrap(),
                        usize::try_from(n.y.rem_euclid(d.y)).unwrap(),
                    );
                    g[project] != '#'
                })
            })
            .collect();
        reach.clear();
        for &p in &new {
            if !seen[(1 + iteration) % 2].contains(&p) {
                seen[(1 + iteration) % 2].insert(p);
                reach.insert(p);
            }
        }
    }
    seen[N % 2].len().try_into().unwrap()
    // reach.len()
}

const EG: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn given_example_1() {
        assert_eq!(p2::<6>(EG), 16);
        assert_eq!(p2::<10>(EG), 50);
        assert_eq!(p2::<50>(EG), 1594);
        assert_eq!(p2::<100>(EG), 6536);
        assert_eq!(p2::<500>(EG), 167_004);
        assert_eq!(p2::<1000>(EG), 668_697);
        assert_eq!(p2::<5000>(EG), 16_733_044);
    }
}

// 17161y = x^2.15549 + 26684x + 236838
// x = 26501365
