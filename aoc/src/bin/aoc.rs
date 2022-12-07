use std::{collections::BTreeMap, time::Duration};

use aoc_harness::{aoc_all_main, dayresult::DayResult, dayresult::ExecutionTime, Opts};
use clap::Parser;
type Day = ((i32, u8), fn(&mut DayResult, &mut Opts));

aoc_all_main!("aoc/src/solutions");

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
    let mut opts = aoc_harness::Opts::default();
    let mut results: BTreeMap<(i32, u8), DayResult> = BTreeMap::new();
    for ((year, day), f) in all {
        if (args.year.is_none() || args.year == Some(year))
            && (args.day.is_none() || args.day == Some(day))
        {
            let dr = results
                .entry((year, day))
                .or_insert_with(|| DayResult::new(year, day));
            f(dr, &mut opts);
        }
    }
    let mut total_time = Duration::ZERO;
    let mut time_per_year: BTreeMap<i32, Duration> = BTreeMap::new();
    for v in results.values() {
        let d = v.total_time();
        total_time += d;
        *time_per_year.entry(v.year).or_default() += d;
    }

    for (y, d) in &time_per_year {
        println!("Time for year {}: {}", y, aoc_harness::render_duration(*d));
    }
    if time_per_year.len() > 1 {
        println!("Total time: {}", aoc_harness::render_duration(total_time));
    }
}
