pub mod engine;

use engine::board;

use crate::engine::bitboard;

fn main() {
    let board = board::Board::default();
    println!("{board}");

    println!(
        "{}: {}",
        board::Square::G8,
        board.get_piece(board::Square::G8).as_full_str()
    );

    println!("A : {:?}", bitboard::A_FILE);
    println!("B : {:?}", bitboard::B_FILE);
    println!("C : {:?}", bitboard::C_FILE);
    println!("D : {:?}", bitboard::D_FILE);
    println!("E : {:?}", bitboard::E_FILE);
    println!("F : {:?}", bitboard::F_FILE);
    println!("G : {:?}", bitboard::G_FILE);
    println!("H : {:?}", bitboard::H_FILE);

    println!("1 : {:?}", bitboard::RANK_1);
    println!("2 : {:?}", bitboard::RANK_2);
    println!("3 : {:?}", bitboard::RANK_3);
    println!("4 : {:?}", bitboard::RANK_4);
    println!("5 : {:?}", bitboard::RANK_5);
    println!("6 : {:?}", bitboard::RANK_6);
    println!("7 : {:?}", bitboard::RANK_7);
    println!("8 : {:?}", bitboard::RANK_8);
}
