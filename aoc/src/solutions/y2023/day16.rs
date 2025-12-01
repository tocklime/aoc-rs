use utils::{
    cartesian::{Dir, Point},
    grid2d::{Coord, Grid2d},
};

aoc_harness::aoc_main!(2023 day 16, generator gen_, part1 [p1] => 7884, part2 [p2] => 8185, example both EG => (46, 51));

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
    while let Some((mut loc, mut dir)) = beams.pop() {
        while let Some(&char_here) = g.get(loc) {
            if energised[loc] & dir_to_n(dir) > 0 {
                //been here before.
                break;
            }
            energised[loc] |= dir_to_n(dir);
            match char_here {
                '/' => {
                    dir = dir.map([Dir::Right, Dir::Left, Dir::Down, Dir::Up]);
                }
                '\\' => {
                    dir = dir.map([Dir::Left, Dir::Right, Dir::Up, Dir::Down]);
                }
                '-' => {
                    if dir == Dir::Up || dir == Dir::Down {
                        dir = Dir::Left;
                        beams.push((loc, Dir::Right));
                    }
                }
                '|' => {
                    if dir == Dir::Left || dir == Dir::Right {
                        dir = Dir::Down;
                        beams.push((loc, Dir::Up));
                    }
                }
                _ => ()
            }
            loc = loc.step_flip_y(dir);
        }
    }
    energised.iter().filter(|&&x| x > 0).count()
}
fn gen_(input: &str) -> Grid2d<char> {
    Grid2d::from_str(input, |x| x)
}
fn p1(g: &Grid2d<char>) -> usize {
    run_sim(g, Point::new(0, 0), Dir::Right)
}

fn p2(g: &Grid2d<char>) -> usize {
    let (my, mx) = g.dim().into();
    let starts: [(Dir, Box<dyn Iterator<Item = Coord>>); 4] = [
        (Dir::Down, Box::new((0..mx).map(|x| Point::new(x, 0)))),
        (Dir::Up, Box::new((0..mx).map(|x| Point::new(x, my - 1)))),
        (Dir::Right, Box::new((0..my).map(|y| Point::new(0, y)))),
        (Dir::Left, Box::new((0..my).map(|y| Point::new(mx - 1, y)))),
    ];
    starts
        .into_iter()
        .flat_map(|(d, ps)| ps.map(move |p| run_sim(g, p, d)))
        .max()
        .unwrap()
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
