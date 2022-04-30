use std::{fmt::Display, time::Duration};

use crate::render_duration;

enum ExecutionTime {
    NoneRecorded,
    Part1(Duration),
    Part2(Duration),
    Separate(Duration, Duration),
    Both(Duration),
}
pub struct DayResult {
    year: i32,
    day: u8,
    name: &'static str,
    generator_time: Option<Duration>,
    solve_time: ExecutionTime,
    part1_ans: Option<String>,
    part2_ans: Option<String>,
}

impl DayResult {
    pub fn new(year: i32, day: u8, name: &'static str) -> Self {
        Self {
            year,
            day,
            name,
            generator_time: None,
            solve_time: ExecutionTime::NoneRecorded,
            part1_ans: None,
            part2_ans: None,
        }
    }
    pub fn output_line(&self) -> String {
        let t = match self.solve_time {
            ExecutionTime::NoneRecorded => "Not run".to_string(),
            ExecutionTime::Part1(a) => format!("p1:{}", render_duration(a)),
            ExecutionTime::Part2(b) => format!("p2:{}", render_duration(b)),
            ExecutionTime::Separate(a, b) => {
                format!("p1:{}\tp2:{}", render_duration(a), render_duration(b))
            }
            ExecutionTime::Both(b) => format!("both: {}", render_duration(b)),
        };
        format!(
            "{}\tgen: {}\t{}\t{}\t{}",
            self.desc(),
            self.generator_time
                .map(render_duration)
                .unwrap_or_else(|| "N/A".to_owned()),
            t,
            self.part1_ans.as_ref().unwrap_or(&"N/A".to_owned()),
            self.part2_ans.as_ref().unwrap_or(&"N/A".to_owned())
        )
    }
    fn desc(&self) -> String {
        format!("Year {} day {:02} {}", self.year, self.day, self.name)
    }
    pub fn record_generator(&mut self, t: Duration) {
        match &self.generator_time {
            Some(a) if *a <= t => {}
            _ => self.generator_time = Some(t),
        }
    }
    fn record_ans<T: Display>(part_num: u8, slot: &mut Option<String>, ans: T) -> Result<(), String> {
        let ans = format!("{}", ans);
        match slot {
            Some(s) if s != &ans => {
                Err(format!("conflicting results for part {}: {} and {}", part_num, s, ans))
            }
            None => {
                *slot = Some(ans);
                Ok(())
            }
            _ => Ok(()),
        }
    }
    pub fn expect_p1<T: Display>(&mut self, s: T) {
        self.part1_ans = Some(format!("{}", s));
    }
    pub fn expect_p2<T: Display>(&mut self, s: T) {
        self.part2_ans = Some(format!("{}", s));
    }
    pub fn record_p1<T: Display>(&mut self, ans: T, time: Duration) {
        if let Err(x) = Self::record_ans(1, &mut self.part1_ans, ans) {
            panic!("{} {}", self.desc(), x);
        }
        match &mut self.solve_time {
            ExecutionTime::NoneRecorded => self.solve_time = ExecutionTime::Part1(time),
            ExecutionTime::Part2(b) => self.solve_time = ExecutionTime::Separate(time, *b),
            ExecutionTime::Part1(p) if *p > time => *p = time,
            ExecutionTime::Separate(a, _) if *a > time => *a = time,
            ExecutionTime::Both(_) => {} //assume 'both' times are better.
            _ => {}
        }
    }
    pub fn record_p2<T: Display>(&mut self, ans: T, time: Duration) {
        if let Err(x) = Self::record_ans(2, &mut self.part2_ans, ans) {
            panic!("{} {}", self.desc(), x);
        }
        match &mut self.solve_time {
            ExecutionTime::NoneRecorded => self.solve_time = ExecutionTime::Part2(time),
            ExecutionTime::Part1(a) => self.solve_time = ExecutionTime::Separate(*a, time),
            ExecutionTime::Part2(p) if *p > time => *p = time,
            ExecutionTime::Separate(_, b) if *b > time => *b = time,
            ExecutionTime::Both(_) => {} //assume 'both' times are better.
            _ => {}
        }
    }
    pub fn record_both<T: Display, T2: Display>(&mut self, ans: (T, T2), time: Duration) {
        if let Err(x) = Self::record_ans(1, &mut self.part1_ans, ans.0) {
            panic!("{} {}", self.desc(), x);
        }
        if let Err(x) = Self::record_ans(2, &mut self.part2_ans, ans.1) {
            panic!("{} {}", self.desc(), x);
        }
        match &mut self.solve_time {
            ExecutionTime::Both(b) => {
                if *b > time {
                    *b = time;
                }
            }
            _ => self.solve_time = ExecutionTime::Both(time),
        }
    }
    pub fn total_time(&self) -> Duration {
        self.generator_time.unwrap_or(Duration::ZERO)
            + match self.solve_time {
                ExecutionTime::Both(b) => b,
                ExecutionTime::NoneRecorded => Duration::ZERO,
                ExecutionTime::Part1(a) => a,
                ExecutionTime::Separate(a, b) => a + b,
                ExecutionTime::Part2(b) => b,
            }
    }
}
