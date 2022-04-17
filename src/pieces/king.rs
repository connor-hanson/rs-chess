use crate::pieces::piece::{Piece, Color};
use crate::board::{Board, File};
use crate::utils::{mask_file};

pub struct King {}

impl Piece for King {
    fn all_moves(&self, current_pos: u64, color: &Color) -> u64 {
        let mut res = !current_pos;
        res &= self.all_moves_unbound( current_pos);

        let tz = current_pos.trailing_zeros();
        if tz % 8 == 7 {
            res ^= res & mask_file(File::A); // XOR
        } else if tz % 8 == 0 {
            res ^= res & mask_file(File::H);
        }
        return res;
    }
}

impl King {
    fn all_moves_unbound(&self, piece_pos: u64) -> u64 {
        let offsets: [u32; 4] = [1, 7, 8, 9];
        let mut moves: u64 = piece_pos;
        for offset in offsets {
            moves += piece_pos.checked_shr(offset).unwrap();
            moves += piece_pos.checked_shl(offset).unwrap();
        }
        return moves;
    }
}

#[cfg(test)]
mod test {

}



