//Convenience re-exports

use std::{path::PathBuf, str::FromStr};

pub use aoc_harness_macros::*;
pub use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
pub struct Opts {
    /// Override the input with the contents of this file
    #[structopt(short, long)]
    input: Option<PathBuf>,
    #[structopt(short, long)]
    quiet: bool,
    ///panic if results don't match expected.
    #[structopt(short, long)]
    test_mode: bool,
}

impl Opts {
    pub fn for_test() -> Self {
        Self {
            input: None,
            quiet: true,
            test_mode: true,
        }
    }
    pub fn log<F: FnOnce() -> String>(&self, f: F) {
        if !self.quiet {
            println!("{}", f());
        }
    }
    pub fn assert_eq<T: Eq + core::fmt::Debug>(&self, actual: T, expected: T, print_actual: bool) {
        if self.test_mode {
            assert_eq!(actual, expected);
        } else if actual != expected {
            if print_actual {
                self.log(|| format!("!!! Answer does not match expected"));
            } else {
                self.log(|| format!("!!! Answer does not match expected: {:?}", actual));
            }
        }
    }
    pub fn get_input(&self, year: i32, day: u8) -> String {
        match &self.input {
            None => {
                let mut aoc = aocf::Aoc::new()
                    .year(Some(year))
                    .day(Some(day.into()))
                    .parse_cli(false)
                    .init()
                    .unwrap();
                aoc.get_input(false)
                    .expect("Couldn't get input for day from adventofcode.com.")
            }
            Some(f) => std::fs::read_to_string(f).expect("Couldn't read file"),
        }
    }
    pub fn time_fn<O, F>(&self, f: F) -> (time::Duration, O)
    where
        F: FnOnce() -> O,
    {
        time::Duration::time_fn(f)
    }
}

pub fn render_duration(d: time::Duration) -> String {
    let (n, unit) = if d.whole_seconds() > 0 {
        (d.whole_seconds(), "s")
    } else if d.subsec_milliseconds() > 0 {
        (d.subsec_milliseconds().into(), "ms")
    } else if d.subsec_microseconds() > 0 {
        (d.subsec_microseconds().into(), "µs")
    } else {
        (d.subsec_nanoseconds().into(), "ns")
    };
    format!("{} {}", n, unit)
}

pub fn lines<O>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.lines().map(|x| x.parse().unwrap()).collect()
}
pub fn input<O: FromStr, const S: char>(i: &str) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    i.split(S).map(|x| x.parse().unwrap()).collect()
}
