use std::cmp::{max, min, Ordering};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Span<T> {
    pub start: T,
    pub end: T,
}
#[derive(PartialEq, Eq, Debug)]
pub enum CollisionType<T> {
    Equal,
    Before(Span<T>),
    OverlapsStart(Span<T>, Span<T>, Span<T>),
    StrictlyBigger(Span<T>, Span<T>, Span<T>),
    StrictlySmaller(Span<T>, Span<T>, Span<T>),
    OverlapsEnd(Span<T>, Span<T>, Span<T>),
    After(Span<T>),
}

impl<T: Eq + Ord + Copy> Span<T> {
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
    pub fn contains(&self, candidate: T) -> bool {
        candidate >= self.start && candidate < self.end
    }
    pub fn intersects(&self, other: &Self) -> bool {
        !(self.end <= other.start || other.end <= self.start)
    }
    pub fn collide_with(&self, other: &Self) -> CollisionType<T> {
        match (
            self.start.cmp(&other.start),
            self.end.cmp(&other.end),
            self.start.cmp(&other.end),
            self.end.cmp(&other.start),
        ) {
            (Ordering::Equal, Ordering::Equal, _, _) => CollisionType::Equal,
            (Ordering::Less, Ordering::Greater, _, _) => CollisionType::StrictlyBigger(
                Span::new(self.start, other.start),
                Span::new(other.start, other.end),
                Span::new(other.end, self.end),
            ),
            (_, _, Ordering::Greater | Ordering::Equal, _) => {
                CollisionType::After(self.union(other))
            }
            (_, _, _, Ordering::Less | Ordering::Equal) => CollisionType::Before(self.union(other)),
            //Conditions above are independent. Conditions below depend on something above not matching.
            (Ordering::Greater | Ordering::Equal, Ordering::Less | Ordering::Equal, _, _) => {
                CollisionType::StrictlySmaller(
                    Span::new(other.start, self.start),
                    Span::new(self.start, self.end),
                    Span::new(self.end, other.end),
                )
            }
            (Ordering::Less, _, _, Ordering::Greater) => CollisionType::OverlapsStart(
                Span::new(self.start, other.start),
                Span::new(other.start, self.end),
                Span::new(self.end, other.end),
            ),
            (_, _, _, Ordering::Greater) => CollisionType::OverlapsEnd(
                Span::new(other.start, self.start),
                Span::new(self.start, other.end),
                Span::new(other.end, self.end),
            ),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test_collisions() {
        let mut me = Span::new(10, 20);
        let other = Span::new(30, 40);
        assert_eq!(
            me.collide_with(&other),
            CollisionType::Before(Span::new(10, 40))
        );
        me.end = 30;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::Before(Span::new(10, 40))
        );
        me.end = 35;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::OverlapsStart(Span::new(10, 30), Span::new(30, 35), Span::new(35, 40))
        );
        me.end = 40;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::OverlapsStart(Span::new(10, 30), Span::new(30, 40), Span::new(40, 40))
        );
        me.end = 45;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::StrictlyBigger(Span::new(10, 30), Span::new(30, 40), Span::new(40, 45))
        );
        me.start = 30;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::OverlapsEnd(Span::new(30, 30), Span::new(30, 40), Span::new(40, 45))
        );
        me.start = 35;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::OverlapsEnd(Span::new(30, 35), Span::new(35, 40), Span::new(40, 45))
        );
        me.start = 40;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::After(Span::new(30, 45))
        );
        me.start = 30;
        me.end = 40;
        assert_eq!(me.collide_with(&other), CollisionType::Equal);
        me.end = 39;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::StrictlySmaller(Span::new(30, 30), Span::new(30, 39), Span::new(39, 40),)
        );
        me.start = 31;
        me.end = 40;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::StrictlySmaller(Span::new(30, 31), Span::new(31, 40), Span::new(40, 40),)
        );
        me.end = 39;
        assert_eq!(
            me.collide_with(&other),
            CollisionType::StrictlySmaller(Span::new(30, 31), Span::new(31, 39), Span::new(39, 40),)
        );
    }
}