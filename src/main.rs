mod pieces;
use pieces::{king, piece::Piece};

mod board;
use board::{Board};

mod utils;

fn print_moves(b: u64) {
    let mut mask: u64 = 0xff00000000000000;
    for i in (0..8).rev() {
        println!("{:08b}", (((b & mask) >> (8 * i)).reverse_bits()) >> 56);
        mask = mask >> 8;
    }
    println!("");
}


fn main() {
    let board = Board {
        w_king:  0x0000000000000010,
        bb_king: 0x1000000000000000,
        b_pawns: 0x00ff000000000000,
        w_pawns: 0x000000000000ff00,
    };

    let king = king::King{};
    print_moves(king.all_moves(board.w_king));
    print_moves(king.all_moves(board.bb_king));
}
