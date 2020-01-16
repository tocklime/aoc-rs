use num::{Num, abs, Signed, Unsigned};
use std::convert::{TryInto, TryFrom, Into};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Mul, Add};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
    pub fn turn_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
    pub fn as_point_step<T: From<i8> + Num>(self) -> Point<T> {
        match self {
            Dir::Up => Point::new(T::zero(), T::one()),
            Dir::Down => Point::new(T::zero(), (-1).into()),
            Dir::Left => Point::new((-1).into(), T::zero()),
            Dir::Right => Point::new(T::one(), T::zero()),
        }
    }
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
    pub fn step(self, d: Dir) -> Self {
        match d {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
        }
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
        where T: Copy
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

impl<T: Num + Signed> Point<T> {
    pub fn manhattan(self) -> T {
        abs(self.x) + abs(self.y)
    }
}

impl<T: Num + Unsigned> Point<T> {
    pub fn manhattan_unsigned(self) -> T {
        self.x + self.y
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

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Add + Num> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Mul + Copy + Num> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}