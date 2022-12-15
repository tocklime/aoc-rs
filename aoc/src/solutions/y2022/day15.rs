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
use utils::{cartesian::Point, span::Span};

aoc_main!(2022 day 15, part1 [p1], part2 [p2], example both EG => (26, 56000011));

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
    Ok((
        input,
        Sensor {
            location: Point::new(xa, ya),
            closest_beacon: Point::new(xb, yb),
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
    let mut shadows = BinaryHeap::new();
    for l in &sensors {
        let clear_range = (l.closest_beacon - l.location).manhattan();
        //Sensor at x=9, y=16: closest beacon is at x=10, y=16

        //how much of y=y does this one span?
        //well, if it's /on/ y, then it's `clear_range * 2 + 1`.
        //if it's a long way away, then it's 0.
        //if it's one away from y, then it's clear_range * 2 + 1 - 2.
        //the shadow we cast on y=10 is max(0, clear_range * 2 + 1 - distance to y * 2)
        //centered on our x.
        let y_delta = (l.location.y - target_y).abs();
        let y_shadow_size = (clear_range * 2 + 1 - 2 * y_delta).max(0);
        if y_shadow_size > 0 {
            let y_shadow_start = l.location.x - clear_range + y_delta;
            let shadow = Span::new_from_range(y_shadow_start..y_shadow_start + y_shadow_size);
            shadows.push(Reverse(shadow));
        }
    }
    let mut count = 0;
    let mut cur_span = None;

    while let Some(Reverse(s)) = shadows.pop() {
        match cur_span {
            None => {
                cur_span = Some(s);
            }
            Some(c) => match c.collide_with(&s) {
                utils::span::CollisionType::OverlapsEnd(_, _, _) => unreachable!(),
                utils::span::CollisionType::Before(_) => {
                    cur_span = Some(s);
                    count += s.size();
                }
                utils::span::CollisionType::StrictlyBigger(_, _, _) => (),
                utils::span::CollisionType::Equal => (),
                utils::span::CollisionType::OverlapsStart(_, _, _) => {
                    cur_span = Some(s.union(&c));
                }
                utils::span::CollisionType::StrictlySmaller(_, _, _) => unreachable!(),
                utils::span::CollisionType::After(_) => unreachable!(),
            },
        }
    }
    let y_beacons: HashSet<Point<i64>> = sensors
        .iter()
        .map(|l| l.closest_beacon)
        .filter(|l| l.y == target_y)
        .collect();
    count += cur_span.unwrap().size();
    count - y_beacons.len() as i64
}

fn has_gap_x(sensors: &[Sensor], target_x: i64, full_size: i64) -> bool {
    let mut shadows = BinaryHeap::new();
    for l in sensors {
        let clear_range = (l.closest_beacon - l.location).manhattan();
        //Sensor at x=9, y=16: closest beacon is at x=10, y=16

        //how much of y=y does this one span?
        //well, if it's /on/ y, then it's `clear_range * 2 + 1`.
        //if it's a long way away, then it's 0.
        //if it's one away from y, then it's clear_range * 2 + 1 - 2.
        //the shadow we cast on y=10 is max(0, clear_range * 2 + 1 - distance to y * 2)
        //centered on our x.
        let x_delta = (l.location.x - target_x).abs();
        let x_shadow_size = (clear_range * 2 + 1 - 2 * x_delta).max(0);
        if x_shadow_size > 0 {
            let x_shadow_start = l.location.y - clear_range + x_delta;
            let shadow = Span::new_from_range(x_shadow_start..x_shadow_start + x_shadow_size);
            shadows.push(Reverse(shadow));
        }
    }
    let mut cur_span = shadows.pop().unwrap().0;
    if target_x == 3 {
    }

    while let Some(Reverse(s)) = shadows.pop() {
        match cur_span.collide_with(&s) {
            utils::span::CollisionType::OverlapsEnd(_, _, _) => unreachable!(),
            utils::span::CollisionType::Before(union) => {
                if union.size() == (cur_span.size() + s.size()) {
                    cur_span = union
                } else {
                    return true;
                }
            }
            utils::span::CollisionType::StrictlyBigger(_, _, _) => (),
            utils::span::CollisionType::Equal => (),
            utils::span::CollisionType::OverlapsStart(_, _, _) => {
                cur_span = s.union(&cur_span);
            }
            utils::span::CollisionType::StrictlySmaller(_, _, _) => {
                cur_span = s.union(&cur_span);
            }
            utils::span::CollisionType::After(_) => unreachable!(),
        }
    }
    cur_span.start > 0 || cur_span.end <= full_size
}
fn has_gap_y(sensors: &[Sensor], target_y: i64, full_size: i64) -> bool {
    let mut shadows = BinaryHeap::new();
    for l in sensors {
        let clear_range = (l.closest_beacon - l.location).manhattan();
        //Sensor at x=9, y=16: closest beacon is at x=10, y=16

        //how much of y=y does this one span?
        //well, if it's /on/ y, then it's `clear_range * 2 + 1`.
        //if it's a long way away, then it's 0.
        //if it's one away from y, then it's clear_range * 2 + 1 - 2.
        //the shadow we cast on y=10 is max(0, clear_range * 2 + 1 - distance to y * 2)
        //centered on our x.
        let y_delta = (l.location.y - target_y).abs();
        let y_shadow_size = (clear_range * 2 + 1 - 2 * y_delta).max(0);
        if y_shadow_size > 0 {
            let y_shadow_start = l.location.x - clear_range + y_delta;
            let shadow = Span::new_from_range(y_shadow_start..y_shadow_start + y_shadow_size);
            shadows.push(Reverse(shadow));
        }
    }
    let mut cur_span = shadows.pop().unwrap().0;

    while let Some(Reverse(s)) = shadows.pop() {
        match cur_span.collide_with(&s) {
            utils::span::CollisionType::OverlapsEnd(_, _, _) => unreachable!(),
            utils::span::CollisionType::Before(_) => {
                return true;
            }
            utils::span::CollisionType::StrictlyBigger(_, _, _) => (),
            utils::span::CollisionType::Equal => (),
            utils::span::CollisionType::OverlapsStart(_, _, _) => {
                cur_span = s.union(&cur_span);
            }
            utils::span::CollisionType::StrictlySmaller(_, _, _) => {
                cur_span = s.union(&cur_span);
            }
            utils::span::CollisionType::After(_) => unreachable!(),
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
    let found_y = (0..=max_coord).find(|&y| has_gap_y(&sensors, y, max_coord));
    let found_x = (0..=max_coord).find(|&x| has_gap_x(&sensors, x, max_coord));
    4000000 * found_x.unwrap() + found_y.unwrap()
}
