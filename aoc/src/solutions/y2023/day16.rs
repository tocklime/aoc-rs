use utils::{
    cartesian::{Dir, Point},
    grid2d::{self, Grid2d},
};

aoc_harness::aoc_main!(2023 day 16, part1 [p1], part2 [p2], example both EG => (46, 51));

fn dir_to_n(d: Dir) -> u8 {
    match d {
        Dir::Up => 1,
        Dir::Down => 2,
        Dir::Left => 4,
        Dir::Right => 8,
    }
}
fn run_sim(g: &Grid2d<char>, beam_start: Point<usize>, beam_dir: Dir) -> usize {
    let mut energised = Grid2d::from_elem(g.dim(), 0);
    let mut beams = vec![(beam_start, beam_dir)];
    while let Some((mut beam_source, mut beam_dir)) = beams.pop() {
        while let Some(&char_here) = g.get(beam_source) {
            if energised[beam_source] & dir_to_n(beam_dir) > 0 {
                //been here before.
                break;
            }
            energised[beam_source] |= dir_to_n(beam_dir);
            match char_here {
                '.' => (),
                '/' => {
                    beam_dir = match beam_dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    };
                }
                '\\' => {
                    beam_dir = match beam_dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    };
                },
                '-' => {
                    if beam_dir == Dir::Up || beam_dir == Dir::Down {
                        beam_dir = Dir::Left;
                        beams.push((beam_source, Dir::Right));
                    }
                }
                '|' => {
                    if beam_dir == Dir::Left || beam_dir == Dir::Right {
                        beam_dir = Dir::Down;
                        beams.push((beam_source, Dir::Up));
                    }
                }
                _ => panic!("Bad char {char_here}")
            }
            beam_source = beam_source.step_flip_y(beam_dir);
        }
    }
    energised.iter().filter(|&&x| x > 0).count()
}
fn p1(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    run_sim(&g, Point::new(0,0), Dir::Right)
}

fn p2(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let mut best = 0;
    best = best.max((0..g.dim().x).map(|x| run_sim(&g, Point::new(x, 0), Dir::Down)).max().unwrap());
    best = best.max((0..g.dim().x).map(|x| run_sim(&g, Point::new(x, g.dim().y-1), Dir::Up)).max().unwrap());
    best = best.max((0..g.dim().y).map(|y| run_sim(&g, Point::new(0, y), Dir::Right)).max().unwrap());
    best = best.max((0..g.dim().y).map(|y| run_sim(&g, Point::new(g.dim().x-1, y), Dir::Left)).max().unwrap());
    best
}

const EG: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
