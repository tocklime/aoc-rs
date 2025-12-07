use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2025 day 7, both [both] => (1687,390_684_413_472_684), example part1 EG => 21, example part2 EG => 40);

fn both(input: &str) -> (usize, usize) {
    let g = Grid2d::from_str(input, |x| x);
    let start = g.find(|c| c == &'S').unwrap().0;
    let mut init: Vec<usize> = vec![0; g.dim().x];
    init[start.x] = 1;
    let mut split_count = 0;
    let timeline_count = (start.y..g.dim().y)
        .fold(init, |mut active, row| {
            for ix in 0..g.dim().x {
                let c = g[(row, ix)];
                if c == '^' && active[ix] > 0 {
                    let count = active[ix];
                    active[ix] = 0;
                    active[ix - 1] += count;
                    active[ix + 1] += count;
                    split_count += 1;
                }
            }
            active
        })
        .iter()
        .sum();
    (split_count, timeline_count)
}

const EG: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
