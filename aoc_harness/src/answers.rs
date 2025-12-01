use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::dayresult::DayResult;

#[derive(Deserialize, Serialize, Debug, Default)]
enum AnswerPart {
    #[default]
    Unknown,
    Checked(String),
    Observed(String),
}
impl AnswerPart {
    fn from_option(ans: &Option<String>, confirmed: bool) -> Self {
        match (ans, confirmed) {
            (None, true) => panic!("Missing answer is confirmed"),
            (None, false) => AnswerPart::Unknown,
            (Some(x), true) => AnswerPart::Checked(x.clone()),
            (Some(x), false) => AnswerPart::Observed(x.clone()),
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Answer {
    part1: AnswerPart,
    part2: AnswerPart,
}

pub type AnswerYear = BTreeMap<u8, Answer>;
#[derive(Debug)]
pub struct AnswerAll {
    data: BTreeMap<i32, AnswerYear>,
    save_on_drop: bool,
}
impl AnswerAll {
    #[must_use]
    pub fn from_file() -> Self {
        let data = match std::fs::File::open("answers.ron") {
            Ok(f) => match ron::de::from_reader(&f) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("Failed to parse existing answers.ron, will overwrite: {err}");
                    BTreeMap::new()
                }
            },
            Err(_) => BTreeMap::new(),
        };
        Self {
            data,
            save_on_drop: true,
        }
    }
    #[must_use]
    pub fn blank() -> Self {
        Self {
            data: BTreeMap::new(),
            save_on_drop: false,
        }
    }
    pub fn record_dayresult(&mut self, dr: &DayResult) -> Result<(), String> {
        let me = self
            .data
            .entry(dr.year)
            .or_default()
            .entry(dr.day)
            .or_default();
        if let (AnswerPart::Checked(f), Some(e)) = (&me.part1, &dr.part1_ans)
            && f != e
        {
            return Err(format!(
                "actual part 1 result {e} doesn't match expected {f}"
            ));
        }
        me.part1 = AnswerPart::from_option(&dr.part1_ans, dr.part1_confirmed);
        if let (AnswerPart::Checked(f), Some(e)) = (&me.part2, &dr.part2_ans)
            && f != e
        {
            return Err(format!(
                "actual part 2 result {e} doesn't match expected {f}"
            ));
        }
        me.part2 = AnswerPart::from_option(&dr.part2_ans, dr.part2_confirmed);
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
        if self.save_on_drop {
            let f = std::fs::File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open("answers.new.ron")
                .expect("could not open answers.new.ron");
            let pc = PrettyConfig::new().depth_limit(2);
            let o = ron::options::Options::default();
            o.to_io_writer_pretty(f, &self.data, pc)
                .expect("Failed serialising answers.new.ron");
            std::fs::rename("answers.new.ron", "answers.ron")
                .expect("Failed to move answers.new.ron to answers.ron");
        }
    }
}
