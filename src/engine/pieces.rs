use super::bitboard::{self, BitBoard};
use ::std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    #[default]
    NoPiece,

    // White
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,

    // Black
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl PieceType {
    pub fn as_full_str(&self) -> &'static str {
        match *self {
            PieceType::NoPiece => "no piece",
            // White
            PieceType::WhitePawn => "white pawn",
            PieceType::WhiteKnight => "white knight",
            PieceType::WhiteBishop => "white bishop",
            PieceType::WhiteRook => "white rook",
            PieceType::WhiteQueen => "white queen",
            PieceType::WhiteKing => "white king",

            // Black
            PieceType::BlackPawn => "black pawn",
            PieceType::BlackKnight => "black knight",
            PieceType::BlackBishop => "black bishop",
            PieceType::BlackRook => "black rook",
            PieceType::BlackQueen => "black queen",
            PieceType::BlackKing => "black king",
        }
    }

    pub fn moves(&self, piece: BitBoard) -> BitBoard {
        match *self {
            Self::NoPiece => bitboard::EMPTY,

            // White
            Self::WhitePawn => piece,
            Self::WhiteKnight => piece,
            Self::WhiteBishop => piece,
            Self::WhiteRook => piece,
            Self::WhiteQueen => piece,
            Self::WhiteKing => piece,

            // Black
            Self::BlackPawn => piece,
            Self::BlackKnight => piece,
            Self::BlackBishop => piece,
            Self::BlackRook => piece,
            Self::BlackQueen => piece,
            Self::BlackKing => piece,
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoPiece => ' ',
                Self::WhitePawn => '♙',
                Self::WhiteKnight => '♘',
                Self::WhiteBishop => '♗',
                Self::WhiteRook => '♖',
                Self::WhiteQueen => '♕',
                Self::WhiteKing => '♔',
                Self::BlackPawn => '♟',
                Self::BlackKnight => '♞',
                Self::BlackBishop => '♝',
                Self::BlackRook => '♜',
                Self::BlackQueen => '♛',
                Self::BlackKing => '♚',
            }
        )
    }
}
