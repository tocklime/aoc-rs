use num::Num;
use std::convert::{TryInto, TryFrom};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, BuildHasher};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
    pub fn up(self) -> Self {
        Self { x: self.x, y: self.y + T::one() }
    }
    pub fn down(self) -> Self {
        Self { x: self.x, y: self.y - T::one() }
    }
    pub fn left(self) -> Self {
        Self { x: self.x - T::one(), y: self.y }
    }
    pub fn right(self) -> Self {
        Self { x: self.x + T::one(), y: self.y }
    }
    pub fn follow_arrow(self, arrow: char) -> Self {
        match arrow {
            '^' => self.up(),
            '<' => self.left(),
            '>' => self.right(),
            'v' => self.down(),
            _ => panic!("Unknown char")
        }
    }
    pub fn neighbours_with_diagonals(&self) -> [Self; 8]
    where T : Copy
    {
        [
            self.up(),
            self.up().right(),
            self.right(),
            self.right().down(),
            self.down(),
            self.down().left(),
            self.left(),
            self.left().up()
        ]
    }
}

pub fn as_point_map<T>(input: &str) -> HashMap<Point<T>, char>
    where T: Num + TryFrom<usize> + Hash + Eq
    , <T as TryFrom<usize>>::Error: Debug
{
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x.try_into().unwrap(), y.try_into().unwrap()), c))
        })
        .collect()
}

