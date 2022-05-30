include!(aoc_harness::find_most_recent!("aoc/src/solutions", "day"));

pub fn main() {
    dotenv::dotenv().ok();
    run_main();
}
