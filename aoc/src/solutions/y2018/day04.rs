aoc_harness::aoc_main!(2018 day 4, generator gen, part1 [part1], part2 [part2]);

use utils::nums::de_prefixsum;
#[allow(clippy::trivial_regex)]
#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Eq, Debug)]
pub enum LogEvent {
    #[display("Guard #{guard_id} begins shift")]
    BeginShift { guard_id: usize },
    #[display("falls asleep")]
    Asleep,
    #[display("wakes up")]
    Wakes,
}

#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Eq, Debug)]
#[display("[{date}:{minute}] {event}")]
pub struct LogLine {
    date: String,
    minute: usize,
    event: LogEvent,
}

#[cfg(test)]
const HINT_INPUT: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

fn gen(input: &str) -> Vec<LogLine> {
    let mut v: Vec<LogLine> = input
        .lines()
        .map(|x| x.parse().expect("Bad line"))
        .collect();
    v.sort_unstable_by(|e1, e2| (&e1.date, e1.minute).cmp(&(&e2.date, e2.minute)));
    v
}

#[test]
fn test_parse() {
    assert_eq!(gen(HINT_INPUT).len(), 17);
}

fn part1(input: &[LogLine]) -> usize {
    let mut mins_per_guard: std::collections::HashMap<usize, Vec<isize>> =
        std::collections::HashMap::new();
    let mut guard_id: usize = 0;
    for i in input {
        match i.event {
            LogEvent::BeginShift { guard_id: g_id } => {
                mins_per_guard.entry(g_id).or_insert_with(|| vec![0; 60]);
                guard_id = g_id;
            }
            LogEvent::Asleep => mins_per_guard.get_mut(&guard_id).unwrap()[i.minute] += 1,
            LogEvent::Wakes => mins_per_guard.get_mut(&guard_id).unwrap()[i.minute] -= 1,
        }
    }
    // find the sleepiest guard, g.
    let converted: std::collections::HashMap<usize, Vec<isize>> = mins_per_guard
        .iter()
        .map(|(k, v)| (*k, de_prefixsum(v)))
        .collect();
    let (g, _) = converted
        .iter()
        .max_by_key(|kvp| kvp.1.iter().sum::<isize>())
        .expect("No lines?");
    //what time was guard g most asleep?

    let v = &converted[g];
    let (m, _) = v
        .iter()
        .enumerate()
        .max_by_key(|x| x.1)
        .expect("no minutes");
    *g * m
}

fn part2(input: &[LogLine]) -> usize {
    let mut hm = std::collections::HashMap::new();
    let mut guard_id = 0;
    let mut asleep_at = 0;
    for i in input {
        match i.event {
            LogEvent::BeginShift { guard_id: g_id } => guard_id = g_id,
            LogEvent::Asleep => asleep_at = i.minute,
            LogEvent::Wakes => {
                for i in asleep_at..i.minute {
                    *hm.entry((guard_id, i)).or_insert(0) += 1;
                }
            }
        }
    }
    let ((g, m), _) = hm.iter().max_by_key(|x| x.1).expect("no lines");
    *g * *m
}

#[test]
fn test_part1() {
    assert_eq!(part1(&gen(HINT_INPUT)), 10 * 24);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&gen(HINT_INPUT)), 99 * 45);
}
