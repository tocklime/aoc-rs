mod day1;

use std::path::PathBuf;

use advent_of_code_traits::Solution;
use aocf::Aoc;
use seq_macro::seq;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    /// run just one day
    #[structopt(short, long)]
    day: Option<u32>,
    /// Benchmark each day
    #[structopt(short, long)]
    bench: bool,
    #[structopt(short, long)]
    input: Option<PathBuf>,
}

struct AdventOfCode2021;

fn get_input(day: u32) -> String {
    let mut aoc = Aoc::new()
        .year(Some(2020)) //Will be 2021 when we get going, but I want to see inputs now!
        .day(Some(day))
        .parse_cli(false)
        .init()
        .unwrap();
    aoc.get_input(false).expect("Couldn't get input for day.")
}

macro_rules! create_do_day {
    ($max:literal) => {
        const IMPLEMENTED_TO: u32 = $max;
        fn do_day(n: u32, _bench: bool) {
            let input = get_input(n.into());
            seq!(N in 1..=$max {
                match n {
                    #(N => <AdventOfCode2021 as Solution<N>>::run(&input),)*
                    _ => unimplemented!()
                }
            })
        }
    };
}
create_do_day!(1);

fn main() {
    let opts = Opts::from_args();
    match opts.day {
        None => {
            for x in 1..=IMPLEMENTED_TO {
                do_day(x, opts.bench);
            }
        }
        Some(d) => {
            do_day(d, opts.bench);
        }
    }
}
