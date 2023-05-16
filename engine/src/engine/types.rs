pub struct File(u8);
pub struct Rank(u8);
pub struct Square(u16);

impl File {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Rank {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl Square {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn from(file: File, rank: Rank) -> Self {
        Self(file.0 as u16 + rank.0 as u16 * 19)
    }

    pub fn file(&self) -> File {
        File::new((self.0 % 19) as u8)
    }

    pub fn rank(&self) -> Rank {
        Rank::new((self.0 / 19) as u8)
    }
}

#[derive(Clone, Copy)]
pub enum Stone {
    Empty,
    Black,
    White,
}
