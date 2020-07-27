use crate::utils::aabb::Aabb;
use num::{abs, Num, Signed, Unsigned};
use std::collections::HashMap;
use std::convert::{Into, TryFrom, TryInto};
use std::fmt::{Debug, Display};
use std::hash::{BuildHasher, Hash};
use std::ops::{Add, AddAssign, Mul, RangeInclusive, Sub};

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
        Self::try_from_x(udlr, c).expect("Unknown direction")
    }
    pub fn try_from_x(udlr: &str, c: char) -> Option<Self> {
        udlr.find(c)
            .map(|x| [Self::Up, Self::Down, Self::Left, Self::Right][x])
    }
    pub fn to_udlr(self) -> char {
        match self {
            Self::Up => 'U',
            Self::Down => 'D',
            Self::Left => 'L',
            Self::Right => 'R',
        }
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
    pub fn turn_about(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
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

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + T::one(),
        }
    }
    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - T::one(),
        }
    }
    pub fn left(self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y,
        }
    }
    pub fn right(self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y,
        }
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
        self.step(Dir::from_x(udlr, c))
    }

    pub fn follow_arrow(self, arrow: char) -> Self {
        self.follow_x("^v<>", arrow)
    }
    pub fn neighbours(&self) -> [Self; 4]
    where
        T: Copy,
    {
        [self.up(), self.down(), self.left(), self.right()]
    }
    pub fn neighbours_with_diagonals(&self) -> [Self; 8]
    where
        T: Copy,
    {
        [
            self.up(),
            self.up().right(),
            self.right(),
            self.right().down(),
            self.down(),
            self.down().left(),
            self.left(),
            self.left().up(),
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

pub fn as_point_map<T>(input: &str, increasing_y_is_up: bool) -> HashMap<Point<T>, char>
where
    T: Num + TryFrom<usize> + Hash + Eq,
    <T as TryFrom<usize>>::Error: Debug,
{
    let boxed: Box<dyn Iterator<Item = _>> = if increasing_y_is_up {
        Box::new(input.lines().rev())
    } else {
        Box::new(input.lines())
    };
    boxed
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
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Mul + Copy + Num> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

pub fn point_map_bounding_box<N, T, S>(hm: &HashMap<Point<N>, T, S>) -> Aabb<N>
where
    N: Copy + Num + TryInto<usize> + Ord,
    RangeInclusive<N>: Iterator<Item = N>,
    S: BuildHasher,
{
    Aabb::from_iter(&mut hm.keys().cloned())
}

pub fn render_char_map_w<N, S>(
    m: &HashMap<Point<N>, char, S>,
    width: u8,
    default: char,
    flip: bool,
) -> String
where
    S: BuildHasher,
    N: Copy + Num + TryInto<usize> + Ord + Eq + Hash,
    RangeInclusive<N>: Iterator<Item = N>,
{
    let bb = point_map_bounding_box(&m);
    let v = bb.vec_with(|p| *m.get(&p).unwrap_or(&default));
    let x = v.iter().map(|l| {
        "\n".to_string()
            + &l.iter()
                .flat_map(|&x| (0..width).map(move |_| x))
                .collect::<String>()
    });
    if flip {
        x.rev().collect()
    } else {
        x.collect()
    }
}
