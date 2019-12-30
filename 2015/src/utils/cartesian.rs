use num::Num;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    pub fn up(self) -> Self {
        Point { x: self.x, y: self.y + T::one() }
    }
    pub fn down(self) -> Self {
        Point { x: self.x, y: self.y - T::one() }
    }
    pub fn left(self) -> Self {
        Point { x: self.x - T::one(), y: self.y }
    }
    pub fn right(self) -> Self {
        Point { x: self.x + T::one(), y: self.y }
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
}