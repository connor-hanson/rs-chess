mod pieces;
use pieces::{pawn, king, piece::{Piece, Color}};


mod board;
use board::{Board};

mod utils;
use utils::{tile_u64};


fn main() {
    let board = Board {
        w_king:  utils::tile_u64("e1"),
        bb_king: utils::tile_u64("e8"),
        b_pawns: 0x00ff000000000000,
        w_pawns: 0x000000000000ff00,
    };

    let king = king::King{};
    let pawn = pawn::Pawn{};
    // print_moves(pawn.attacks_west(tile_u64("a4"), Color::WHITE));
    // print_moves(king.all_moves(board.w_king, Color::WHITE));
    // print_moves(king.all_moves(board.bb_king, Color::BLACK));
}

#[cfg(test)]
pub mod test_util {
    // assert eq with print board
    pub fn board_string(b: u64) -> String {
        let mut formatted_str: String = "".to_string();
        let mut mask: u64 = 0xff00000000000000;
        for i in (0..8).rev() {
            formatted_str.push_str(&format!("{:08b}\n", (((b & mask) >> (8*i)).reverse_bits() >> 56) ));
            mask = mask >> 8;
        }

        return formatted_str;
    }

    pub fn bdiff(b: u64, expected: u64) {
        return assert_eq!(expected, b, "\nexpected: \n{}\nreceived:\n{}", board_string(expected), board_string(b));
    }
}
