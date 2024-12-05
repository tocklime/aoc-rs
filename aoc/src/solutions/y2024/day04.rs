use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 4, generator Grid2d::from_str_as_bytes, part1 [p1] => 2633, part2 [p2] => 1936, example part1 EG => 18, example part2 EG => 9);

const DIRS: [Point<isize>; 8] = [
    Point::new(1, 0),
    Point::new(1, 1),
    Point::new(0, 1),
    Point::new(-1, 1),
    Point::new(-1, 0),
    Point::new(-1, -1),
    Point::new(0, -1),
    Point::new(1, -1),
];
fn p1(g: &Grid2d<u8>) -> usize {
    const TARGET: &[u8; 4] = b"XMAS";
    g.indexed_iter()
        .filter(|(_, c)| **c == b'X')
        .map(|(p, _)| {
            DIRS.iter()
                .filter(|&&d| {
                    g.values_in_direction(p, d)
                        .take(TARGET.len())
                        .map(|x| x.1)
                        .eq(TARGET.iter())
                })
                .count()
        })
        .sum()
}
fn p2(g: &Grid2d<u8>) -> usize {
    g.indexed_iter()
        .filter(|(p, c)| {
            c == &&b'A'
                && !g.is_edge(*p)
                && (g[p.up().right()] == b'S' && g[p.down().left()] == b'M'
                    || g[p.up().right()] == b'M' && g[p.down().left()] == b'S')
                && (g[p.up().left()] == b'S' && g[p.down().right()] == b'M'
                    || g[p.up().left()] == b'M' && g[p.down().right()] == b'S')
        })
        .count()
}

const EG: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
