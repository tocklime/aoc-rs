use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc_harness::*;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::many1,
    sequence::tuple,
    IResult,
};
use utils::{aabb::Aabb, cartesian::Point, span::Span};

aoc_main!(2022 day 15, part1 [p1] => 5607466, part2 [p2, dividing_quadrants] => 12543202766584, example both EG => (26, 56000011));

const EG: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

#[derive(Debug)]
struct Sensor {
    location: Point<i64>,
    closest_beacon: Point<i64>,
    range: i64,
}
impl Sensor {
    fn can_see(&self, point: Point<i64>) -> bool {
        (self.location - point).manhattan() <= self.range
    }
    fn can_see_all(&self, bb: Aabb<i64>) -> bool {
        bb.corners_inclusive().into_iter().all(|x| self.can_see(x))
    }
    fn shadow_y(&self, target: i64) -> Option<Span<i64>> {
        let clear_range = self.range;
        let delta = (self.location.y - target).abs();
        let shadow_size = clear_range * 2 + 1 - 2 * delta;
        if shadow_size > 0 {
            let shadow_start = self.location.x - clear_range + delta;
            let shadow = Span::new_from_range(shadow_start..shadow_start + shadow_size);
            Some(shadow)
        } else {
            None
        }
    }
    fn shadow_x(&self, target: i64) -> Option<Span<i64>> {
        let clear_range = (self.closest_beacon - self.location).manhattan();
        let delta = (self.location.x - target).abs();
        let shadow_size = clear_range * 2 + 1 - 2 * delta;
        if shadow_size > 0 {
            let shadow_start = self.location.y - clear_range + delta;
            let shadow = Span::new_from_range(shadow_start..shadow_start + shadow_size);
            Some(shadow)
        } else {
            None
        }
    }
}
fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, (_, xa, _, ya, _, xb, _, yb, _)) = tuple((
        tag("Sensor at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        tag(": closest beacon is at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        newline,
    ))(input)?;
    let location = Point::new(xa, ya);
    let closest_beacon = Point::new(xb, yb);
    Ok((
        input,
        Sensor {
            location,
            closest_beacon,
            range: (location - closest_beacon).manhattan(),
        },
    ))
}

fn p1(input: &str) -> i64 {
    let (_, sensors) = all_consuming(many1(parse_sensor))(input).unwrap();
    let target_y = if sensors[0].location.x == 2 {
        10
    } else {
        2000000
    };
    let mut shadows: BinaryHeap<_> = sensors
        .iter()
        .filter_map(|x| x.shadow_y(target_y).map(Reverse))
        .collect();
    let mut count = 0;
    let mut cur_span = shadows.pop().unwrap().0;

    while let Some(Reverse(s)) = shadows.pop() {
        if s.start > cur_span.end {
            //gap!
            cur_span = s;
            count += s.size();
        } else {
            cur_span = s.union(&cur_span);
        }
    }
    let y_beacons: HashSet<Point<i64>> = sensors
        .iter()
        .map(|l| l.closest_beacon)
        .filter(|l| l.y == target_y)
        .collect();
    count += cur_span.size();
    count - y_beacons.len() as i64
}

fn has_gap<F>(sensors: &[Sensor], full_size: i64, get_shadow: F) -> bool
where
    F: Fn(&Sensor) -> Option<Span<i64>>,
{
    let mut shadows: BinaryHeap<_> = sensors
        .iter()
        .filter_map(|x| get_shadow(x).map(Reverse))
        .collect();

    let mut cur_span = shadows.pop().unwrap().0;
    while let Some(Reverse(s)) = shadows.pop() {
        if s.start > cur_span.end {
            return true;
        } else {
            cur_span = cur_span.union(&s);
        }
    }
    cur_span.start > 0 || cur_span.end <= full_size
}

fn p2(input: &str) -> i64 {
    let (_, sensors) = all_consuming(many1(parse_sensor))(input).unwrap();
    let max_coord = if sensors[0].location.x == 2 {
        20
    } else {
        4000000
    };
    let found_y = (0..=max_coord).find(|&y| has_gap(&sensors, max_coord, |p| p.shadow_y(y)));
    let found_x = (0..=max_coord).find(|&x| has_gap(&sensors, max_coord, |p| p.shadow_x(x)));
    4000000 * found_x.unwrap() + found_y.unwrap()
}

fn dividing_quadrants(input: &str) -> i64 {
    let (_, sensors) = all_consuming(many1(parse_sensor))(input).unwrap();
    let max_coord = if sensors[0].location.x == 2 {
        20
    } else {
        4000000
    };
    let bb: Aabb<i64> = [Point::new(0, 0), Point::new(max_coord, max_coord)]
        .iter()
        .collect();
    let mut to_search = vec![bb];
    while let Some(x) = to_search.pop() {
        if sensors.iter().any(|s| s.can_see_all(x)) {
            //covered by some sensor.
        } else if x.area() == 1 {
            return x.bottom_left.x * 4000000 + x.bottom_left.y;
        } else if let Some(new) = x.quadrants() {
            to_search.extend(new);
        }
    }
    unreachable!()
}
