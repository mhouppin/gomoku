use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const ROW_SIZE: u16 = 19;
pub const SQUARE_COUNT: u16 = ROW_SIZE * ROW_SIZE;
pub const BOARD_SIZE: usize = SQUARE_COUNT as usize;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct File(u8);
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Rank(u8);
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Square(u16);

impl File {
    pub const FIRST: Self = Self(0);
    pub const LAST: Self = Self(18);

    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Add<u8> for File {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u8> for File {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;
    }
}

impl Sub<u8> for File {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<File> for File {
    type Output = i8;

    fn sub(self, rhs: File) -> Self::Output {
        self.0 as i8 - rhs.0 as i8
    }
}

impl SubAssign<u8> for File {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;
    }
}

impl Rank {
    pub const FIRST: Self = Self(0);
    pub const LAST: Self = Self(18);

    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Add<u8> for Rank {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u8> for Rank {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;
    }
}

impl Sub<u8> for Rank {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<Rank> for Rank {
    type Output = i8;

    fn sub(self, rhs: Rank) -> Self::Output {
        self.0 as i8 - rhs.0 as i8
    }
}

impl SubAssign<u8> for Rank {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Square {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn from(file: File, rank: Rank) -> Self {
        Self(file.0 as u16 + rank.0 as u16 * ROW_SIZE)
    }

    pub fn file(&self) -> File {
        File::new((self.0 % ROW_SIZE) as u8)
    }

    pub fn rank(&self) -> Rank {
        Rank::new((self.0 / ROW_SIZE) as u8)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn shift(self, dir: Direction) -> Self {
        match dir {
            Direction::North => self - ROW_SIZE,
            Direction::South => self + ROW_SIZE,
            Direction::East => self + 1,
            Direction::West => self - 1,
            Direction::NorthEast => self - ROW_SIZE + 1,
            Direction::NorthWest => self - ROW_SIZE - 1,
            Direction::SouthEast => self + ROW_SIZE + 1,
            Direction::SouthWest => self + ROW_SIZE - 1,
        }
    }
}

impl Add<u16> for Square {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u16> for Square {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs;
    }
}

impl Sub<u16> for Square {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u16> for Square {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 -= rhs;
    }
}

#[derive(Clone, Copy)]
pub enum Stone {
    Empty,
    Black,
    White,
}

impl Stone {
    pub fn flip(&self) -> Stone {
        match *self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Empty => Self::Empty,
        }
    }
}
