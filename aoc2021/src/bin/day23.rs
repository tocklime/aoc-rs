use std::str::FromStr;

use aoc_harness::*;

use pathfinding::prelude::{astar, dijkstra};
use smallvec::smallvec;
use smallvec::SmallVec;
use utils::cartesian::Point;

aoc_main!(2021 day 23, generator whole_input_is::<X>, 
        part1 [solve_astar::<false>,  solve_dijkstra::<false>] => 15358, 
        part2 [solve_astar::<true>,  solve_dijkstra::<true>]=>51436, 
        example part1 EG => 12521);

const EG: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct X {
    rooms: [SmallVec<[u8; 4]>; 4],
    hallway: [u8; 11],
    room_depth: usize,
}
const EMPTY: u8 = 99;
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
    #[inline]
    fn move_cost(c: u8) -> usize {
        [1, 10, 100, 1000][c as usize]
    }
    fn success(&self) -> bool {
        self.hallway.iter().all(|&x| x == EMPTY)
            && self.rooms.iter().enumerate().all(|(ix, r)| {
                r.len() == self.room_depth && r.iter().all(|c| usize::from(*c) == ix)
            })
    }
    fn heuristic(&self) -> usize {
        //if we could move everything home with no collisions, how much would it cost?
        let mut hole_counts = self
            .rooms
            .iter()
            .enumerate()
            .map(|(r_ix, x)| {
                self.room_depth - x.iter().take_while(|a| usize::from(**a) == r_ix).count()
            })
            .collect_vec();
        let hallway_moves = self
            .hallway
            .iter()
            .enumerate()
            .map(|(ix, &c)| match c {
                EMPTY => 0,
                c => {
                    let step_cost = Self::move_cost(c);
                    let hall_target = usize::from(2 * c + 2);
                    let s = abs_diff(hall_target, ix);
                    let cusize = usize::from(c);
                    let into_room = hole_counts[cusize];
                    hole_counts[cusize] -= 1;
                    (into_room + s) * step_cost
                }
            })
            .sum::<usize>();
        let room_moves = self
            .rooms
            .iter()
            .enumerate()
            .map(|(ix, v)| {
                v.iter()
                    .enumerate()
                    .map(|(ix2, &c)| {
                        let step_cost = Self::move_cost(c);
                        let cu = usize::from(c);
                        if cu == ix {
                            0
                        } else {
                            let out_of_this_room = ix2 + 1;
                            let across_hall = abs_diff(ix, cu) * 2;
                            let into_room = hole_counts[cu];
                            hole_counts[cu] -= 1;
                            (out_of_this_room + across_hall + into_room) * step_cost
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        hallway_moves + room_moves
    }
    fn do_move(&self, from: Location, to: Location) -> Self {
        let mut new = self.clone();
        let c = match from {
            Location::Room(r) => new.rooms[r].pop().unwrap(),
            Location::Hallway(h) => {
                let a = new.hallway[h];
                new.hallway[h] = EMPTY;
                a
            }
        };
        match to {
            Location::Room(r) => new.rooms[r].push(c),
            Location::Hallway(h) => new.hallway[h] = c,
        }
        new
    }
    fn path_len(&self, c: u8, from: Location, to: Location) -> Option<(Self, usize)> {
        let (hall_pos, leave_room) = match from {
            Location::Room(r) => (2 * r + 2, 1 + self.room_depth - self.rooms[r].len()),
            Location::Hallway(a) => (a, 0),
        };
        let hallway_target = match to {
            Location::Room(r) => 2 * r + 2,
            Location::Hallway(n) => n,
        };
        let hall_move = if hallway_target < hall_pos {
            //going left.
            let blocked = self.hallway[hallway_target..hall_pos]
                .iter()
                .any(|&x| x != EMPTY);
            if blocked {
                return None;
            }
            hall_pos - hallway_target
        } else {
            //going right.
            let blocked = self.hallway[hall_pos + 1..=hallway_target]
                .iter()
                .any(|&x| x != EMPTY);
            if blocked {
                return None;
            }
            hallway_target - hall_pos
        };
        let into_room = match to {
            Location::Room(r) => {
                if self.rooms[r].iter().any(|&x| usize::from(x) != r) {
                    return None;
                }
                self.room_depth.checked_sub(self.rooms[r].len())?
            }
            Location::Hallway(_) => 0,
        };
        let step_cost = Self::move_cost(c);
        Some((
            self.do_move(from, to),
            step_cost * (leave_room + hall_move + into_room),
        ))
    }
    fn successors(&self) -> Vec<(Self, usize)> {
        //try moving hall to room.
        for (ix, &c) in self.hallway.iter().enumerate() {
            if c != EMPTY {
                //c is at hallway pos ix,
                //i am in hallway, am only allowed to go to target room, and only if the path there is free
                //and only if all things in that room are in target room.
                //if i can do that, it is undoubtedly the best thing to do.
                let cu = usize::from(c);
                if let Some(r) = self.path_len(c, Location::Hallway(ix), Location::Room(cu)) {
                    return vec![r];
                }
            }
        }
        //try moving direct from room to room.
        for (ix, x) in self.rooms.iter().enumerate() {
            if let Some(&top) = x.last() {
                let topu = usize::from(top);
                if topu != ix {
                    if let Some(p) = self.path_len(top, Location::Room(ix), Location::Room(topu)) {
                        return vec![p];
                    }
                }
            }
        }
        //otherwise we need to unpack something from room to hallway.
        let mut ans = Vec::new();
        for (ix, v) in self.rooms.iter().enumerate() {
            const HALL_STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
            //can only move last in room.
            if let Some(&c) = v.last() {
                let costs = HALL_STOPS
                    .iter()
                    .filter_map(|s| self.path_len(c, Location::Room(ix), Location::Hallway(*s)));
                ans.extend(costs);
            }
        }
        ans
    }
    fn part2_mod(&mut self) {
        self.room_depth = 4;
        self.rooms[0].insert(1, 3);
        self.rooms[0].insert(1, 3);
        self.rooms[1].insert(1, 2);
        self.rooms[1].insert(1, 1);
        self.rooms[2].insert(1, 1);
        self.rooms[2].insert(1, 0);
        self.rooms[3].insert(1, 0);
        self.rooms[3].insert(1, 2);
    }
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = utils::cartesian::as_point_map::<usize>(s, false);
        let rooms = [
            smallvec![
                (map[&Point::new(3, 3)] as u32 - b'A' as u32) as u8,
                (map[&Point::new(3, 2)] as u32 - b'A' as u32) as u8,
            ],
            smallvec![
                (map[&Point::new(5, 3)] as u32 - b'A' as u32) as u8,
                (map[&Point::new(5, 2)] as u32 - b'A' as u32) as u8,
            ],
            smallvec![
                (map[&Point::new(7, 3)] as u32 - b'A' as u32) as u8,
                (map[&Point::new(7, 2)] as u32 - b'A' as u32) as u8,
            ],
            smallvec![
                (map[&Point::new(9, 3)] as u32 - b'A' as u32) as u8,
                (map[&Point::new(9, 2)] as u32 - b'A' as u32) as u8,
            ],
        ];
        let hallway = [EMPTY; 11];
        Ok(Self {
            hallway,
            rooms,
            room_depth: 2,
        })
    }
}
fn solve_astar<const PART2: bool>(input: &X) -> usize {
    let mut start = input.clone();
    if PART2 {
        start.part2_mod();
    }
    astar(&start, X::successors, X::heuristic, X::success)
        .unwrap()
        .1
}

fn solve_dijkstra<const PART2: bool>(input: &X) -> usize {
    let mut start = input.clone();
    if PART2 {
        start.part2_mod();
    }
    dijkstra(&start, X::successors, X::success).unwrap().1
}
