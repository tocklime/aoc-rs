use nom::{bytes::complete::tag, IResult};
use utils::cartesian::Point;

aoc_harness::aoc_main!(2024 day 13, part1 [solve::<0>] => 27157, part2[solve::<10_000_000_000_000>] => 104_015_411_578_548, example part1 EG => 480);

#[derive(Debug)]
struct Machine {
    a: Point<i64>,
    b: Point<i64>,
    p: Point<i64>,
}
fn try_div(a: i64, b: i64) -> Option<i64> {
    (a % b == 0).then_some(a / b)
}
impl Machine {
    fn solve(&self) -> Option<i64> {
        let Machine{a,b,p} = *self;
        //in x, we have A * a_x + B * b_x == p_x
        //in y we have A * a_y + B * b_y == p_y
        //this is intersection of two lines.
        // A == (p_x - B*b_x)/a_x
        // A == (p_y - B*b_y)/a_y
        //(p_x - B*b_x)/a_x == (p_y - B*b_y)/a_y
        // a_y * (p_x - B*b_x) == a_x * (p_y - B*b_y)
        // a_y*p_x - B*a_y*b_x == a_x * p_y - a_x - B * a_x * b_y
        // a_y*p_x - a_x * p_y == B (a_y*b_x - a_x * b_y)
        // B == (a_y*p_x - a_x*p_y) / (a_y*b_x - a_x * b_y)
        let b_presses = try_div(a.y * p.x - a.x * p.y, a.y * b.x - a.x * b.y)?;
        let a_presses = try_div(p.x - b_presses * b.x, a.x)?;
        Some(3*a_presses + b_presses)
    }
}

fn parse(input: &str, prize_offset: i64) -> IResult<&str, Machine> {
    let (input, _) = tag("Button A: X+")(input)?;
    let (input, ax) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, ay) = nom::character::complete::i64(input)?;
    let (input, _) = tag("\nButton B: X+")(input)?;
    let (input, bx) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, by) = nom::character::complete::i64(input)?;
    let (input, _) = tag("\nPrize: X=")(input)?;
    let (input, px) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, py) = nom::character::complete::i64(input)?;
    Ok((
        input,
        Machine {
            a: Point::new(ax, ay),
            b: Point::new(bx, by),
            p: Point::new(px + prize_offset, py + prize_offset),
        },
    ))
}

fn solve<const PRIZE_OFFSET: i64>(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|x| parse(x, PRIZE_OFFSET).unwrap().1)
        .filter_map(|x| x.solve())
        .sum()
}

const EG: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
