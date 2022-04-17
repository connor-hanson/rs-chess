use crate::pieces::piece::{
    Piece, 
    Color,
};
use crate::board::{Board, File};
use crate::utils::{mask_rank, mask_file};

pub struct Pawn {}

impl Piece for Pawn {
    /// unused for Pawns, since they can only move forward so need color param
    fn all_moves_unbound(&self, piece_pos: u64) -> u64 {
        panic!("pawns require a color to compute move");
    }

    fn all_moves(&self, curr_pos: u64, color: Color) -> u64 {
        return 1;
    }

    fn legal_moves(&self, board: Board) -> u64 {
        return 1;
    }
}

impl Pawn {

    // forward, attack, ...
    fn all_moves_unbound(&self, piece_pos: u64, color: &Color) -> u64 {
        // compute avail down moves
        // loop through BB - skipping 1st and last rank since pawns cant exist there
        let mut res: u64 = 0;

        let mut rank_mask = mask_rank(2);
        for _i in 2..8 {
            let tmp: u64 = rank_mask & piece_pos;

            if *color == Color::WHITE {
                res += tmp << 8;
            } else {
                res += tmp >> 8;
            }

            res += self.attacks_east(tmp, color);
            res += self.attacks_west(tmp, color);

            rank_mask = rank_mask << 8
        }

        // append any fast moves available
        res += self.fast_moves(piece_pos, color);
        return res;
    }

    fn fast_moves(&self, piece_pos: u64, color: &Color) -> u64 {
        if *color == Color::WHITE {
            return (piece_pos & mask_rank(2)) << 16;
        }
        return (piece_pos & mask_rank(7)) >> 16;
    }

    fn attacks_east(&self, masked_rank: u64, color: &Color) -> u64 {
        let res = !mask_file(File::A); // invert it
        if *color == Color::WHITE {
            return (masked_rank << 9) & res;
        } 
        return (masked_rank >> 7) & res;
    }

    fn attacks_west(&self, masked_rank: u64, color: &Color) -> u64 {
        let res = !mask_file(File::H);
        if *color == Color::WHITE {
            return (masked_rank << 7) & res;
        }
        return (masked_rank >> 9) & res;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::bdiff;
    use crate::utils::{tile_u64, tile_list_u64};
    use super::*;

    #[test]
    fn test_single_pawn_attacks_unbound() {
        let p = Pawn{};

        // try borders
        bdiff(p.attacks_west(tile_u64("a4"), &Color::WHITE), 0);
        bdiff(p.attacks_east(tile_u64("h2"), &Color::WHITE), 0);
        bdiff(p.attacks_west(tile_u64("a4"), &Color::BLACK), 0);
        bdiff(p.attacks_east(tile_u64("h2"), &Color::BLACK), 0);

        // test middle
        bdiff(p.attacks_west(tile_u64("d4"), &Color::WHITE), tile_u64("c5"));
        bdiff(p.attacks_east(tile_u64("g4"), &Color::WHITE), tile_u64("h5"));
        bdiff(p.attacks_west(tile_u64("d4"), &Color::BLACK), tile_u64("c3"));
        bdiff(p.attacks_east(tile_u64("g4"), &Color::BLACK), tile_u64("h3"));

        // // should show nothing if bad rank
        // _cmp( p._attacks_west(tile_u64("c8"), Color::WHITE), 0);
        // _cmp(p._attacks_east(tile_u64("h1"), Color:: WHITE), 0);
        // _cmp(p._attacks_west(tile_u64("f8"), Color::BLACK), 0);
        // _cmp(p._attacks_east(tile_u64("f1"), Color::BLACK), 0);
    }

    #[test]
    fn test_fast_moves() {
        let p = Pawn{};

        // white
        bdiff(p.fast_moves(tile_u64("a2"), &Color::WHITE), tile_u64("a4"));
        bdiff(p.fast_moves(tile_u64("c3"), &Color::WHITE), 0);
        bdiff(p.fast_moves(tile_u64("g7"), &Color::WHITE), 0);

        // black
        bdiff(p.fast_moves(tile_u64("a2"), &Color::BLACK), 0);
        bdiff(p.fast_moves(tile_u64("g6"), &Color::BLACK), 0);
        bdiff(p.fast_moves(tile_u64("g7"), &Color::BLACK), tile_u64("g5"));
    }

    #[test]
    fn test_all_moves_unbound(){
        let p = Pawn{};

        // White in bounds
        bdiff(p.all_moves_unbound(tile_u64("a2"), &Color::WHITE), 
            tile_list_u64(vec!["a3", "a4", "b3"]));

        bdiff(p.all_moves_unbound(tile_u64("d2"), &Color::WHITE), 
            tile_list_u64(vec!["c3", "d3", "e3", "d4"]));

        bdiff(p.all_moves_unbound(tile_u64("h3"), &Color::WHITE),
            tile_list_u64(vec!["g4", "h4"]));

        // Black in bounds
        bdiff(p.all_moves_unbound(tile_u64("h7"), &Color::BLACK),
            tile_list_u64(vec!["h6", "h5", "g6"]));
        
        bdiff(p.all_moves_unbound(tile_u64("c5"), &Color::BLACK),
            tile_list_u64(vec!["c4", "b4", "d4"]));

        bdiff(p.all_moves_unbound(tile_u64("a3"), &Color::BLACK),
            tile_list_u64(vec!["a2", "b2"]));

        // OOB
        bdiff(p.all_moves_unbound(tile_u64("h8"), &Color::BLACK), 0);
        bdiff(p.all_moves_unbound(tile_u64("d1"), &Color::BLACK), 0);
        bdiff(p.all_moves_unbound(tile_u64("h8"), &Color::WHITE), 0);
        bdiff(p.all_moves_unbound(tile_u64("d1"), &Color::WHITE), 0);
        
    }


}