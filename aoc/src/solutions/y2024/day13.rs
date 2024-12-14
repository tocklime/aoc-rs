use itertools::Itertools;
use nalgebra::{matrix, vector};
use utils::{cartesian::Point, inputs::parse_numbers};

aoc_harness::aoc_main!(2024 day 13, 
    generator gen, 
    part1 [solve::<0>, solve_mat::<0>] => 27157, 
    part2[solve::<10_000_000_000_000>,solve_mat::<10_000_000_000_000>] => 104_015_411_578_548, 
    example part1 EG => 480);

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
    fn move_prize(&self, n: i64) -> Self {
        Self {
            a: self.a,
            b: self.b,
            p: self.p + Point::new(n, n),
        }
    }
    fn solve(&self) -> Option<i64> {
        let Machine { a, b, p } = *self;
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
        Some(3 * a_presses + b_presses)
    }
    fn solve_mat(&self) -> Option<i64> {
        let m_a = matrix![
            self.a.x as f64, self.b.x as f64;
            self.a.y as f64, self.b.y as f64;
        ];
        let m_b = vector![self.p.x as f64, self.p.y as f64];
        let a_inv = m_a.try_inverse()?;
        let x = a_inv * m_b;
        let a = x[0].round() as i64;
        let b = x[1].round() as i64;
        //check that the solution works with whole numbers.
        if a * self.a.x + b * self.b.x == self.p.x && a * self.a.y + b * self.b.y == self.p.y {
            Some(3 * a + b)
        } else {
            None
        }
    }
}

fn gen(input: &str) -> Vec<Machine> {
    parse_numbers(input)
        .chunks(6)
        .into_iter()
        .map(|mut x| Machine {
            a: Point::new(x.next().unwrap() as i64, x.next().unwrap() as i64),
            b: Point::new(x.next().unwrap() as i64, x.next().unwrap() as i64),
            p: Point::new(x.next().unwrap() as i64, x.next().unwrap() as i64),
        })
        .collect()
}
fn solve_mat<const PRIZE_OFFSET: i64>(input: &[Machine]) -> i64 {
    input
        .iter()
        .filter_map(|x| x.move_prize(PRIZE_OFFSET).solve_mat())
        .sum()
}

fn solve<const PRIZE_OFFSET: i64>(input: &[Machine]) -> i64 {
    input
        .iter()
        .filter_map(|x| x.move_prize(PRIZE_OFFSET).solve())
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
