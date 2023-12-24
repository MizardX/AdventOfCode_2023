#![warn(clippy::pedantic)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::{Add, Index, IndexMut, Mul};
use std::str::FromStr;

use num_traits::{Num, PrimInt};
use thiserror::Error;

/// Grid position
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub const fn row(&self) -> isize {
        self.row
    }

    pub const fn col(&self) -> isize {
        self.col
    }

    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn distance(self, other: Self) -> usize {
        let dr = self.row().abs_diff(other.row());
        let dc = self.col().abs_diff(other.col());
        int_sqrt(dr * dr + dc * dc)
    }
}

fn int_sqrt<N: PrimInt>(x: N) -> N {
    if x.is_zero() {
        return x;
    }

    let mut s = N::one();
    let mut t = x;

    while s < t {
        s = s << 1;
        t = t >> 1;
    }

    t = s;
    s = (x / s + s) >> 1;

    while s < t {
        t = s;
        s = (x / s + s) >> 1;
    }

    t
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.row).field(&self.col).finish()
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::N => self.row -= 1,
            Dir::E => self.col += 1,
            Dir::S => self.row += 1,
            Dir::W => self.col -= 1,
        }
        self
    }
}

impl Add<MultiDir> for Pos {
    type Output = Self;

    #[allow(clippy::cast_possible_wrap)]
    fn add(mut self, rhs: MultiDir) -> Self::Output {
        match rhs {
            MultiDir { dir: Dir::N, count } => self.row -= count as isize,
            MultiDir { dir: Dir::E, count } => self.col += count as isize,
            MultiDir { dir: Dir::S, count } => self.row += count as isize,
            MultiDir { dir: Dir::W, count } => self.col -= count as isize,
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    /// North
    N,
    /// East
    E,
    /// South
    S,
    /// West
    W,
}

impl Dir {
    pub fn reverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }

    pub fn turn_cw(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn turn_ccw(self) -> Self {
        self.turn_cw().reverse()
    }
}

impl Mul<usize> for Dir {
    type Output = MultiDir;

    fn mul(self, count: usize) -> Self::Output {
        MultiDir { dir: self, count }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiDir {
    dir: Dir,
    count: usize,
}

impl MultiDir {
    pub fn dir(&self) -> Dir {
        self.dir
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_vec(width, height, vec![Default::default(); width * height])
    }
}

impl<T> Grid<T> {
    pub fn from_vec(width: usize, height: usize, values: Vec<T>) -> Self {
        assert_eq!(values.len(), width * height);
        Self {
            width,
            height,
            values,
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        let ix = self.to_index(pos)?;
        Some(&mut self.values[ix])
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        if let Some(ix) = self.to_index(pos) {
            self.values[ix] = value;
        }
    }

    pub fn get_row(&self, row: isize) -> Option<&[T]> {
        let row_usize = usize::try_from(row).ok()?;
        if (0..self.height).contains(&row_usize) {
            Some(&self.values[row_usize * self.width..(row_usize + 1) * self.width])
        } else {
            None
        }
    }

    pub fn get_row_mut(&mut self, row: isize) -> Option<&mut [T]> {
        let row_usize = usize::try_from(row).ok()?;
        if (0..self.height).contains(&row_usize) {
            Some(&mut self.values[row_usize * self.width..(row_usize + 1) * self.width])
        } else {
            None
        }
    }

    #[inline]
    pub fn is_inside(&self, pos: Pos) -> bool {
        let Ok(height) = isize::try_from(self.height) else {
            return false;
        };
        let Ok(width) = isize::try_from(self.width) else {
            return false;
        };
        (0..height).contains(&pos.row) && (0..width).contains(&pos.col)
    }

    #[inline]
    fn to_index(&self, pos: Pos) -> Option<usize> {
        let Ok(width) = isize::try_from(self.width) else {
            return None;
        };
        if self.is_inside(pos) {
            usize::try_from(pos.row * width + pos.col).ok()
        } else {
            None
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn position(&self, mut check: impl FnMut(T) -> bool) -> Option<Pos> {
        for r in 0..self.height {
            for (c, &value) in self
                .get_row(isize::try_from(r).unwrap())
                .unwrap()
                .iter()
                .enumerate()
            {
                if check(value) {
                    return Some(Pos::new(
                        isize::try_from(r).unwrap(),
                        isize::try_from(c).unwrap(),
                    ));
                }
            }
        }
        None
    }

    pub fn count_if(&self, mut check: impl FnMut(T) -> bool) -> usize {
        let mut count = 0;
        for &value in &self.values {
            if check(value) {
                count += 1;
            }
        }
        count
    }

    pub fn reset(&mut self, initial_value: T) {
        for x in &mut self.values {
            *x = initial_value;
        }
    }

    pub fn get(&self, pos: Pos) -> Option<T> {
        Some(self.values[self.to_index(pos)?])
    }

    pub fn map(&mut self, pos: Pos, f: impl FnOnce(T) -> T) {
        if let Some(ix) = self.to_index(pos) {
            let x = &mut self.values[ix];
            *x = f(*x);
        }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        let ix = self.to_index(index).unwrap();
        &self.values[ix]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        let ix = self.to_index(index).unwrap();
        &mut self.values[ix]
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stride = self.width;
        writeln!(f, "[")?;
        for r in 0..self.height {
            write!(f, "  ")?;
            for cell in &self.values[r * stride..(r + 1) * stride] {
                write!(f, "{cell:?}")?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
    CommonParseError: Into<<T as TryFrom<u8>>::Error>,
    // reverse of <T as TryFrom<u8>>::Error: From<CommonParseError> -- But the type checker didn't like this
{
    type Err = <T as TryFrom<u8>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().enumerate();
        let first = lines.next().ok_or(CommonParseError::EmptyInput.into())?.1;
        let width = first.len();
        let mut height = 1;
        let mut values = Vec::with_capacity(width * width);
        for ch in first.bytes() {
            values.push(ch.try_into()?);
        }
        for (r, line) in lines {
            height = r + 1;
            for ch in line.bytes() {
                values.push(ch.try_into()?);
            }
        }
        Ok(Self {
            width,
            height,
            values,
        })
    }
}

pub struct RepeatingGrid<'a, T>(&'a Grid<T>);

impl<'a, T> RepeatingGrid<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self(grid)
    }
}

impl<'a, T> Index<Pos> for RepeatingGrid<'a, T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        let pos = Pos::new(
            index
                .row()
                .rem_euclid(isize::try_from(self.0.height).unwrap()),
            index
                .col()
                .rem_euclid(isize::try_from(self.0.width).unwrap()),
        );
        &self.0[pos]
    }
}

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Num,
{
    let zero = T::zero();
    while a != zero {
        let r = b % a;
        b = a;
        a = r;
    }
    b
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Num,
{
    a * b / gcd(a, b)
}

#[derive(Debug, Error)]
pub enum CommonParseError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Expected char: {0:?}")]
    ExpectedChar(char),
    #[error("Invalid integer: {0:?}")]
    InvalidInteger(#[from] ParseIntError),
}
