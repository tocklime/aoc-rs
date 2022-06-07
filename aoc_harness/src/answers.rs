use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::dayresult::DayResult;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Answer {
    part1: Option<String>,
    part1_confirmed: bool,
    part2: Option<String>,
    part2_confirmed: bool,
}

pub type AnswerYear = BTreeMap<u8, Answer>;
#[derive(Debug)]
pub struct AnswerAll {
    data: BTreeMap<i32, AnswerYear>,
}
impl AnswerAll {
    #[must_use]
    pub fn from_file() -> Self {
        let data = match std::fs::File::open("answers.yaml") {
            Ok(f) => serde_yaml::from_reader(f).expect("Bad yaml format in answers.yaml"),
            Err(_) => BTreeMap::new(),
        };
        Self { data }
    }
    #[must_use]
    pub fn blank() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
    pub fn record_dayresult(&mut self, dr: &DayResult) -> Result<(), String> {
        let me = self
            .data
            .entry(dr.year)
            .or_default()
            .entry(dr.day)
            .or_default();
        match (&me.part1, &dr.part1_ans) {
            (Some(f), Some(e)) if f != e && me.part1_confirmed => {
                return Err(format!(
                    "actual part 1 result {} doesn't match expected {}",
                    e, f
                ));
            }
            _ => {}
        }
        me.part1 = dr.part1_ans.clone();
        me.part1_confirmed = dr.part1_confirmed;
        match (&me.part2, &dr.part2_ans) {
            (Some(f), Some(e)) if f != e && me.part2_confirmed => {
                return Err(format!(
                    "actual part 2 result {} doesn't match expected {}",
                    e, f
                ));
            }
            _ => {}
        }
        me.part2 = dr.part2_ans.clone();
        me.part2_confirmed = dr.part2_confirmed;
        Ok(())
    }
}

impl Default for AnswerAll {
    fn default() -> Self {
        Self::from_file()
    }
}
impl Drop for AnswerAll {
    fn drop(&mut self) {
        let f = std::fs::File::options()
            .write(true)
            .create(true)
            .open("answers.yaml")
            .expect("could not open answers.yaml");
        serde_yaml::to_writer(f, &self.data).expect("Failed serializing answers.yaml");
    }
}
