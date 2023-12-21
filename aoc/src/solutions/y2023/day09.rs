use rayon::prelude::*;
use utils::polynomials::PolynomialDetector;

aoc_harness::aoc_main!(2023 day 9, generator gen, both [with_poly_detector, single_thread, multi_thread] => (1_877_825_184, 1108), example both EG => (114,2));



fn gen(input: &str) -> Vec<Vec<i64>> {
    let parse_line = |l: &str| l.split_whitespace().map(|x| x.parse().unwrap()).collect();
    input.lines().map(parse_line).collect()
}
fn solve_line(mut ns: Vec<i64>) -> (i64, i64) {
    let mut ans = (0, 0);
    let mut mults = [1, -1].into_iter().cycle();
    while ns.iter().any(|x| x != &0) {
        ans.0 += ns[ns.len() - 1];
        ans.1 += mults.next().unwrap() * ns[0];
        (0..ns.len() - 1).for_each(|i| ns[i] = ns[i + 1] - ns[i]);
        ns.pop();
    }
    ans
}
fn solve_with_poly(ns: &[i64]) -> (i64,i64) {
    let mut pd = PolynomialDetector::with_capacity(ns.len());
    for &n in ns {
        pd.add(n);
        if pd.get_certainty_and_power().certainty > 3 {
            break;
        }
    }
    let r = pd.get_equation();
    (r.evaluate(1 + ns.len() as i64), r.evaluate(0))
}

fn with_poly_detector(input: &[Vec<i64>]) -> (i64, i64) {
    input.iter()
    .map(|x| solve_with_poly(x))
    .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn single_thread(input: &[Vec<i64>]) -> (i64, i64) {
    input
        .iter()
        .cloned()
        .map(solve_line)
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn multi_thread(input: &[Vec<i64>]) -> (i64, i64) {
    input
        .to_vec()
        .into_par_iter()
        .map(solve_line)
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

const EG: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
