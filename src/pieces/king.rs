use crate::pieces::piece::{Piece, mask_file};
use crate::board::{Board, File};

pub struct King {}

impl Piece for King {
    fn _all_moves_unbound(&self, piece_pos: u64, pos: Vec<u32>) -> u64 {
        let mut moves: u64 = piece_pos;
        for offset in pos {
            moves += piece_pos.checked_shr(offset).unwrap();
            moves += piece_pos.checked_shl(offset).unwrap();
        }
        return moves;
    }

    fn all_moves(&self, current_pos: u64) -> u64 {
        let mut res = !current_pos;
        let k = current_pos;
        res &= self._all_moves_unbound(
            k, 
            vec![7, 8, 9, 1],
        );

        let tz = k.trailing_zeros();
        if tz % 8 == 7 {
            res ^= res & mask_file(File::A);
        } else if tz % 8 == 0 {
            res ^= res & mask_file(File::H);
        }

        // return self._mask_rank(3) | self._mask_file(BoardFile::H);

        return res;
    }

    fn legal_moves(&self, board: Board) -> u64 {
        return 1;
    }
}



