use std::{
    fmt::{Display, Write},
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use num::Integer;

use crate::nums::add_i_mod;

#[derive(Debug, Clone)]
pub struct Grid2d<T> {
    data: Vec<T>,
    size: Coord,
}

pub type Coord = (usize, usize);
pub type ICoord = (isize, isize);
impl<T: Copy> Grid2d<T> {
    pub fn from_elem(size: Coord, elem: T) -> Self {
        Self {
            data: vec![elem; size.0 * size.1],
            size,
        }
    }
    pub fn from_fn<F>(size: Coord, mut f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        let mut data = Vec::with_capacity(size.0 * size.1);
        for a in 0..size.0 {
            for b in 0..size.1 {
                data.push(f((a, b)));
            }
        }
        Self { data, size }
    }
}
impl<T> Index<Coord> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[index.0 * self.size.1 + index.1]
    }
}
impl<T> IndexMut<Coord> for Grid2d<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.0 * self.size.1 + index.1]
    }
}
impl<T: Display> Display for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((_, x), t) in self.indexed_iter() {
            f.write_fmt(format_args!("{}", t))?;
            if x == self.size.1 - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
impl<T: Copy> Grid2d<T> {
    pub fn grow_and_invalidate_all_data(&mut self, new_size: Coord, new_t: T) {
        self.size = new_size;
        let need_len = self.size.0 * self.size.1;
        self.data.extend((self.data.len()..need_len).map(|_| new_t));
    }
}
impl<T> Grid2d<T> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[must_use]
    pub fn get(&self, p: Coord) -> Option<&T> {
        if p.0 < self.size.0 && p.1 < self.size.1 {
            Some(&self[p])
        } else {
            None
        }
    }
    #[must_use]
    pub fn get_i(&self, p: ICoord) -> Option<&T> {
        let y: usize = p.0.try_into().ok()?;
        let x: usize = p.1.try_into().ok()?;
        self.get((y, x))
    }
    #[must_use]
    pub fn dim(&self) -> Coord {
        self.size
    }
    pub fn indexes(&'_ self) -> impl Iterator<Item = Coord> {
        let max = self.size;
        (0..max.0).cartesian_product(0..max.1)
    }
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.1), v))
    }
    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.1), v))
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    #[must_use]
    pub fn ordered_neighbours_with_self(&self, p: Coord) -> [Option<Coord>; 9] {
        let s = self.dim();
        let up = p.0.checked_sub(1);
        let left = p.1.checked_sub(1);
        let down = if p.0 + 1 == s.0 { None } else { Some(p.0 + 1) };
        let right = if p.1 + 1 == s.1 { None } else { Some(p.1 + 1) };
        [
            up.and_then(|y| left.map(|x| (y, x))),
            up.map(|y| (y, p.1)),
            up.and_then(|y| right.map(|x| (y, x))),
            left.map(|x| (p.0, x)),
            Some(p),
            right.map(|x| (p.0, x)),
            down.and_then(|y| left.map(|x| (y, x))),
            down.map(|y| (y, p.1)),
            down.and_then(|y| right.map(|x| (y, x))),
        ]
    }
    pub fn neighbours_with_diagonals(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
            (p.0.wrapping_sub(1), p.1.wrapping_sub(1)),
            (p.0 + 1, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1 + 1),
            (p.0.wrapping_sub(1), p.1 + 1),
        ]
        .into_iter()
        .filter(move |&x| x.0 < s.0 && x.1 < s.1)
    }
    pub fn neighbours(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
        ]
        .into_iter()
        .filter(move |&x| x.0 < s.0 && x.1 < s.1)
    }
    pub fn wraparound_relative_lookup(&self, p: Coord, relative: ICoord) -> &T {
        let d = self.dim();
        let targety = add_i_mod(p.0, &relative.0, d.0);
        let targetx = add_i_mod(p.1, &relative.1, d.1);
        &self[(targety, targetx)]
    }
    pub fn wraparound_neighbours(&self, (y, x): Coord) -> [Coord; 4] {
        let (sy, sx) = self.dim();
        [
            ((y + sy - 1) % sy, x),
            (y, (x + sx - 1) % sx),
            (y, (x + 1) % sx),
            ((y + 1) % sy, x),
        ]
    }
    pub fn to_string_with<F>(&self, disp: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut ans = String::with_capacity(self.data.len());
        for ((_, x), t) in self.indexed_iter() {
            ans.push_str(&disp(t));
            if x == self.size.1 - 1 {
                ans.push('\n');
            }
        }
        ans
    }
    pub fn from_str<F>(input: &str, conv: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.len());
        let mut rows = 0;
        for l in input.lines() {
            rows += 1;
            match (stride, l.len()) {
                (None, l) => stride = Some(l),
                (Some(a), b) if a != b => panic!("Not equal line lengths"),
                _ => {}
            }
            for c in l.chars() {
                data.push(conv(c));
            }
        }
        Self {
            data,
            size: (rows, stride.unwrap()),
        }
    }
}
