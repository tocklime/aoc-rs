use utils::grid2d::{Grid2d, Coord};

aoc_harness::aoc_main!(2023 day 11, part1 [solve::<2>], part2 [solve::<1000000>], example part1 EG => 374);

fn solve<const N: usize> (input: &str) -> usize {
    let mut map = Grid2d::from_str(input, |x| x);
    let s = map.dim();
    let mut wide_rows = vec![];
    let mut wide_cols = vec![];
    for x in 0..s.x {
        if (0..s.y).all(|y| map[(y,x)] != '#') {
            wide_cols.push(x);
            (0..s.y).for_each(|y| map[(y,x)] = ':');
        }
    }
    for y in 0..s.y {
        if (0..s.x).all(|x| map[(y,x)] != '#') {
            wide_rows.push(y);
            (0..s.x).for_each(|x| map[(y,x)] = ':');
        }
    }
    // println!("{map}");
    let stars : Vec<Coord> = map.indexed_iter().filter(|x| x.1 == &'#').map(|x| x.0).collect();
    let mut total = 0;
    for (a_ix, a) in stars.iter().enumerate() {
        for (b_ix, b)in stars.iter().enumerate().skip(a_ix) {
            let x_range = if b.x > a.x { a.x..b.x } else { b.x..a.x };
            let y_range = if b.y > a.y { a.y..b.y } else { b.y..a.y };
            let x_extra = wide_cols.iter().filter(|c| x_range.contains(c)).count();
            let y_extra = wide_rows.iter().filter(|r| y_range.contains(r)).count();
            let dist = a.x.abs_diff(b.x) + a.y.abs_diff(b.y) + (N-1) * (x_extra + y_extra);
            // println!("Dist between {} and {} is {}", a_ix+1,b_ix+1,dist);
            total += dist;
        }
    }
    total
}

const EG: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";


//wrong: 726820896326
#[cfg(test)]
mod test {
    #[test]
    fn tests() {
        assert_eq!(super::solve::<10>(super::EG), 1030);
        assert_eq!(super::solve::<100>(super::EG), 8410);
    }
}