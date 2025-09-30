use crate::engine::{bitboard::BitBoard, pieces::PieceType};
use ::std::{fmt::Display, marker::PhantomData};

pub struct White;
pub struct Black;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Compass {
    NorthWest = 7,
    North = 8,
    NorthEast = 9,
    West = -1,
    #[default]
    None = 0,
    East = 1,
    SouthWest = -9,
    South = -8,
    SouthEast = -7,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

//impl Iterator for Square {
//    type Item = Self;
//    fn next(&mut self) -> Option<Self::Item> {
//        match *self {
//            Self::A1 => Some(Self::B1),
//            Self::B1 => Some(Self::C1),
//            Self::C1 => Some(Self::D1),
//            Self::D1 => Some(Self::E1),
//            Self::E1 => Some(Self::F1),
//            Self::F1 => Some(Self::G1),
//            Self::G1 => Some(Self::H1),
//            Self::H1 => Some(Self::A2),
//            Self::A2 => Some(Self::B2),
//            Self::B2 => Some(Self::C2),
//            Self::C2 => Some(Self::D2),
//            Self::D2 => Some(Self::E2),
//            Self::E2 => Some(Self::F2),
//            Self::F2 => Some(Self::G2),
//            Self::G2 => Some(Self::H2),
//            Self::H2 => Some(Self::A3),
//            Self::A3 => Some(Self::B3),
//            Self::B3 => Some(Self::C3),
//            Self::C3 => Some(Self::D3),
//            Self::D3 => Some(Self::E3),
//            Self::E3 => Some(Self::F3),
//            Self::F3 => Some(Self::G3),
//            Self::G3 => Some(Self::H3),
//            Self::H3 => Some(Self::A4),
//            Self::A4 => Some(Self::B4),
//            Self::B4 => Some(Self::C4),
//            Self::C4 => Some(Self::D4),
//            Self::D4 => Some(Self::E4),
//            Self::E4 => Some(Self::F4),
//            Self::F4 => Some(Self::G4),
//            Self::G4 => Some(Self::H4),
//            Self::H4 => Some(Self::A5),
//            Self::A5 => Some(Self::B5),
//            Self::B5 => Some(Self::C5),
//            Self::C5 => Some(Self::D5),
//            Self::D5 => Some(Self::E5),
//            Self::E5 => Some(Self::F5),
//            Self::F5 => Some(Self::G5),
//            Self::G5 => Some(Self::H5),
//            Self::H5 => Some(Self::A6),
//            Self::A6 => Some(Self::B6),
//            Self::B6 => Some(Self::C6),
//            Self::C6 => Some(Self::D6),
//            Self::D6 => Some(Self::E6),
//            Self::E6 => Some(Self::F6),
//            Self::F6 => Some(Self::G6),
//            Self::G6 => Some(Self::H6),
//            Self::H6 => Some(Self::A7),
//            Self::A7 => Some(Self::B7),
//            Self::B7 => Some(Self::C7),
//            Self::C7 => Some(Self::D7),
//            Self::D7 => Some(Self::E7),
//            Self::E7 => Some(Self::F7),
//            Self::F7 => Some(Self::G7),
//            Self::G7 => Some(Self::H7),
//            Self::H7 => Some(Self::A8),
//            Self::A8 => Some(Self::B8),
//            Self::B8 => Some(Self::C8),
//            Self::C8 => Some(Self::D8),
//            Self::D8 => Some(Self::E8),
//            Self::E8 => Some(Self::F8),
//            Self::F8 => Some(Self::G8),
//            Self::G8 => Some(Self::H8),
//            Self::H8 => None,
//        }
//    }
//}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A1 => "a1",
                Self::B1 => "b1",
                Self::C1 => "c1",
                Self::D1 => "d1",
                Self::E1 => "e1",
                Self::F1 => "f1",
                Self::G1 => "g1",
                Self::H1 => "h1",
                Self::A2 => "a2",
                Self::B2 => "b2",
                Self::C2 => "c2",
                Self::D2 => "d2",
                Self::E2 => "e2",
                Self::F2 => "f2",
                Self::G2 => "g2",
                Self::H2 => "h2",
                Self::A3 => "a3",
                Self::B3 => "b3",
                Self::C3 => "c3",
                Self::D3 => "d3",
                Self::E3 => "e3",
                Self::F3 => "f3",
                Self::G3 => "g3",
                Self::H3 => "h3",
                Self::A4 => "a4",
                Self::B4 => "b4",
                Self::C4 => "c4",
                Self::D4 => "d4",
                Self::E4 => "e4",
                Self::F4 => "f4",
                Self::G4 => "g4",
                Self::H4 => "h4",
                Self::A5 => "a5",
                Self::B5 => "b5",
                Self::C5 => "c5",
                Self::D5 => "d5",
                Self::E5 => "e5",
                Self::F5 => "f5",
                Self::G5 => "g5",
                Self::H5 => "h5",
                Self::A6 => "a6",
                Self::B6 => "b6",
                Self::C6 => "c6",
                Self::D6 => "d6",
                Self::E6 => "e6",
                Self::F6 => "f6",
                Self::G6 => "g6",
                Self::H6 => "h6",
                Self::A7 => "a7",
                Self::B7 => "b7",
                Self::C7 => "c7",
                Self::D7 => "d7",
                Self::E7 => "e7",
                Self::F7 => "f7",
                Self::G7 => "g7",
                Self::H7 => "h7",
                Self::A8 => "a8",
                Self::B8 => "b8",
                Self::C8 => "c8",
                Self::D8 => "d8",
                Self::E8 => "e8",
                Self::F8 => "f8",
                Self::G8 => "g8",
                Self::H8 => "h8",
            }
        )
    }
}

