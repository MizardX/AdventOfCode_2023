use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;

/// Grid position
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

#[allow(dead_code)]
impl<T> Grid<T>
where
    T: Copy,
{
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

    pub fn get(&self, pos: Pos) -> Option<T> {
        Some(self.values[self.to_index(pos)?])
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        if let Some(ix) = self.to_index(pos) {
            self.values[ix] = value;
        }
    }

    pub fn map(&mut self, pos: Pos, f: impl FnOnce(T) -> T) {
        if let Some(ix) = self.to_index(pos) {
            let x = &mut self.values[ix];
            *x = f(*x);
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

    #[inline]
    fn is_inside(&self, pos: Pos) -> bool {
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

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stride = self.width;
        writeln!(f, "[")?;
        for r in 0..self.height {
            writeln!(f, "  {:?},", &self.values[r * stride..(r + 1) * stride])?;
        }
        write!(f, "]")
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
    <T as TryFrom<u8>>::Error: CommonErrors,
{
    type Err = <T as TryFrom<u8>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().enumerate();
        let first = lines.next().ok_or(Self::Err::empty_input())?.1;
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

pub trait CommonErrors {
    fn empty_input() -> Self;
}
