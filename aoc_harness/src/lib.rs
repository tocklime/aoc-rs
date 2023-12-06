//Convenience re-exports

pub mod answers;
pub mod answertype;
pub mod dayresult;

use std::{
    env,
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

use answers::AnswerAll;
use answertype::AnswerType;
pub use aoc_harness_macros::*;
use chrono::TimeZone;
use clap::arg;
pub use itertools::Itertools;

#[derive(clap::Parser, Debug)]
pub struct Opts {
    /// Override the input with the contents of this file
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    pub quiet: bool,
    ///panic if results don't match expected.
    #[arg(short, long)]
    pub test_mode: bool,
    #[arg(short, long, default_value = "1")]
    pub repeats: usize,
    /// Bypass lightweight benchmarking
    #[arg(short, long)]
    pub bypass: bool,
    #[arg(skip)]
    pub answers: AnswerAll,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            input: None,
            quiet: false,
            test_mode: false,
            repeats: 1,
            bypass: false,
            answers: AnswerAll::from_file(),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum InputFetchFailure {
    CantReadCachedFile,
    CantWriteCachedFile,
    PuzzleNotReleasedYet,
    NoAdventOfCodeCookie,
    CantCreateDirectoryForYear,
    Unauthorized,
    HttpNotFound,
    SomethingElse(String),
}

/// This is used in macro code to hint to the compiler that the expected results
/// in the `aoc_main`! macro have the same type as the return type of a function.
/// but we also need to unwrap Result or Options, using the `AnswerType`.
pub fn type_hint_value_has_same_type_as_func_return<Value, Func, FuncIn, FuncOut, TAns>(
    _val: &Value,
    _func: &Func,
) where
    Func: Fn(FuncIn) -> FuncOut,
    FuncOut: AnswerType<Output = TAns>,
    TAns: PartialEq<Value>,
{
}
/// This is used in macro code to hint to the compiler that the expected result pairs
/// in the `aoc_main!` macro have the same type as the return type of a function.
/// but we also need to unwrap Result or Options, using the `AnswerType`.
pub fn type_hint_pair_has_values_in_func_return<V1, V2, Func, FuncIn, FuncOut, Ans1, Ans2>(
    _v1: &V1,
    _v2: &V2,
    _func: &Func,
) where
    Func: Fn(FuncIn) -> FuncOut,
    FuncOut: AnswerType<Output = (Ans1, Ans2)>,
    Ans1: PartialEq<V1>,
    Ans2: PartialEq<V2>,
{
}

fn user_agent() -> &'static str {
    "https://github.com/tocklime/aoc-rs"
}
impl Opts {
    const TARGET_DUR: Duration = Duration::from_millis(50);

    #[must_use]
    pub fn for_test() -> Self {
        Self {
            input: None,
            quiet: true,
            test_mode: true,
            repeats: 1,
            bypass: true,
            answers: AnswerAll::blank(),
        }
    }
    pub fn log<F: FnOnce() -> String>(&self, f: F) {
        if !self.quiet {
            println!("{}", f());
        }
    }
    pub fn assert_eq<T1, T2>(&self, actual: &T1, expected: &T2)
    where
        T1: std::fmt::Debug + PartialEq<T2>,
        T2: std::fmt::Debug,
    {
        if self.test_mode {
            assert_eq!(actual, expected);
        } else if actual != expected {
            self.log(|| format!("!!! Answer does not match expected: {actual:?} != {expected:?}"));
        }
    }
    pub fn get_input(&self, year: i32, day: u8) -> Result<String, InputFetchFailure> {
        match &self.input {
            None => {
                //try in cache dir first.
                let p = PathBuf::from(format!(
                    "{}aoc-inputs/{}/day{:02}.txt",
                    std::env::var("CARGO_WORKSPACE_DIR").unwrap_or_else(|_| String::new()),
                    year,
                    day
                ));
                if p.exists() {
                    Ok(std::fs::read_to_string(p)
                        .map_err(|_| InputFetchFailure::CantReadCachedFile)?
                        .replace('\r', ""))
                } else {
                    let now = chrono::Utc::now();
                    let release_date = chrono::Utc
                        .with_ymd_and_hms(year, 12, day.into(), 5, 0, 0)
                        .unwrap();
                    if release_date > now {
                        return Err(InputFetchFailure::PuzzleNotReleasedYet);
                    }
                    std::fs::create_dir_all(p.parent().unwrap())
                        .map_err(|_| InputFetchFailure::CantCreateDirectoryForYear)?;
                    let i = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
                        .set("User-Agent", user_agent())
                        .set(
                            "cookie",
                            &format!(
                                "session={}",
                                env::var("AOC_SESSION").expect("AOC_SESSION env var not set")
                            ),
                        )
                        .call()
                        .map_err(|e| match e {
                            ureq::Error::Status(s, r) => match s {
                                404 => InputFetchFailure::HttpNotFound,
                                403 => InputFetchFailure::Unauthorized,
                                _ => InputFetchFailure::SomethingElse(format!(
                                    "HTTP Error {s} fetching input: {r:?}"
                                )),
                            },
                            ureq::Error::Transport(t) => InputFetchFailure::SomethingElse(format!(
                                "HTTP Transport error: {t}"
                            )),
                        })?
                        .into_string()
                        .expect("Failed to convert HTTP response to string");
                    std::fs::write(p, &i).map_err(|_| InputFetchFailure::CantWriteCachedFile)?;
                    Ok(i)
                }
            }
            Some(f) => Ok(std::fs::read_to_string(f)
                .map_err(|_| InputFetchFailure::CantReadCachedFile)?
                .replace('\r', "")),
        }
    }
    pub fn time_fn<O, F>(&self, f: F) -> (std::time::Duration, O)
    where
        F: Fn() -> O,
    {
        let start = Instant::now();
        let ans = f();
        let end = Instant::now();
        let dur = end - start;
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )] //it's only for reporting the time.
        if !self.bypass && dur < Self::TARGET_DUR {
            let bench =
                benchmarking::bench_function_with_duration(Self::TARGET_DUR, move |measurer| {
                    measurer.measure(&f);
                })
                .unwrap();
            let overall = bench.elapsed().as_secs_f64();
            //took less than 5ms. How many could we do in 5ms?
            // let min_time = (0..10).map(|_| {
            //     let start = Instant::now();
            //     for _ in 0..c {
            //         f();
            //     }
            //     let end = Instant::now();
            //     end - start
            // }).min().unwrap();
            // let overall = (min_time).as_secs_f64() / (c as f64);
            (std::time::Duration::from_secs_f64(overall), ans)
        } else {
            (dur, ans)
        }
    }
}

#[must_use]
pub fn appropriate_scale(d: std::time::Duration) -> (u64, &'static str) {
    let value = d.as_secs_f64();
    let units = ["s", "ms", "\u{3bc}s", "ns"];
    let mut scale = 1;
    for u in units {
        #[allow(clippy::cast_precision_loss)]
        if (scale as f64 * value) > 1.0 {
            return (scale, u);
        }
        scale *= 1_000;
    }
    (1_000_000_000, "ns")
}

#[must_use]
pub fn render_duration(d: std::time::Duration) -> String {
    let (scale, suffix) = appropriate_scale(d);
    #[allow(clippy::cast_precision_loss)]
    if suffix == "ns" {
        format!("{:.0}{}", scale as f64 * d.as_secs_f64(), suffix)
    } else {
        format!("{:.3}{}", scale as f64 * d.as_secs_f64(), suffix)
    }
}

#[must_use]
pub fn whole_input_is<O>(i: &str) -> O
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.parse().unwrap()
}
#[must_use]
pub fn lines<O>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.trim().lines().map(|x| x.parse().unwrap()).collect()
}
#[must_use]
pub fn input<O, const S: char>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.trim().split(S).map(|x| x.parse().unwrap()).collect()
}