#[allow(dead_code)]
pub struct Board<T> {
    // Full board
    board: BitBoard,

    // Colors
    white_board: BitBoard,
    black_board: BitBoard,

    // White
    white_pawn: (PieceType, BitBoard),
    white_knight: (PieceType, BitBoard),
    white_bishop: (PieceType, BitBoard),
    white_rook: (PieceType, BitBoard),
    white_queen: (PieceType, BitBoard),
    white_king: (PieceType, BitBoard),

    // Black
    black_pawn: (PieceType, BitBoard),
    black_knight: (PieceType, BitBoard),
    black_bishop: (PieceType, BitBoard),
    black_rook: (PieceType, BitBoard),
    black_queen: (PieceType, BitBoard),
    black_king: (PieceType, BitBoard),

    // Moves
    turn: PhantomData<T>,
}

impl<T> Board<T> {
    pub fn is_empty(&self) -> bool {
        self.board.is_empty()
    }

    pub fn has_empty(&self, square: Square) -> bool {
        !self.board.check(square)
    }

    pub fn get_piece(&self, square: Square) -> PieceType {
        if self.white_board.check(square) {
            for wb in [
                &self.white_pawn,
                &self.white_knight,
                &self.white_bishop,
                &self.white_rook,
                &self.white_queen,
            ] {
                if wb.1.check(square) {
                    return wb.0;
                }
            }

            return PieceType::WhiteKing;
        }
        if self.black_board.check(square) {
            for bb in [
                &self.black_pawn,
                &self.black_knight,
                &self.black_bishop,
                &self.black_rook,
                &self.black_queen,
            ] {
                if bb.1.check(square) {
                    return bb.0;
                }
            }

            return PieceType::BlackKing;
        }
        PieceType::NoPiece
    }
}

impl Board<White> {
    pub fn moves(&self) -> BitBoard {
        todo!()
    }
    pub fn best_move(self) -> Board<Black> {
        todo!()
    }
}
impl Board<Black> {
    pub fn moves(&self) -> BitBoard {
        todo!()
    }
    pub fn best_move(self) -> Board<White> {
        todo!()
    }
}

