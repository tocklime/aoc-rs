#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use utils::algorithms::{automata_step, automata_step_mut};
use utils::points::Point;
use aoc_harness::aoc_main;
use itertools::iterate;
use num::pow;
use std::collections::HashSet;
use std::convert::TryInto;
use clap::Parser;
use dotenv;
pub fn run_with_opts(
    results: &mut aoc_harness::dayresult::DayResult,
    opts: &mut aoc_harness::Opts,
) {
    let basename = std::path::Path::new(
        "\\\\?\\C:\\Users\\greg\\Documents\\GitHub\\aoc-rs\\aoc\\src\\solutions\\y2019\\day24.rs",
    )
    .file_name()
    .unwrap();
    let desc = {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &["Year ", " Day ", " in "],
            &[
                ::core::fmt::ArgumentV1::new_display(&2019i32),
                ::core::fmt::ArgumentV1::new_display(&24u8),
                ::core::fmt::ArgumentV1::new_debug(&basename),
            ],
            &[
                ::core::fmt::rt::v1::Argument {
                    position: 0usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 0u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Implied,
                    },
                },
                ::core::fmt::rt::v1::Argument {
                    position: 1usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 8u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Is(2usize),
                    },
                },
                ::core::fmt::rt::v1::Argument {
                    position: 2usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 0u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Implied,
                    },
                },
            ],
            unsafe { ::core::fmt::UnsafeArg::new() },
        ));
        res
    };
    let s: String = match opts.get_input(2019i32, 24u8) {
        Ok(s) => s,
        Err(i) => {
            if i != aoc_harness::InputFetchFailure::PuzzleNotReleasedYet {
                opts.log(|| {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Missing input for ", ": "],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&&desc),
                            ::core::fmt::ArgumentV1::new_debug(&i),
                        ],
                    ));
                    res
                });
            }
            return;
        }
    };
    let (t, generated) = opts.time_fn(|| gen(&s));
    results.record_generator(t);
    opts.log(|| {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " generated in "],
            &[
                ::core::fmt::ArgumentV1::new_display(&&desc),
                ::core::fmt::ArgumentV1::new_display(&aoc_harness::render_duration(t)),
            ],
        ));
        res
    });
    let expected = 32_509_983;
    ::aoc_harness::type_hint_value_has_same_type_as_func_return(&expected, &p1);
    results.expect_p1(expected);
    let solver_name = "p1";
    let full_name = {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " ", " via `", "`"],
            &[
                ::core::fmt::ArgumentV1::new_display(&&desc),
                ::core::fmt::ArgumentV1::new_display(&"Part 1"),
                ::core::fmt::ArgumentV1::new_display(&solver_name),
            ],
        ));
        res
    };
    let (t, a) = opts.time_fn(|| p1(&generated));
    let a = match aoc_harness::answertype::AnswerType::to_option(a) {
        Some(x) => x,
        None => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["", " failed to produce an answer"],
            &[::core::fmt::ArgumentV1::new_display(&&full_name)],
        )),
    };
    results.record_p1(Some(a.clone()), t);
    opts.log(|| {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " solved in ", ": "],
            &[
                ::core::fmt::ArgumentV1::new_display(&&full_name),
                ::core::fmt::ArgumentV1::new_display(&aoc_harness::render_duration(t)),
                ::core::fmt::ArgumentV1::new_debug(&a),
            ],
        ));
        res
    });
    let expected = 2012;
    ::aoc_harness::type_hint_value_has_same_type_as_func_return(&expected, &p2m);
    opts.log(|| {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " ", " expected result: "],
            &[
                ::core::fmt::ArgumentV1::new_display(&desc),
                ::core::fmt::ArgumentV1::new_display(&"Part 2"),
                ::core::fmt::ArgumentV1::new_debug(&expected),
            ],
        ));
        res
    });
    results.expect_p2(expected);
    let solver_name = "p2m";
    let full_name = {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " ", " via `", "`"],
            &[
                ::core::fmt::ArgumentV1::new_display(&&desc),
                ::core::fmt::ArgumentV1::new_display(&"Part 2"),
                ::core::fmt::ArgumentV1::new_display(&solver_name),
            ],
        ));
        res
    };
    let (t, a) = opts.time_fn(|| p2m(&generated));
    let a = match aoc_harness::answertype::AnswerType::to_option(a) {
        Some(x) => x,
        None => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["", " failed to produce an answer"],
            &[::core::fmt::ArgumentV1::new_display(&&full_name)],
        )),
    };
    results.record_p2(Some(a.clone()), t);
    opts.log(|| {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " solved in "],
            &[
                ::core::fmt::ArgumentV1::new_display(&&full_name),
                ::core::fmt::ArgumentV1::new_display(&aoc_harness::render_duration(t)),
            ],
        ));
        res
    });
    let solver_name = "p2c";
    let full_name = {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " ", " via `", "`"],
            &[
                ::core::fmt::ArgumentV1::new_display(&&desc),
                ::core::fmt::ArgumentV1::new_display(&"Part 2"),
                ::core::fmt::ArgumentV1::new_display(&solver_name),
            ],
        ));
        res
    };
    let (t, a) = opts.time_fn(|| p2c(&generated));
    let a = match aoc_harness::answertype::AnswerType::to_option(a) {
        Some(x) => x,
        None => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["", " failed to produce an answer"],
            &[::core::fmt::ArgumentV1::new_display(&&full_name)],
        )),
    };
    results.record_p2(Some(a.clone()), t);
    opts.log(|| {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", " solved in "],
            &[
                ::core::fmt::ArgumentV1::new_display(&&full_name),
                ::core::fmt::ArgumentV1::new_display(&aoc_harness::render_duration(t)),
            ],
        ));
        res
    });
    opts.answers
        .record_dayresult(results)
        .expect("Mismatched results");
}
fn check_examples() {}
#[allow(dead_code)]
pub fn run_main() -> aoc_harness::dayresult::DayResult {
    let mut opts = aoc_harness::Opts::parse();
    check_examples();
    let mut results = aoc_harness::dayresult::DayResult::new(2019i32, 24u8);
    for _ in 0..opts.repeats {
        run_with_opts(&mut results, &mut opts);
    }
    results
}
#[must_use]
pub fn gen(input: &str) -> HashSet<Point> {
    let hm = utils::points::as_point_map(input);
    hm.iter()
        .filter_map(|(a, b)| if b == &'#' { Some(*a) } else { None })
        .collect()
}
pub fn p1(input: &HashSet<Point>) -> usize {
    let mut seen = HashSet::new();
    iterate(input.clone(), |g| automata_step(g, flat_neighbours, lives))
        .map(|x| biodiversity(&x))
        .find(|&x| !seen.insert(x))
        .unwrap()
}
pub fn p2m(input: &HashSet<Point>) -> usize {
    let mut g: HashSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    for _ in 0..200 {
        automata_step_mut(&mut g, recur_neighbours, lives);
    }
    g.len()
}
pub fn p2c(input: &HashSet<Point>) -> usize {
    let mut g: HashSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    for _ in 0..200 {
        g = automata_step(&g, recur_neighbours, lives);
    }
    g.len()
}
pub fn flat_neighbours(p: Point) -> Vec<Point> {
    p.neighbours()
        .iter()
        .cloned()
        .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < 5 && p.1 < 5)
        .collect()
}
pub fn recur_neighbours(p: (Point, i32)) -> Vec<(Point, i32)> {
    let mut ans = Vec::with_capacity(8);
    flat_neighbours(p.0)
        .into_iter()
        .filter(|x| x != &Point(2, 2))
        .for_each(|x| ans.push((x, p.1)));
    match (p.0).0 {
        0 => ans.push((Point(1, 2), p.1 + 1)),
        4 => ans.push((Point(3, 2), p.1 + 1)),
        _ => (),
    };
    match (p.0).1 {
        0 => ans.push((Point(2, 1), p.1 + 1)),
        4 => ans.push((Point(2, 3), p.1 + 1)),
        _ => (),
    };
    match p.0 {
        Point(2, 1) => (0..5).for_each(|x| ans.push((Point(x, 0), p.1 - 1))),
        Point(1, 2) => (0..5).for_each(|x| ans.push((Point(0, x), p.1 - 1))),
        Point(3, 2) => (0..5).for_each(|x| ans.push((Point(4, x), p.1 - 1))),
        Point(2, 3) => (0..5).for_each(|x| ans.push((Point(x, 4), p.1 - 1))),
        _ => (),
    };
    ans
}
#[must_use]
pub fn lives(is_alive: bool, neighbour_count: usize) -> bool {
    neighbour_count == 1 || (!is_alive && neighbour_count == 2)
}
pub fn biodiversity(g: &HashSet<Point>) -> usize {
    g.iter()
        .map(|&p| pow(2, (p.0 + p.1 * 5).try_into().unwrap()))
        .sum()
}
pub fn main() {
    dotenv::dotenv().ok();
    run_main();
}
