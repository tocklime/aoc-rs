use std::{collections::BTreeMap, time::Duration};

use aoc_harness::{aoc_all_main, dayresult::DayResult, Itertools, Opts};
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
    let mut results: BTreeMap<i32, BTreeMap<u8, Vec<DayResult>>> = BTreeMap::new();
    for ((year, day), f) in all {
        if (args.year.is_none() || args.year == Some(year))
            && (args.day.is_none() || args.day == Some(day))
        {
            let mut dr = DayResult::new(year, day);
            f(&mut dr, &mut opts);
            results
                .entry(year)
                .or_default()
                .entry(day)
                .or_default()
                .push(dr);
        }
    }

    let mut total_time = Duration::ZERO;
    for (&y, day_map) in &results {
        let best_times_by_day = day_map
            .values()
            .map(|d| d.iter().map(|dr| dr.total_time()).min().unwrap())
            .filter(|&d| d > Duration::ZERO)
            .collect_vec();
        let (scale, suffix) = best_times_by_day
            .iter()
            .map(|&d| aoc_harness::appropriate_scale(d))
            .min()
            .unwrap();
        let days_str = best_times_by_day
            .iter()
            .map(|d| format!("{:.3}", scale as f64 * d.as_secs_f64()))
            .join(" ");
        let this_year_time = best_times_by_day.iter().sum();
        total_time += this_year_time;
        println!(
            "Time for year {}: {} [{}] {}",
            y,
            aoc_harness::render_duration(this_year_time),
            days_str,
            suffix
        );
    }
    if results.len() > 1 {
        println!("Total time: {}", aoc_harness::render_duration(total_time));
    }
}
