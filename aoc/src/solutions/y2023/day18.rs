use itertools::Itertools;
use nom::{
    bytes::complete::take_while_m_n,
    character::complete::{self, newline, one_of, space1},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::tuple,
    Parser,
};
use nom_supreme::{
    multi::{collect_separated_terminated, parse_separated_terminated},
    tag::{self, complete::tag},
    ParserExt,
};
use utils::{
    aabb::Aabb,
    cartesian::{Dir, Point},
    grid2d::Grid2d,
    nom::IResult,
};

aoc_harness::aoc_main!(2023 day 18, part1 [p1, solve::<1>], part2 [solve::<2>], example both EG => (62, 952_408_144_115));

fn hex<const N: usize>(input: &str) -> IResult<usize> {
    take_while_m_n(N, N, |x: char| x.is_digit(16))
        .map(|x| u64::from_str_radix(x, 16).unwrap() as usize)
        .parse(input)
}
fn parse_line(input: &str) -> IResult<[(Dir, isize);2]> {
    let (input, (dir, dist)) = tuple((
        one_of("UDLR")
            .map(|x| Dir::from_x("UDLR", x))
            .terminated(space1),
        complete::u32.terminated(space1),
    ))
    .parse(input)?;
    let (input, (dist2, dir2)) = tuple((hex::<5>, hex::<1>))
        .terminated(tag(")"))
        .preceded_by(tag("(#"))
        .parse(input)?;
    let dir2 = [Dir::Right, Dir::Down, Dir::Left, Dir::Up][dir2];
    Ok((input, [(dir, dist as isize), (dir2, dist2 as isize)]))
}
fn p1(input: &str) -> u32 {
    let (input, lines): (_, Vec<_>) = separated_list1(newline, parse_line).parse(input).unwrap();
    assert_eq!(input, "\n");

    let mut pos = Point::new(0, 0);
    let mut b = Aabb::new(Point::new(0, 0));
    for [l, _] in &lines {
        pos += match l.0 {
            Dir::Up => Point::new(0, -l.1),
            Dir::Down => Point::new(0, l.1),
            Dir::Left => Point::new(-l.1, 0),
            Dir::Right => Point::new(l.1, 0),
        };
        b = b.extend(pos);
    }

    // assert_eq!(b.bottom_left, Point::new(0, 0));
    let max = Point::new(b.width() + 1, b.height() + 1);

    let offset = -b.bottom_left;
    let mut pos = Point::<usize>::new(offset.x as usize, offset.y as usize);
    let mut g = Grid2d::from_elem(max, 0);
    let mut g2 = Grid2d::from_elem(max, '.');
    let mut last_dir = Dir::Right;
    for [l,_] in lines {
        let target = pos
            + match l.0 {
                Dir::Up => Point::new(0isize, -l.1),
                Dir::Down => Point::new(0, l.1),
                Dir::Left => Point::new(-l.1, 0),
                Dir::Right => Point::new(l.1, 0),
            };
        for (ix, p) in pos.steps_to(target, false).enumerate() {
            if g.get(p).is_some() {
                g[p] += 1;
                g2[p] = if ix == 0 {
                    match (last_dir, l.0) {
                        (Dir::Up, Dir::Up) => '|',
                        (Dir::Up, Dir::Left) => '7',
                        (Dir::Up, Dir::Right) => 'F',
                        (Dir::Down, Dir::Left) => 'J',
                        (Dir::Down, Dir::Right) => 'L',
                        (Dir::Left, Dir::Up) => 'L',
                        (Dir::Left, Dir::Down) => 'F',
                        (Dir::Right, Dir::Up) => 'J',
                        (Dir::Right, Dir::Down) => '7',
                        _ => 'F',
                    }
                } else {
                    match l.0 {
                        Dir::Up | Dir::Down => '|',
                        Dir::Left | Dir::Right => '-',
                    }
                }
            } else {
                panic!();
            }
        }
        last_dir = l.0;
        pos = target;
    }

    let mut inside = false;
    for (p, c) in g2.indexed_iter_mut() {
        match *c {
            '|' | 'J' | 'L' => inside = !inside,
            '.' if inside => {
                g[p] += 1;
                *c = 'I';
            }
            '.' if !inside => *c = 'O',
            _ => (),
        }
    }

    g.iter().sum()
}

fn solve<const PART: usize>(input: &str) -> u64 {
    let (input, lines): (_, Vec<_>) = separated_list1(newline, parse_line).parse(input).unwrap();
    assert_eq!(input, "\n");

    let mut pos = Point::new(0, 0);
    let mut b = Aabb::new(Point::new(0, 0));
    let mut corners = vec![pos];
    for l in &lines {
        let l = &l[PART-1];
        pos += match l.0 {
            Dir::Up => Point::new(0isize, -l.1),
            Dir::Down => Point::new(0, l.1),
            Dir::Left => Point::new(-l.1, 0),
            Dir::Right => Point::new(l.1, 0),
        };
        b = b.extend(pos);
        corners.push(pos);
    }
    let mut sum = 0i64;
    for (a, b) in corners.iter().tuple_windows() {
        //need size of trapezoid (a, b, (b.x,0), (a.x,0)).
        let n = (a.y - b.y).abs() + (a.x - b.x).abs();
        //if horiz line, then (b.y-a.y) == 0, so just add n.
        //if vert line, then (a.x == b.x...)
        //roughly, we're counting half-squares between this line and the x=0 line.
        let y_diff = b.y - a.y;
        if y_diff != 0 {
            assert_eq!(b.x,a.x);
        }
        let x2 = (2 * a.x)*(b.y - a.y) + n;

        // println!("{a:?}->{b:?} adds {x2}");

        sum += x2 as i64;
    }
    //1+ because we seem to be out by one?
    1 + (sum as u64) / 2
}

const EG: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
