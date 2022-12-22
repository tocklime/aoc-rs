use std::collections::HashMap;

use aoc_harness::*;
use nom::{
    branch::alt,
    character::complete::u32,
    combinator::{map, value},
    multi::many1,
    IResult,
};
use utils::{
    aabb::Aabb,
    cartesian::{self, Dir, Point},
};

aoc_main!(2022 day 22, part1 [p1] => 58248, part2 [p2] => 179091); //, example both EG => (6032,5031));

const EG: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum I {
    Go(u32),
    TurnLeft,
    TurnRight,
}
fn instr(input: &str) -> IResult<&str, I> {
    alt((
        map(u32, I::Go),
        value(I::TurnLeft, nom::character::complete::char('L')),
        value(I::TurnRight, nom::character::complete::char('R')),
    ))(input)
}

fn p1(input: &str) -> u32 {
    let (board, instrs) = input.trim_end().split_once("\n\n").unwrap();
    let mut map: HashMap<Point<u32>, char> = cartesian::as_point_map(board, true);
    let spaces = map
        .iter()
        .filter_map(|(k, v)| (*v == ' ').then_some(*k))
        .collect::<Vec<_>>();
    for s in spaces {
        map.remove(&s);
    }
    let mut log = map.clone();
    let bb: Aabb<u32> = map.keys().collect();
    let (_, instrs) = many1(instr)(instrs).unwrap();
    // println!("{}", cartesian::render_char_map_w(&map, 1, " ", true));
    let first_x = board.chars().position(|c| c == '.').unwrap();
    let mut position = Point::new(first_x as u32, bb.top_right.y);
    let mut dir = Dir::Right;
    let mut timer = None;
    for i in instrs {
        match i {
            I::Go(count) => {
                for _ in 0..count {
                    timer = timer.map(|x| x - 1);
                    if timer == Some(0) {
                        println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));
                        panic!();
                    }
                    let next = match map.get(&position.step(dir)) {
                        Some('.') => position.step(dir),
                        Some('#') => position,
                        Some(' ') | None =>
                        //try wrap
                        {
                            match dir {
                                Dir::Right => {
                                    //need left most on same row.
                                    let new_p = (0..bb.width())
                                        .map(|x| Point::new(x as u32, position.y))
                                        .find(|p| map.contains_key(p))
                                        .unwrap();
                                    if map[&new_p] == '.' {
                                        new_p
                                    } else {
                                        position
                                    }
                                }
                                Dir::Left => {
                                    //need right most on same row.
                                    let new_p = (0..bb.width())
                                        .rev()
                                        .map(|x| Point::new(x as u32, position.y))
                                        .find(|p| map.contains_key(p))
                                        .unwrap();
                                    if map[&new_p] == '.' {
                                        new_p
                                    } else {
                                        position
                                    }
                                }
                                Dir::Down => {
                                    //need top most on same col.
                                    let new_p = (0..bb.height())
                                        .rev()
                                        .map(|y| Point::new(position.x, y as u32))
                                        .find(|p| map.contains_key(p))
                                        .unwrap();
                                    if map[&new_p] == '.' {
                                        new_p
                                    } else {
                                        position
                                    }
                                }
                                Dir::Up => {
                                    //need top most on same col.
                                    let new_p = (0..bb.height())
                                        .map(|y| Point::new(position.x, y as u32))
                                        .find(|p| map.contains_key(p))
                                        .unwrap();
                                    if map[&new_p] == '.' {
                                        new_p
                                    } else {
                                        position
                                    }
                                }
                            }
                        }
                        _ => panic!(),
                    };
                    if next == position {
                        break;
                    }
                    position = next;
                    log.insert(position, dir.to_udlr());
                }
            }
            I::TurnLeft => dir = dir.turn_left(),
            I::TurnRight => dir = dir.turn_right(),
        }
        log.insert(position, dir.to_udlr());
        // dbg!(count, c, position, dir);
        // println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));
    }
    // dbg!(position, dir);
    // println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));

    1000 * (bb.height() as u32 - position.y)
        + 4 * (position.x + 1)
        + match dir {
            Dir::Up => 3,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 0,
        }
}
//wrong: 142392, 58392

