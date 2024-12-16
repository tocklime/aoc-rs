use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

aoc_harness::aoc_main!(2024 day 16, both [both] => (95444,513), example both EG => (7036,45));

fn both(input: &str) -> (usize, usize) {
    let g = Grid2d::from_str_as_char(input);
    let start = g.find(|&x| x == 'S').unwrap().0;
    let end = g.find(|&x| x == 'E').unwrap().0;
    let (ans, p1) = pathfinding::directed::astar::astar_bag(
        &(start, Dir::Right),
        |&(p, d)| {
            //move forward (if poss) costs one.
            //or turn either way.
            let next: Point<usize> = p.step(d);
            g.get(next)
                .filter(|&&c| c == 'E' || c == '.')
                .map(|_| ((next, d), 1))
                .into_iter()
                .chain([((p, d.turn_left()), 1000), ((p, d.turn_right()), 1000)])
        },
        |&(p, _)| end.manhattan_unsigned(&p),
        |&(p, _)| p == end,
    )
    .unwrap();
    let mut good_seat = Grid2d::from_elem(g.dim(), false);
    for n in ans {
        for (p, _) in n {
            good_seat[p] = true;
        }
    }
    (p1, good_seat.iter().filter(|x| **x).count())
}

const EG: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
