use std::{collections::BTreeMap, time::Duration};

use aoc_harness::{dayresult::DayResult, dayresult::ExecutionTime, Opts};
use clap::Parser;
use seq_macro::seq;
type Day = ((i32, u8), fn(&mut DayResult, &mut Opts));
fn make_all() -> Vec<Day> {
    let mut ans: Vec<Day> = Vec::with_capacity(56);
    seq!(N in 01..=25 {
        ans.push(((2019, N), aoc::solutions::y2019::y19_d~N::run_with_opts));
    });
    seq!(N in 01..=05 {
        ans.push(((2020, N), aoc::solutions::y2020::day~N::run_with_opts));
    });
    seq!(N in 01..=25 {
        ans.push(((2021, N), aoc::solutions::y2021::y21_d~N::run_with_opts));
    });
    ans
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Only run this year
    #[clap(short, long)]
    year: Option<i32>,
    /// Only run this day
    #[clap(short, long)]
    day: Option<u8>,
}
pub fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let all = make_all();
    let mut times = Vec::new();
    let mut opts = Default::default();
    let mut total_time = Duration::ZERO;
    let mut time_per_year: BTreeMap<i32, Duration> = BTreeMap::new();
    for ((year, day), f) in all {
        if (args.year.is_none() || args.year == Some(year))
            && (args.day.is_none() || args.day == Some(day))
        {
            let mut dr = DayResult::new(year, day, "Name");
            f(&mut dr, &mut opts);
            let t = {
                let this = &dr;
                this.generator_time.unwrap_or(Duration::ZERO)
                    + match this.solve_time {
                        ExecutionTime::Both(b) => b,
                        ExecutionTime::NoneRecorded => Duration::ZERO,
                        ExecutionTime::Part1(a) => a,
                        ExecutionTime::Separate(a, b) => a + b,
                        ExecutionTime::Part2(b) => b,
                    }
            };
            total_time += t;
            *time_per_year.entry(year).or_default() += t;
            times.push(dr);
        }
    }

    for (y, d) in &time_per_year {
        println!("Time for year {}: {}", y, aoc_harness::render_duration(*d));
    }
    if time_per_year.len() > 1 {
        println!("Total time: {}", aoc_harness::render_duration(total_time));
    }
}
