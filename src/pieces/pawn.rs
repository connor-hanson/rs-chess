use crate::pieces::piece::{Piece, mask_file};
use crate::board::{Board, File};

pub struct Pawn {}

impl Piece for Pawn {
    fn _all_moves_unbound(&self, piece_pos: u64, pos: Vec<u32>) -> u64 {
        return 1;
    }

    fn all_moves(&self, curr_pos: u64, color: str) -> u64 {
        return 1;
    }

    fn legal_moves(&self, board: Board) -> u64 {
        return 1;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pawn_basic_avail_move() {
        // check that can move two or one on starting rank
        assert_eq!(
            
        );

        // check that can move one past starting rank
        assert_eq!(

        );
    }
}