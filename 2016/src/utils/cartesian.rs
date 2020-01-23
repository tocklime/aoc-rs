use num::{Num, abs, Signed, Unsigned};
use std::convert::{TryInto, TryFrom, Into};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Mul, Add, Sub};

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
    pub fn from_x(udlr: &str, c: char) -> Self {
        let ix = udlr.find(c).expect("Unknown direction");
        [Self::Up, Self::Down, Self::Left, Self::Right][ix]
    }
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    pub fn as_point_step<T: From<i8> + Num>(self) -> Point<T> {
        match self {
            Self::Up => Point::new(T::zero(), T::one()),
            Self::Down => Point::new(T::zero(), (-1).into()),
            Self::Left => Point::new((-1).into(), T::zero()),
            Self::Right => Point::new(T::one(), T::zero()),
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
    pub fn follow_x(self, udlr: &str, c: char) -> Self {
        self.step(Dir::from_x(udlr,c))
    }

    pub fn follow_arrow(self, arrow: char) -> Self {
        self.follow_x("^v<>",arrow)
    }
    pub fn neighbours(&self) -> [Self; 4]
        where T: Copy
    {
        [
            self.up(),
            self.right(),
            self.down(),
            self.left(),
        ]
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
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<T: Sub + Num> Sub for Point<T> {
    type  Output = Self;
    fn sub(self, other: Self) -> Self {Self::new(self.x - other.x, self.y - other.y)}
}

impl<T: Mul + Copy + Num> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}