impl Default for Board<White> {
    fn default() -> Self {
        Board {
            board: 0xFFFF00000000FFFF.into(),

            white_board: 0xFFFF.into(),
            black_board: 0xFFFF000000000000.into(),

            white_pawn: (PieceType::WhitePawn, 0xFF00.into()),
            white_knight: (PieceType::WhiteKnight, 0x42.into()),
            white_bishop: (PieceType::WhiteBishop, 0x24.into()),
            white_rook: (PieceType::WhiteRook, 0x81.into()),
            white_queen: (PieceType::WhiteQueen, 0x8.into()),
            white_king: (PieceType::WhiteKing, 0x10.into()),

            black_pawn: (PieceType::BlackPawn, 0x00FF000000000000.into()),
            black_knight: (PieceType::BlackKnight, 0x4200000000000000.into()),
            black_bishop: (PieceType::BlackBishop, 0x2400000000000000.into()),
            black_rook: (PieceType::BlackRook, 0x8100000000000000.into()),
            black_queen: (PieceType::BlackQueen, 0x800000000000000.into()),
            black_king: (PieceType::BlackKing, 0x01000000000000000.into()),

            turn: PhantomData,
        }
    }
}
impl<T> Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
        ┌──┬──┬──┬──┬──┬──┬──┬──┐
      8 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      7 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      6 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      5 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      4 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      3 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      2 │{} │{} │{} │{} │{} │{} │{} │{} │
        ├──┼──┼──┼──┼──┼──┼──┼──┤
      1 │{} │{} │{} │{} │{} │{} │{} │{} │
        └──┴──┴──┴──┴──┴──┴──┴──┘
         a  b  c  d  e  f  g  h
        ",
            // Row 8
            self.get_piece(Square::A8),
            self.get_piece(Square::B8),
            self.get_piece(Square::C8),
            self.get_piece(Square::D8),
            self.get_piece(Square::E8),
            self.get_piece(Square::F8),
            self.get_piece(Square::G8),
            self.get_piece(Square::H8),
            // Row 7
            self.get_piece(Square::A7),
            self.get_piece(Square::B7),
            self.get_piece(Square::C7),
            self.get_piece(Square::D7),
            self.get_piece(Square::E7),
            self.get_piece(Square::F7),
            self.get_piece(Square::G7),
            self.get_piece(Square::H7),
            // Row 6
            self.get_piece(Square::A6),
            self.get_piece(Square::B6),
            self.get_piece(Square::C6),
            self.get_piece(Square::D6),
            self.get_piece(Square::E6),
            self.get_piece(Square::F6),
            self.get_piece(Square::G6),
            self.get_piece(Square::H6),
            // Row 5
            self.get_piece(Square::A5),
            self.get_piece(Square::B5),
            self.get_piece(Square::C5),
            self.get_piece(Square::D5),
            self.get_piece(Square::E5),
            self.get_piece(Square::F5),
            self.get_piece(Square::G5),
            self.get_piece(Square::H5),
            // Row 4
            self.get_piece(Square::A4),
            self.get_piece(Square::B4),
            self.get_piece(Square::C4),
            self.get_piece(Square::D4),
            self.get_piece(Square::E4),
            self.get_piece(Square::F4),
            self.get_piece(Square::G4),
            self.get_piece(Square::H4),
            // Row 3
            self.get_piece(Square::A3),
            self.get_piece(Square::B3),
            self.get_piece(Square::C3),
            self.get_piece(Square::D3),
            self.get_piece(Square::E3),
            self.get_piece(Square::F3),
            self.get_piece(Square::G3),
            self.get_piece(Square::H3),
            // Row 2
            self.get_piece(Square::A2),
            self.get_piece(Square::B2),
            self.get_piece(Square::C2),
            self.get_piece(Square::D2),
            self.get_piece(Square::E2),
            self.get_piece(Square::F2),
            self.get_piece(Square::G2),
            self.get_piece(Square::H2),
            // Row 1
            self.get_piece(Square::A1),
            self.get_piece(Square::B1),
            self.get_piece(Square::C1),
            self.get_piece(Square::D1),
            self.get_piece(Square::E1),
            self.get_piece(Square::F1),
            self.get_piece(Square::G1),
            self.get_piece(Square::H1),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_piece() {
        let board = Board::default();
        assert_eq!(board.get_piece(Square::A8), PieceType::BlackRook);
        assert_eq!(board.get_piece(Square::B8), PieceType::BlackKnight);
        assert_eq!(board.get_piece(Square::C8), PieceType::BlackBishop);
        assert_eq!(board.get_piece(Square::D8), PieceType::BlackQueen);
        assert_eq!(board.get_piece(Square::E8), PieceType::BlackKing);
        assert_eq!(board.get_piece(Square::F8), PieceType::BlackBishop);
        assert_eq!(board.get_piece(Square::G8), PieceType::BlackKnight);
        assert_eq!(board.get_piece(Square::H8), PieceType::BlackRook);

        assert_eq!(board.get_piece(Square::A7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::B7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::C7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::D7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::E7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::F7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::G7), PieceType::BlackPawn);
        assert_eq!(board.get_piece(Square::H7), PieceType::BlackPawn);

        assert_eq!(board.get_piece(Square::A1), PieceType::WhiteRook);
        assert_eq!(board.get_piece(Square::B1), PieceType::WhiteKnight);
        assert_eq!(board.get_piece(Square::C1), PieceType::WhiteBishop);
        assert_eq!(board.get_piece(Square::D1), PieceType::WhiteQueen);
        assert_eq!(board.get_piece(Square::E1), PieceType::WhiteKing);
        assert_eq!(board.get_piece(Square::F1), PieceType::WhiteBishop);
        assert_eq!(board.get_piece(Square::G1), PieceType::WhiteKnight);
        assert_eq!(board.get_piece(Square::H1), PieceType::WhiteRook);

        assert_eq!(board.get_piece(Square::A2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::B2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::C2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::D2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::E2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::F2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::G2), PieceType::WhitePawn);
        assert_eq!(board.get_piece(Square::H2), PieceType::WhitePawn);
    }
}
