use std::{ops::{Index, IndexMut}, fmt::{Display, Write}};

use itertools::Itertools;
use num::Integer;

#[derive(Debug, Clone)]
pub struct Grid2d<T> {
    data: Vec<T>,
    size: Coord,
}

pub type Coord = (usize, usize);
impl<T: Copy> Grid2d<T> {
    pub fn from_elem(size: Coord, elem: T) -> Self {
        Self {
            data: vec![elem; size.0 * size.1],
            size,
        }
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
impl<T : Display>  Display for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((_,x),t) in self.indexed_iter() {
            f.write_fmt(format_args!("{}",t))?;
            if x == self.size.1 - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
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
    pub fn to_string_with<F>(&self, disp: F) -> String
    where F : Fn(&T) -> String {
        let mut ans = String::with_capacity(self.data.len());
        for ((_,x),t) in self.indexed_iter() {
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
