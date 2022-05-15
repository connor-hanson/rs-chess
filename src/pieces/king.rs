use crate::pieces::piece::{Piece};
use crate::game::{Board, File, Color};
use crate::utils::{mask_file};

pub struct King {}

impl Piece for King {
    fn all_moves(&self, board: &Board, color: &Color) -> u64 {
        let current_pos: u64 = 0;
        // if *color == Color::BLACK {
        //     current_pos = board.b_pawns;
        // } else {
        //     current_pos = board.w_pawns;
        // }

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

    fn all_moves_pseudolegal_no_blocks(position: u64) -> u64 {
        return 1;
    }

    fn get_points(&self) -> i32 {
        return 3;
    }
}

impl King {
    pub fn all_moves_unbound(&self, piece_pos: u64) -> u64 {
        let offsets: [u32; 4] = [1, 7, 8, 9];
        let mut moves: u64 = piece_pos;
        for offset in offsets {
            moves += piece_pos.checked_shr(offset).unwrap();
            moves += piece_pos.checked_shl(offset).unwrap();
        }

        let tz = piece_pos.trailing_zeros();
        if tz % 8 == 7 {
            moves ^= moves & mask_file(File::A); // XOR
        } else if tz % 8 == 0 {
            moves ^= moves & mask_file(File::H);
        }

        moves ^= piece_pos;
        return moves;
    }
}

#[cfg(test)]
mod test {

}



