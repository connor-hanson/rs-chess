use crate::pieces::piece::{Piece};
use crate::game::{Board, File, Color};
use crate::utils::{mask_file, mask_rank};

pub struct Rook {}

impl Piece for Rook {
    // https://www.chessprogramming.org/Classical_Approach
    fn all_moves(&self, board: &Board, color: &Color) -> u64 {
        let mut res: u64;
        
        // get all moves - disregarding collisions
        res = Rook::all_moves_unbound(board, color);

        // figure out collisons


        return res;
    }

    fn all_moves_pseudolegal_no_blocks(position: u64) -> u64 {
        return 1;
    }

    fn get_points(&self) -> i32 {
        return 5;
    }
}

impl Rook {
    fn all_moves_unbound(board: &Board, color: &Color)  -> u64{
        // cast a ray for each direction the rook can see
        let rooks: u64;
        if *color == Color::BLACK {
            rooks = board.b_rooks;
        } else {
            rooks = board.w_rooks;
        }

        if rooks == 0 {
            return 0;
        }

        let mut res: u64 = 0;

        // get the first and last rooks to minimize rank computations
        // ceiling division for first only - want range 1 - 8
        let first_rank = (rooks.trailing_zeros() + 8) / 8;
        let last_rank =  8 - (rooks.leading_zeros() / 8);
        let mut rank_mask = mask_rank(first_rank);
        println!("{}{}", first_rank, last_rank);

        for i in first_rank .. last_rank + 1 {
            if rooks & rank_mask != 0 {
                res |= rank_mask
            };
            println!("{}", i);
            rank_mask = rank_mask << 8;
        }

        // now check files
        for f in File::all() {
            if rooks & f.mask() != 0 {
                res |= f.mask();
            }
        }

        return res;
    }

    // fn all_moves_bound_by_collision(board: &Board, color)

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::bdiff;
    use crate::utils::{tile_u64, tile_list_u64, mask_file, mask_rank};

    #[test]
    fn test_rook_empty_board() {
        let r = Rook{};

        let mut b: Board = Board::default();
        b.b_rooks = tile_u64("A1");
        b.w_rooks = tile_list_u64(vec!["H8"]);
        bdiff(r.all_moves(&b, &Color::BLACK), 
            File::mask(&File::A) | mask_rank(1));

        bdiff(r.all_moves(&b, &Color::WHITE), 
            File::mask(&File::H) | mask_rank(8))
    }
}