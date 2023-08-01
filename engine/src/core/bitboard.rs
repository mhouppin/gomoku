use std::fmt::Display;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

use super::types::{File, Rank, Square, ROW_SIZE};

#[derive(Clone, Copy, Debug)]
pub struct Bitboard([u64; 6]);

impl Bitboard {
    pub const LEFT_EDGE: Self = Self([
        0x0200004000080001,
        0x0004000080001000,
        0x4000080001000020,
        0x0080001000020000,
        0x0001000020000400,
        0x0000000000400008,
    ]);

    pub const RIGHT_EDGE: Self = Self([
        0x0100002000040000,
        0x0002000040000800,
        0x2000040000800010,
        0x0040000800010000,
        0x0000800010000200,
        0x0000010000200004,
    ]);

    pub const BOTTOM_EDGE: Self = Self([
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x000001FFFFC00000,
    ]);

    pub fn new() -> Self {
        Self([0; 6])
    }

    pub fn raw_buffer(&self) -> [u64; 6] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|x| x.count_ones()).sum::<u32>() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0 == [0u64; 6]
    }

    pub fn get_square(&self, sq: Square) -> bool {
        ((self.0[(sq.value() / 64) as usize] >> (sq.value() % 64)) & 1) != 0
    }

    pub fn set_square(&mut self, sq: Square) {
        self.0[(sq.value() / 64) as usize] |= 1u64 << (sq.value() % 64);
    }

    pub fn rst_square(&mut self, sq: Square) {
        self.0[(sq.value() / 64) as usize] &= !(1u64 << (sq.value() % 64));
    }

    pub fn swp_square(&mut self, sq: Square) {
        self.0[(sq.value() / 64) as usize] ^= 1u64 << (sq.value() % 64);
    }

    pub fn shift_up(&self) -> Self {
        *self >> ROW_SIZE as u32
    }

    pub fn shift_down(&self) -> Self {
        (*self & !Self::BOTTOM_EDGE) << ROW_SIZE as u32
    }

    pub fn shift_left(&self) -> Self {
        (*self & !Self::LEFT_EDGE) >> 1
    }

    pub fn shift_right(&self) -> Self {
        (*self & !Self::RIGHT_EDGE) << 1
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rank = Rank::FIRST;

        while rank <= Rank::LAST {
            let mut file = File::FIRST;

            while file <= File::LAST {
                write!(
                    f,
                    "{}",
                    if self.get_square(Square::from(file, rank)) {
                        'x'
                    } else {
                        '.'
                    }
                )?;

                file += 1;
            }

            rank += 1;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self;

        result &= rhs;
        result
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(l, r)| *l &= r);
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self;

        result |= rhs;
        result
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(l, r)| *l |= r);
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = self;

        result ^= rhs;
        result
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(l, r)| *l ^= r);
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result = self;

        result.0.iter_mut().for_each(|v| *v = !*v);
        result
    }
}

impl Shl<u32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        let mut result = self;

        result <<= rhs;
        result
    }
}

impl ShlAssign<u32> for Bitboard {
    fn shl_assign(&mut self, rhs: u32) {
        let mut carry = 0;

        for i in 0..6 {
            let next_carry = self.0[i] >> (64 - rhs);
            self.0[i] = (self.0[i] << rhs) | carry;
            carry = next_carry;
        }
    }
}

impl Shr<u32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut result = self;

        result >>= rhs;
        result
    }
}

impl ShrAssign<u32> for Bitboard {
    fn shr_assign(&mut self, rhs: u32) {
        let mut carry = 0;

        for i in 0..6 {
            let next_carry = self.0[5 - i] << (64 - rhs);
            self.0[5 - i] = (self.0[5 - i] >> rhs) | carry;
            carry = next_carry;
        }
    }
}

pub struct BitboardIter {
    base_bb: [u64; 6],
    cur_slot: usize,
}

impl BitboardIter {
    pub fn from_bitboard(bitboard: &Bitboard) -> Self {
        Self {
            base_bb: bitboard.raw_buffer(),
            cur_slot: 0,
        }
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.cur_slot..6 {
            if self.base_bb[i] != 0 {
                self.cur_slot = i;

                let bitidx = self.base_bb[i].trailing_zeros();

                self.base_bb[i] &= self.base_bb[i] - 1;

                return Some(Square::new((i as u32 * 64 + bitidx) as u16));
            }
        }

        None
    }
}
