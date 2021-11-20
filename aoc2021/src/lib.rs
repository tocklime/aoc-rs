use std::path::PathBuf;

use aocf::Aoc;
use structopt::StructOpt;

const YEAR: i32 = 2020; //Will be 2021 when we get going, but I want to see inputs now!

pub fn get_day(src_file: &str) -> Day {
    let p = std::path::Path::new(src_file);
    let day: u32 = p
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_matches(char::is_alphabetic)
        .parse()
        .expect("No single number in binary filename, can't determine day");
    let options = Opts::from_args();

    let input = options.get_input(day);

    Day {
        input,
        day,
        options,
    }
}
#[macro_export]
macro_rules! aoc_no_gen {
    () => {{
        let f: &'static str = file!();
        let s = aoc2021::get_day(f);
        (s, std::borrow::Cow::Borrowed(&s.input))
    }};
}
#[macro_export]
macro_rules! aoc_gen {
    ($fn:ident) => {{
        let f: &'static str = file!();
        let s = aoc2021::get_day(f);
        let generated = s.time_fn("Ran", "generator", || $fn(&s.input));
        (s, std::borrow::Cow::Owned(generated))
    }};
}
#[macro_export]
macro_rules! aoc_part {
    ($pd:ident, $n:literal, $($solve_fn:ident),*) => {
        {
            $(
                let fn_name = stringify!($solve_fn);
                let a = $pd.0.time_fn("Solved", fn_name, || $solve_fn(std::convert::AsRef::as_ref(&$pd.1)));
                println!("{}", a);
            )*
        }
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

pub struct Day {
    pub input: String,
    pub day: u32,
    pub options: Opts,
}
pub type ParsedDay<T> = (Day, T);
impl Day {
    pub fn time_fn<O, F>(&self, verb: &str, name: &str, f: F) -> O
    where
        F: FnOnce() -> O,
    {
        let (t, a) = time::Duration::time_fn(f);
        println!("{} {} in {}", verb, name, render_duration(t));
        a
    }
}

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Do full benchmarks for each part
    #[structopt(short, long)]
    bench: bool,
    /// Override the input with the contents of this file
    #[structopt(short, long)]
    input: Option<PathBuf>,
}

impl Opts {
    fn get_input(&self, day: u32) -> String {
        match &self.input {
            None => {
                let mut aoc = Aoc::new()
                    .year(Some(YEAR)) //Will be 2021 when we get going, but I want to see inputs now!
                    .day(Some(day))
                    .parse_cli(false)
                    .init()
                    .unwrap();
                aoc.get_input(false)
                    .expect("Couldn't get input for day from adventofcode.com.")
            }
            Some(f) => std::fs::read_to_string(f).expect("Couldn't read file"),
        }
    }
}
