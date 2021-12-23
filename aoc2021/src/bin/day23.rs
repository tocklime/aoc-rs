use std::str::FromStr;

use aoc_harness::*;

use pathfinding::directed::dijkstra;
use utils::cartesian::Point;

aoc_main!(2021 day 23, generator whole_input_is::<X>, part1 [p1], part2 [p2], example part1 EG => 12521);

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
    fn moves(&self) -> Vec<(Location, Location, usize)> {
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
                        Location::Hallway(ix),
                        Location::Room(target_room),
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
                        Some(c) => Some((Location::Room(ix), Location::Hallway(*s), move_cost * c)),
                        None => None,
                    }
                });
                ans.extend(costs);
            }
        }
        ans
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
fn p1(input: &X) -> usize {
    let mut s = input.clone();
    s.room_depth = 2;
    let x = dijkstra::dijkstra(
        &s,
        |x| {
            let options = x.moves();
            let ans = options
                .into_iter()
                .map(|(f, t, c)| (x.do_move(f, t), c))
                .collect_vec();
            ans
        },
        |x| {
            x.rooms.iter().enumerate().all(|(ix, r)| {
                r.len() == x.room_depth && r.iter().all(|x| ix == X::target_room(*x))
            })
        },
    )
    .unwrap();
    x.1
}
fn p2(input: &X) -> usize {
    let mut s = input.clone();
    s.room_depth = 4;
    s.rooms[0].insert(1, 'D');
    s.rooms[0].insert(1, 'D');
    s.rooms[1].insert(1, 'C');
    s.rooms[1].insert(1, 'B');
    s.rooms[2].insert(1, 'B');
    s.rooms[2].insert(1, 'A');
    s.rooms[3].insert(1, 'A');
    s.rooms[3].insert(1, 'C');

    let x = dijkstra::dijkstra(
        &s,
        |x| {
            let options = x.moves();
            let ans = options
                .into_iter()
                .map(|(f, t, c)| (x.do_move(f, t), c))
                .collect_vec();
            ans
        },
        |x| {
            x.rooms.iter().enumerate().all(|(ix, r)| {
                r.len() == x.room_depth && r.iter().all(|x| ix == X::target_room(*x))
            })
        },
    )
    .unwrap();
    dbg!(&x);
    x.1
}

#[cfg(test)]
mod test {
    use aoc_harness::whole_input_is;

    #[test]
    fn test() {
        let input = "
#############
#...........#
###A#B#D#C###
  #A#B#C#D#
  #########";
        let g = whole_input_is(input.trim());
        let x = super::p1(&g);
        assert_eq!(x, 4600);
    }
}
//43450 toolow.
