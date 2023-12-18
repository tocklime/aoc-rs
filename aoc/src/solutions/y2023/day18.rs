use nom::{
    character::complete::{self, hex_digit1, newline, one_of, space1},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use utils::{cartesian::Dir, nom::IResult};

aoc_harness::aoc_main!(2023 day 18, part1 [solve::<0>], part2 [solve::<1>], example both EG => (62, 952_408_144_115));

const DIRS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
fn parse_line(input: &str) -> IResult<[(Dir, isize); 2]> {
    let (input, p1) = tuple((
        one_of("UDLR")
            .map(|x| Dir::from_x("UDLR", x))
            .terminated(space1),
        complete::u32.terminated(space1),
    ))
    .map(|(a, b)| (a, b as isize))
    .parse(input)?;
    let (input, p2) = hex_digit1::<&str, _>
        .terminated(tag(")"))
        .preceded_by(tag("(#"))
        .map(|x| {
            (
                DIRS[usize::from_str_radix(&x[5..], 16).unwrap()],
                isize::from_str_radix(&x[0..5], 16).unwrap(),
            )
        })
        .parse(input)?;
    Ok((input, [p1, p2]))
}

fn solve<const PART: usize>(input: &str) -> isize {
    let (input, lines): (_, Vec<_>) = separated_list1(newline, parse_line).parse(input).unwrap();
    assert_eq!(input, "\n");

    let (final_x, total) = lines.iter().fold((0, 1), |(x_pos, total), l| {
        let l = &l[PART];
        match l.0 {
            //when going to the right, we just dig our line.
            Dir::Right => (x_pos + l.1, total + l.1),
            //when going left, we dig nothing (the area we travel through is handled in the right and down cases)
            Dir::Left => (x_pos - l.1, total),
            //when going down, we additionally dig everything to the left (up to the x=0 axis).
            //the x_pos +1 represents the width of the digger.
            Dir::Down => (x_pos, total + l.1 * (x_pos + 1)),
            //when going up we anti-dig everything to the left.
            Dir::Up => (x_pos, total - l.1 * x_pos),
        }
    });
    assert_eq!(final_x, 0);
    total
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
