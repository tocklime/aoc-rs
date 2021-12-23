use std::cmp::max;
use std::cmp::min;
use std::str::FromStr;

use aoc_harness::*;

use pathfinding::prelude::dijkstra;
use smallvec::smallvec;
use smallvec::SmallVec;
use utils::cartesian::Point;

aoc_main!(2021 day 23, generator whole_input_is::<X>,
        part1 [solve::<false>] => 15358,
        part2 [solve::<true>]=>51436,
        example part1 EG => 12521);

const EG: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

#[derive(Clone, PartialEq, Eq, Hash)]
struct X {
    rooms: [SmallVec<[u8; 4]>; 4],
    hallway: [u8; 11],
    room_depth: usize,
}
fn u8_to_char(x: u8) -> char {
    match x {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        EMPTY => '.',
        _ => '?',
    }
}
impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "\n".to_owned();
        s += &self
            .hallway
            .iter()
            .map(|&x| u8_to_char(x))
            .collect::<String>();

        for i in (0..self.room_depth).rev() {
            s += "\n  ";
            for r in self.rooms.iter() {
                s += &format!("{} ", r.get(i).map(|&x| u8_to_char(x)).unwrap_or('.'));
            }
        }
        f.write_str(&s)
    }
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
    fn solved_counts(&self) -> Vec<usize> {
        self.rooms
            .iter()
            .enumerate()
            .map(|(r_ix, x)| x.iter().take_while(|a| usize::from(**a) == r_ix).count())
            .collect_vec()
    }
    fn heuristic(&self) -> usize {
        //if we could move everything home with no collisions, how much would it cost?
        let solved_counts = self.solved_counts();
        let mut hole_counts = solved_counts
            .iter()
            .map(|&x| self.room_depth - x)
            .collect_vec();
        let room_moves = self
            .rooms
            .iter()
            .enumerate()
            .map(|(room_ix, v)| {
                v.iter()
                    .enumerate()
                    .map(|(amph_ix, &c)| {
                        let step_cost = Self::move_cost(c);
                        let cu = usize::from(c);
                        if cu == room_ix && amph_ix < solved_counts[cu] {
                            0
                        } else {
                            let out_of_this_room = self.room_depth - amph_ix;
                            let across_hall = abs_diff(room_ix, cu) * 2;
                            let into_room = hole_counts[cu];
                            hole_counts[cu] -= 1;
                            (out_of_this_room + across_hall + into_room) * step_cost
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        room_moves
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
    fn hallway_dist(from: usize, to: usize) -> usize {
        max(from, to) - min(from, to)
    }
    fn path_len(&self, c: u8, from: Location, to: Location) -> Option<(Self, usize)> {
        let hallway_destination = usize::from(2 * c + 2);
        let (h_from, h_to, cost) = match (&from, &to) {
            (&Location::Room(r), &Location::Hallway(h)) => (
                2 * r + 2,
                h,
                Self::hallway_dist(2 * r + 2, h) + Self::hallway_dist(h, hallway_destination)
                    - Self::hallway_dist(2 * r + 2, hallway_destination),
            ),
            (&Location::Hallway(h), &Location::Room(r)) => (h, 2 * r + 2, 0),
            (Location::Room(_), Location::Room(_)) => unreachable!(),
            (Location::Hallway(_), Location::Hallway(_)) => unreachable!(),
        };
        let blocked = if h_to < h_from {
            //going left.
            self.hallway[h_to..h_from].iter().any(|&x| x != EMPTY)
        } else {
            //going right.
            self.hallway[h_from + 1..=h_to].iter().any(|&x| x != EMPTY)
        };
        if blocked {
            return None;
        }
        if let Location::Room(r) = to {
            //can only move into a room if all amphipods in there are
            //the right kind.
            if self.rooms[r].iter().any(|&x| usize::from(x) != r) {
                return None;
            }
            //checked sub here to check room isn't full.
            self.room_depth.checked_sub(self.rooms[r].len())?;
        };
        let step_cost = Self::move_cost(c);
        Some((self.do_move(from, to), step_cost * cost))
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
        //otherwise we need to unpack something from room to hallway.
        let mut ans = Vec::new();
        let solved_counts = self.solved_counts();
        for (room_ix, v) in self.rooms.iter().enumerate() {
            const HALL_STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
            if solved_counts[room_ix] == v.len() {
                continue;
            }
            //can only move last in room.
            if let Some(&c) = v.last() {
                let costs = HALL_STOPS.iter().filter_map(|s| {
                    self.path_len(c, Location::Room(room_ix), Location::Hallway(*s))
                });
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

fn solve<const PART2: bool>(input: &X) -> usize {
    let mut start = input.clone();
    if PART2 {
        start.part2_mod();
    }
    start.heuristic() + dijkstra(&start, X::successors, X::success).unwrap().1
}
