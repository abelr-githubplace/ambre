use super::board::Square;
use ::std::{
    fmt::Debug,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
};
use std::fmt::Binary;

pub trait BoardFlip {
    /// Flip value top-to-bottom.
    fn vertical_flip(self) -> Self;
    /// Flip value left-to-right.
    fn horrizontal_flip(self) -> Self;
    /// Flip value top-left-to-bottom-right.
    fn diagonal_a1_h8_flip(self) -> Self;
    /// Flip value top-right-to-bottom-left.
    fn diagonal_h1_a8_flip(self) -> Self;
}
pub trait BoardRotate: BoardFlip {
    fn rotate_90(self) -> Self;
    fn rotate_180(self) -> Self;
    fn rotate_270(self) -> Self;
}

// Files
pub const A_FILE: BitBoard = BitBoard(0x0101010101010101);
pub const B_FILE: BitBoard = BitBoard(0x0202020202020202);
pub const C_FILE: BitBoard = BitBoard(0x0404040404040404);
pub const D_FILE: BitBoard = BitBoard(0x0808080808080808);
pub const E_FILE: BitBoard = BitBoard(0x1010101010101010);
pub const F_FILE: BitBoard = BitBoard(0x2020202020202020);
pub const G_FILE: BitBoard = BitBoard(0x4040404040404040);
pub const H_FILE: BitBoard = BitBoard(0x8080808080808080);
// Ranks
pub const RANK_1: BitBoard = BitBoard(0x00000000000000FF);
pub const RANK_2: BitBoard = BitBoard(0x000000000000FF00);
pub const RANK_3: BitBoard = BitBoard(0x0000000000FF0000);
pub const RANK_4: BitBoard = BitBoard(0x00000000FF000000);
pub const RANK_5: BitBoard = BitBoard(0x000000FF00000000);
pub const RANK_6: BitBoard = BitBoard(0x0000FF0000000000);
pub const RANK_7: BitBoard = BitBoard(0x00FF000000000000);
pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);

pub const BORDER: BitBoard = BitBoard(0xFF818181818181FF);
pub const A1_H8_DIAGONAL: BitBoard = BitBoard(0x8040201008040201);
pub const H1_A8_DIAGONAL: BitBoard = BitBoard(0x0102040810204080);
pub const LIGHT_SQUARES: BitBoard = BitBoard(0x55AA55AA55AA55AA);
pub const DARK_SQUARES: BitBoard = BitBoard(0xAA55AA55AA55AA55);
pub const EMPTY: BitBoard = BitBoard(0x0000000000000000);
pub const FULL: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);
pub const SQUARE_ARRAY: [u64; 64] = [
    0b1,
    0b10,
    0b100,
    0b1000,
    0b10000,
    0b100000,
    0b1000000,
    0b10000000,
    0b100000000,
    0b1000000000,
    0b10000000000,
    0b100000000000,
    0b1000000000000,
    0b10000000000000,
    0b100000000000000,
    0b1000000000000000,
    0b10000000000000000,
    0b100000000000000000,
    0b1000000000000000000,
    0b10000000000000000000,
    0b100000000000000000000,
    0b1000000000000000000000,
    0b10000000000000000000000,
    0b100000000000000000000000,
    0b1000000000000000000000000,
    0b10000000000000000000000000,
    0b100000000000000000000000000,
    0b1000000000000000000000000000,
    0b10000000000000000000000000000,
    0b100000000000000000000000000000,
    0b1000000000000000000000000000000,
    0b10000000000000000000000000000000,
    0b100000000000000000000000000000000,
    0b1000000000000000000000000000000000,
    0b10000000000000000000000000000000000,
    0b100000000000000000000000000000000000,
    0b1000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000000000,
    0b10000000000000000000000000000000000000000000000000000000000000,
    0b100000000000000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000000000000,
];

#[derive(Default, PartialEq, Clone)]
pub struct BitBoard(u64);

impl BitBoard {
    #[inline]
    pub fn set<T: Into<BitBoard>>(self, value: T) -> Self {
        self | value.into()
    }
    #[inline]
    pub fn unset<T: Into<BitBoard>>(self, value: T) -> Self {
        self & !value.into()
    }
    #[inline]
    pub fn check<T: Into<BitBoard>>(&self, value: T) -> bool {
        self.0 & value.into().0 != 0
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    #[inline]
    pub fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }
    #[inline]
    pub fn is_border(&self) -> bool {
        self.0 & BORDER.0 != 0
    }

