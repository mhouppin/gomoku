use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(i16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreKind {
    Centipoint(i16),
    MateIn(u8),
    MatedIn(u8),
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind().fmt(f)
    }
}

impl Display for ScoreKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ScoreKind::Centipoint(cp) => {
                let sign = match cp.cmp(&0) {
                    Ordering::Less => "-",
                    Ordering::Greater => "+",
                    Ordering::Equal => "",
                };
                write!(f, "{}{}.{:02}", sign, cp.abs() / 100, cp.abs() % 100)
            }
            ScoreKind::MateIn(m) => write!(f, "+M{}", (m + 1) / 2),
            ScoreKind::MatedIn(m) => write!(f, "-M{}", (m + 1) / 2),
        }
    }
}

impl Score {
    pub const ZERO: Self = Self(0);
    pub const DRAW: Self = Self::ZERO;
    pub const MAX: Self = Self(i16::MAX);
    pub const MIN: Self = Self(-Self::MAX.0);
    pub const UNIT: Self = Self(1);

    const MATE_IN_ZERO: Self = Self(i16::MAX - 100);
    const MAX_MATE_IN: Self = Self::mate_in(u8::MAX);
    const MIN_MATE_IN: Self = Self::mate_in(u8::MIN);
    const MAX_MATED_IN: Self = Self::mated_in(u8::MAX);
    const MIN_MATED_IN: Self = Self::mated_in(u8::MIN);

    pub const fn cp(centipoints: i16) -> Self {
        Self(centipoints)
    }

    pub const fn mate_in(plies_to_mate: u8) -> Self {
        Self(Self::MATE_IN_ZERO.0 - plies_to_mate as i16)
    }

    pub const fn mated_in(plies_to_mate: u8) -> Self {
        Self(-Self::mate_in(plies_to_mate).0)
    }

    pub const fn kind(self) -> ScoreKind {
        match self.0 {
            v if v >= Self::MAX_MATE_IN.0 => ScoreKind::MateIn((Self::MIN_MATE_IN.0 - v) as u8),
            v if v <= Self::MAX_MATED_IN.0 => ScoreKind::MatedIn((v - Self::MIN_MATED_IN.0) as u8),
            v => ScoreKind::Centipoint(v),
        }
    }

    pub fn saturating_add(self, other: Self) -> Self {
        Self(
            self.0
                .saturating_add(other.0)
                .clamp(Self::MAX_MATED_IN.0 + 1, Self::MAX_MATE_IN.0 - 1),
        )
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        Self(
            self.0
                .saturating_sub(other.0)
                .clamp(Self::MAX_MATED_IN.0 + 1, Self::MAX_MATE_IN.0 - 1),
        )
    }

    pub fn saturating_mul(self, other: i16) -> Self {
        Self(
            self.0
                .saturating_mul(other)
                .clamp(Self::MAX_MATED_IN.0 + 1, Self::MAX_MATE_IN.0 - 1),
        )
    }
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Score {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<i16> for Score {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i16> for Score {
    type Output = Self;

    fn div(self, rhs: i16) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign<i16> for Score {
    fn mul_assign(&mut self, rhs: i16) {
        self.0 *= rhs;
    }
}

impl DivAssign<i16> for Score {
    fn div_assign(&mut self, rhs: i16) {
        self.0 /= rhs;
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