fn transfer_edge_prob(p: Point<u32>, dir: Dir) -> (Point<u32>, Dir, Option<usize>) {
    const S: u32 = 50;
    let a = match dir {
        Dir::Up => {
            let fl = p.x % S;
            let fl1 = fl + 1;
            match p.x / S {
                0 => (Point::new(S, 3 * S - fl1), Dir::Right, None), //right onto (1,2)
                1 => (Point::new(0, S - fl1), Dir::Right, None),     //Right onto (0,0)
                2 => (Point::new(fl, 0), Dir::Up, None),             //up onto (0,0)
                _ => panic!(),
            }
        }
        Dir::Down => {
            let fl = p.x % S;
            let fl1 = fl + 1;
            match p.x / S {
                0 => (Point::new(2 * S + fl, 4 * S - 1), Dir::Down, None), //Down onto (2,3)
                1 => (Point::new(S - 1, S - fl1), Dir::Left, None),        //Left onto(0,0)
                2 => (Point::new(2 * S - 1, 3 * S - fl1), Dir::Left, None), //Left onto (1,2)
                _ => panic!(),
            }
        }
        Dir::Left => {
            let fb = p.y % S;
            let fb1 = fb + 1;
            match p.y / S {
                0 => (Point::new(2 * S - fb1, 4 * S - 1), Dir::Down, None), //Down on (1,3)
                1 => (Point::new(S, 4 * S - fb1), Dir::Right, None),        //Right on (1,3)
                2 => (Point::new(S - fb1, 2 * S - 1), Dir::Down, None),     //down on (0,1)
                3 => (Point::new(0, 2 * S - fb1), Dir::Right, None),        //right on (0,1)
                _ => panic!(),
            }
        }
        Dir::Right => {
            let fb = p.y % S;
            let fb1 = fb + 1;
            match dbg!(p.y / S) {
                0 => (Point::new(2 * S - fb1, S), Dir::Up, None), //up on (1,1)
                1 => (Point::new(3 * S - 1, 4 * S - fb1), Dir::Left, None), //left on (2,3)
                2 => (Point::new(3 * S - fb1, 3 * S), Dir::Up, None), // up on (2,3)
                3 => (Point::new(2 * S - 1, 2 * S - fb1), Dir::Left, None), // left on (1,1)
                _ => panic!(),
            }
        }
    };
    println!(
        "transport {:?} {:?} -> {:?} ({},{}) -> ({},{})",
        p,
        dir,
        a,
        p.x / S,
        p.y / S,
        a.0.x / S,
        a.0.y / S
    );
    a
}
fn transfer_edge_eg(p: Point<u32>, dir: Dir) -> (Point<u32>, Dir, Option<usize>) {
    //            mnop
    //            ||||
    //           /1111-d
    //          //1111-c
    //    ponm ///1111-b
    //    ||||////1111-a
    //  l-222233334444\
    //  k-222233334444\\
    //  j-222233334444\\\
    //  i-222233334444\\\\
    //    ||||\\\\55556666-a
    //    hgfe \\\55556666-b
    //          \\55556666-c
    //    O      \55556666-d
    //            ||||||||
    //            efghijkl
    const S: u32 = 4;
    let a = match dir {
        Dir::Up =>
        //up from 1, 3, 2 or 6.
        {
            let fl = p.x % S;
            match p.x / S {
                0 => (Point::new(3 * S - (fl + 1), 2 * S - 1), Dir::Down, None), //down on 1.
                1 => (Point::new(2 * S, 3 * S - (fl + 1)), Dir::Right, None),    //right to 1.
                2 => (Point::new(S - (fl + 1), 2 * S - 1), Dir::Down, None),     //down to 2
                3 => (Point::new(2 * S - 1, S + fl), Dir::Left, None),           //left to 4
                _ => panic!(),
            }
        }
        Dir::Down =>
        //down from 2,3,5,6
        {
            let fl = p.x % S;
            match p.x / S {
                0 => (Point::new(4 * S - (fl + 1), 0), Dir::Up, None), //2D-5U
                1 => (Point::new(2 * S, fl), Dir::Right, None),        //3D-5R
                2 => (Point::new(S - (fl + 1), S), Dir::Up, None),     //Up on 2
                3 => (Point::new(0, S + fl), Dir::Right, None),        //R on 2.
                _ => panic!(),
            }
        }
        Dir::Left => {
            //left from 1,2,5
            let fb = p.y % S;
            match p.y / S {
                0 => (Point::new(S + fb, S), Dir::Up, None), //Up on 3.
                1 => (Point::new(3 * S + fb, 0), Dir::Up, None), //Up on 6.
                2 => (Point::new(2 * S - (fb + 1), 2 * S - 1), Dir::Down, None), //down on 3.
                _ => panic!(),
            }
        }
        Dir::Right => {
            let fb = p.y % S;
            match dbg!(p.y / S) {
                0 => (Point::new(3 * S - 1, 3 * S - (fb + 1)), Dir::Left, None), //Left on 1
                1 => (Point::new(3 * S + fb, S - 1), Dir::Down, None),           //Down on 6
                2 => (Point::new(4 * S - 1, S - (fb + 1)), Dir::Up, None),       //Left on 6
                _ => panic!(),
            }
        }
    };
    println!("transport {:?} {:?} -> {:?}", p, dir, a);
    a
}
fn p2(input: &str) -> u32 {
    let (board, instrs) = input.trim_end().split_once("\n\n").unwrap();
    let mut map: HashMap<Point<u32>, char> = cartesian::as_point_map(board, true);
    let spaces = map
        .iter()
        .filter_map(|(k, v)| (*v == ' ').then_some(*k))
        .collect::<Vec<_>>();
    for s in spaces {
        map.remove(&s);
    }

    println!("{}", cartesian::render_char_map_w(&map, 1, " ", true));
    let is_eg = input == EG;
    if !is_eg {
        let offset = 10;
        let S = 50;
        for x in 0..3 {
            for y in 0..4 {
                let up = Point::new(x * S + offset, S * (y + 1) - 1);
                if map.get(&up).is_none() {
                    continue;
                }
                dbg!(up.step(Dir::Up), map.get(&up), map.get(&up.step(Dir::Up)));
                if map.get(&up.step(Dir::Up)).is_none() {
                    let xfer = transfer_edge_prob(up, Dir::Up);
                    let back = transfer_edge_prob(xfer.0, xfer.1.turn_about());
                    assert_eq!(up, back.0);
                    assert_eq!(back.1, Dir::Down);
                }
            }
        }
    }

    let mut log = map.clone();
    let bb: Aabb<u32> = map.keys().collect();
    dbg!(bb);
    let (_, instrs) = many1(instr)(instrs).unwrap();
    // println!("{}", cartesian::render_char_map_w(&map, 1, " ", true));
    let first_x = board.chars().position(|c| c == '.').unwrap();
    let mut position = Point::new(first_x as u32, bb.top_right.y);
    let mut dir = Dir::Right;
    let mut timer = None;
    for i in instrs {
        timer = timer.map(|x| x - 1);
        if timer == Some(0) {
            log.insert(position, '@');
            println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));
            panic!();
        }
        match i {
            I::Go(count) => {
                for _ in 0..count {
                    match map.get(&position.step(dir)) {
                        Some('.') => position = position.step(dir),
                        Some('#') => break,
                        Some(' ') | None =>
                        //try wrap
                        {
                            let (new_pos, new_dir, new_timer) = if is_eg {
                                transfer_edge_eg(position, dir)
                            } else {
                                transfer_edge_prob(position, dir)
                            };
                            if timer.is_none() {
                                log = map.clone();
                                log.insert(position, '%');
                                timer = new_timer;
                            }
                            let t = map[&new_pos];
                            //check that there's space behind us!
                            let behind = new_pos.step(new_dir.turn_about());
                            let b = map.get(&behind);
                            assert!(b.is_none() || b == Some(&' '));
                            if t == '.' {
                                position = new_pos;
                                dir = new_dir;
                            } else if t == '#' {
                                break;
                            } else {
                                panic!();
                            }
                        }
                        _ => panic!(),
                    };
                    log.insert(position, dir.to_udlr());
                }
            }
            I::TurnLeft => dir = dir.turn_left(),
            I::TurnRight => dir = dir.turn_right(),
        }
        log.insert(position, dir.to_udlr());
        // dbg!(count, c, position, dir);
        // println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));
        // dbg!(position, dir);
    }
    log.insert(position, '@');
    // println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));
    // dbg!(position, dir);
    // println!("{}", cartesian::render_char_map_w(&log, 1, " ", true));

    1000 * dbg!((bb.height() as u32 - position.y))
        + 4 * dbg!((position.x + 1))
        + match dir {
            Dir::Up => 3,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 0,
        }
}
//wrong:   129162
//too low: 143304