    /// SWAR-popcount on https://www.chessprogramming.org/Population_Count#The_PopCount_routine
    #[inline]
    fn _swar_popcount(&self) -> u8 {
        const K1: u64 = 0x5555555555555555; /*  -1/3   */
        const K2: u64 = 0x3333333333333333; /*  -1/5   */
        const K4: u64 = 0x0f0f0f0f0f0f0f0f; /*  -1/17  */
        const KF: u64 = 0x0101010101010101; /*  -1/255 */

        let mut x = self.0 - ((self.0 >> 1) & K1); /* put count of each 2 bits into those 2 bits */
        x = (x & K2) + ((x >> 2) & K2); /* put count of each 4 bits into those 4 bits */
        x = (x + (x >> 4)) & K4; /* put count of each 8 bits into those 8 bits */
        x = (x * KF) >> 56; /* returns 8 most significant bits of x + (x<<8) + (x<<16) + (x<<24) + ...  */
        x as u8
    }
    #[inline]
    pub fn population(&self) -> u32 {
        self.0.count_ones()
    }
    #[inline]
    pub fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }
    #[inline]
    pub fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard(SQUARE_ARRAY[value as usize])
    }
}
impl From<&[Square]> for BitBoard {
    fn from(value: &[Square]) -> Self {
        BitBoard(
            value
                .iter()
                .map(|&sq| SQUARE_ARRAY[sq as usize])
                .fold(0, |mut b, bb| {
                    b |= bb;
                    b
                }),
        )
    }
}
impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard(value)
    }
}
impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l8 = (self.0 & 0xFF00000000000000) >> 56;
        let l7 = (self.0 & 0xFF000000000000) >> 48;
        let l6 = (self.0 & 0xFF0000000000) >> 40;
        let l5 = (self.0 & 0xFF00000000) >> 32;
        let l4 = (self.0 & 0xFF000000) >> 24;
        let l3 = (self.0 & 0xFF0000) >> 16;
        let l2 = (self.0 & 0xFF00) >> 8;
        let l1 = self.0 & 0xFF;

        write!(
            f,
            "
        ┌────────┐
        │{l8:08b}│
        │{l7:08b}│
        │{l6:08b}│
        │{l5:08b}│
        │{l4:08b}│
        │{l3:08b}│
        │{l2:08b}│
        │{l1:08b}│
        └────────┘
        "
        )
    }
}

impl Binary for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}
impl Shl for BitBoard {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0.shl(rhs.0))
    }
}
impl ShlAssign for BitBoard {
    fn shl_assign(&mut self, rhs: Self) {
        self.0.shl_assign(rhs.0);
    }
}
impl Shr for BitBoard {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0.shr(rhs.0))
    }
}
impl ShrAssign for BitBoard {
    fn shr_assign(&mut self, rhs: Self) {
        self.0.shr_assign(rhs.0);
    }
}
impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
    }
}
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0);
    }
}
impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
    }
}
impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitxor(rhs.0))
    }
}
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0);
    }
}
impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}
impl Sub for BitBoard {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0.not()))
    }
}
impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0.not())
    }
}
impl BoardFlip for BitBoard {
    fn vertical_flip(self) -> Self {
        Self(self.0.swap_bytes())
    }
    /// Horrizontal flip on https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#MirrorHorizontally
    fn horrizontal_flip(self) -> Self {
        const K1: u64 = 0x5555555555555555;
        const K2: u64 = 0x3333333333333333;
        const K4: u64 = 0x0f0f0f0f0f0f0f0f;
        let mut x = self.0 ^ K4 & (self.0 ^ self.0.rotate_left(8));
        x ^= K2 & (x ^ x.rotate_left(4));
        x ^= K1 & (x ^ x.rotate_left(2));
        Self(x.rotate_right(7))
    }
    fn diagonal_a1_h8_flip(self) -> Self {
        let mut x = self.0;
        const K1: u64 = 0x5500550055005500;
        const K2: u64 = 0x3333000033330000;
        const K4: u64 = 0x0f0f0f0f00000000;
        let mut t = K4 & (x ^ (x << 28));
        x ^= t ^ (t >> 28);
        t = K2 & (x ^ (x << 14));
        x ^= t ^ (t >> 14);
        t = K1 & (x ^ (x << 7));
        Self(x ^ t ^ (t >> 7))
    }
    fn diagonal_h1_a8_flip(self) -> Self {
        let mut x = self.0;
        const K1: u64 = 0xaa00aa00aa00aa00;
        const K2: u64 = 0xcccc0000cccc0000;
        const K4: u64 = 0xf0f0f0f00f0f0f0f;
        let mut t = x ^ (x << 36);
        x ^= K4 & (t ^ (x >> 36));
        t = K2 & (x ^ (x << 18));
        x ^= t ^ (t >> 18);
        t = K1 & (x ^ (x << 9));
        Self(x ^ t ^ (t >> 9))
    }
}
impl BoardRotate for BitBoard {
    fn rotate_180(self) -> Self {
        self.vertical_flip().horrizontal_flip()
    }
    fn rotate_90(self) -> Self {
        self.diagonal_a1_h8_flip().vertical_flip()
    }
    fn rotate_270(self) -> Self {
        self.vertical_flip().diagonal_a1_h8_flip()
    }
}
