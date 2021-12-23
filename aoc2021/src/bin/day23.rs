use std::str::FromStr;

use aoc_harness::*;

use pathfinding::prelude::{astar, dijkstra};
use utils::cartesian::Point;

aoc_main!(2021 day 23, generator whole_input_is::<X>, part1 [solve_astar::<false>, solve_dijkstra::<false>] => 15358, part2 [solve_astar::<true>, solve_dijkstra::<true>]=>51436, example part1 EG => 12521);

const EG: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct X {
    rooms: [Vec<char>; 4],
    hallway: [Option<char>; 11],
    room_depth: usize,
}
#[derive(Debug)]
enum Location {
    Room(usize),
    Hallway(usize),
}
fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
impl X {
    fn move_cost(c: char) -> Option<usize> {
        match c {
            'A' => Some(1),
            'B' => Some(10),
            'C' => Some(100),
            'D' => Some(1000),
            _ => None,
        }
    }
    fn target_room(c: char) -> usize {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => unreachable!(),
        }
    }
    fn heuristic(&self) -> usize {
        //if we could move everything home with no collisions, how much would it cost?
        let mut ans = 0;
        ans += self
            .hallway
            .iter()
            .enumerate()
            .map(|(ix, c)| match c {
                Some(c) => {
                    let target = Self::target_room(*c);
                    let step_cost = Self::move_cost(*c).unwrap();
                    let hall_target = 2 * target + 2;
                    let s = abs_diff(hall_target, ix);
                    s * step_cost
                }
                None => 0,
            })
            .sum::<usize>();
        ans += self
            .rooms
            .iter()
            .enumerate()
            .map(|(ix, v)| {
                v.iter()
                    .enumerate()
                    .map(|(ix2, c)| {
                        let target_room = Self::target_room(*c);
                        let step_cost = Self::move_cost(*c).unwrap();
                        if target_room == ix {
                            0
                        } else {
                            let out_of_this_room = ix2 + 1;
                            let across_hall = abs_diff(ix, target_room) * 2;
                            (out_of_this_room + across_hall) * step_cost
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        ans
    }
    fn do_move(&self, from: Location, to: Location) -> Self {
        let mut new = self.clone();
        let c = match from {
            Location::Room(r) => new.rooms[r].pop().unwrap(),
            Location::Hallway(h) => new.hallway[h].take().unwrap(),
        };
        match to {
            Location::Room(r) => new.rooms[r].push(c),
            Location::Hallway(h) => new.hallway[h] = Some(c),
        }
        new
    }
    fn path_len(&self, from: Location, to: Location) -> Option<usize> {
        let (hall_pos, leave_room) = match from {
            Location::Room(a) => ([2, 4, 6, 8][a], 1 + self.room_depth - self.rooms[a].len()),
            Location::Hallway(a) => (a, 0),
        };
        let hallway_target = match to {
            Location::Room(r) => 2 * r + 2,
            Location::Hallway(n) => n,
        };
        let hall_move = if hallway_target < hall_pos {
            let blocked = (hallway_target..hall_pos).any(|x| self.hallway[x].is_some());
            if blocked {
                return None;
            }
            hall_pos - hallway_target
        } else {
            let blocked = (hall_pos + 1..=hallway_target).any(|x| self.hallway[x].is_some());
            if blocked {
                return None;
            }
            hallway_target - hall_pos
        };
        let into_room = match to {
            Location::Room(r) => {
                if self.rooms[r].iter().any(|x| Self::target_room(*x) != r) {
                    return None;
                }
                if self.room_depth == self.rooms[r].len() {
                    return None;
                } else {
                    self.room_depth - self.rooms[r].len()
                }
            }
            Location::Hallway(_) => 0,
        };
        Some(leave_room + hall_move + into_room)
    }
    fn moves(&self) -> Vec<(Self, usize)> {
        let mut ans = Vec::new();
        let from_hall = self.hallway.iter().enumerate().filter_map(|(ix, x)| {
            if let Some(c) = x {
                //c is at hallway pos ix,
                let target_room = Self::target_room(*c);
                let move_cost = Self::move_cost(*c).unwrap();
                //i am in hallway, am only allowed to go to target room, and only if the path there is free
                //and only if all things in that room are in target room.
                match self.path_len(Location::Hallway(ix), Location::Room(target_room)) {
                    Some(r) => Some((
                        self.do_move(Location::Hallway(ix), Location::Room(target_room)),
                        move_cost * r,
                    )),
                    None => None,
                }
            } else {
                None
            }
        });
        ans.extend(from_hall);
        for (ix, v) in self.rooms.iter().enumerate() {
            //wlog, lets force hallway stops.
            const HALL_STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
            //can only move last in room.
            if let Some(c) = v.last() {
                let move_cost = Self::move_cost(*c).unwrap();
                let costs = HALL_STOPS.iter().filter_map(|s| {
                    match self.path_len(Location::Room(ix), Location::Hallway(*s)) {
                        Some(c) => Some((
                            self.do_move(Location::Room(ix), Location::Hallway(*s)),
                            move_cost * c,
                        )),
                        None => None,
                    }
                });
                ans.extend(costs);
            }
        }
        ans
    }
    fn part2_mod(&mut self) {
        self.room_depth = 4;
        self.rooms[0].insert(1, 'D');
        self.rooms[0].insert(1, 'D');
        self.rooms[1].insert(1, 'C');
        self.rooms[1].insert(1, 'B');
        self.rooms[2].insert(1, 'B');
        self.rooms[2].insert(1, 'A');
        self.rooms[3].insert(1, 'A');
        self.rooms[3].insert(1, 'C');
    }
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = utils::cartesian::as_point_map::<usize>(s, false);
        let mut rooms = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for r in 0..4 {
            rooms[r].push(map[&Point::new(3 + 2 * r, 3)]);
            rooms[r].push(map[&Point::new(3 + 2 * r, 2)]);
        }
        let hallway = [None; 11];
        Ok(Self {
            hallway,
            rooms,
            room_depth: 2,
        })
    }
}
fn solve_astar<const PART2: bool>(input: &X) -> usize {
    let mut s = input.clone();
    if PART2 {
        s.part2_mod();
    }
    let x = astar(&s, |x| x.moves(), |z| z.heuristic(), |x| x.heuristic() == 0).unwrap();
    x.1
}

fn solve_dijkstra<const PART2: bool>(input: &X) -> usize {
    let mut s = input.clone();
    if PART2 {
        s.part2_mod();
    }
    let x = dijkstra(&s, |x| x.moves(), |x| x.heuristic() == 0).unwrap();
    x.1
}
