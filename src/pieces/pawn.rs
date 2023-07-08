use std::io::{Error};

use crate::pieces::piece::{
    Piece, 
};
use crate::game::{Board, File, Color};
use crate::utils::{mask_rank, mask_file, print_moves};

pub struct Pawn {}

impl Piece for Pawn {
    fn all_moves(&self, board: &Board, color: &Color) -> u64 {
        return 1;
    }

    fn all_moves_pseudolegal_no_blocks(position: u64) -> u64 {
        return 1;
    }

    fn get_points(&self) -> i32 {
        return 1;
    }
}

impl Pawn {

    // todo - return results
    fn _validate_pawn_positions(&self, pawns: u64, color: &Color) -> u64 {
        let _pawns_on_first: Result<u64, Error> = match *color == Color::WHITE && (pawns & mask_rank(1) != 0) {
            true => panic!("Illegal position: white pawns on first rank"),
            false => Ok(pawns)
        };

        let _pawns_on_eighth: Result<u64, Error> = match *color == Color::BLACK && (pawns & mask_rank(8) != 0) {
            true => panic!("Illegal position: black pawns on eighth rank"),
            false => Ok(pawns)
        };

        return pawns;
    }

    /// forward, attack, ...
    pub fn all_moves_unbound(&self, piece_pos: u64, color: &Color) -> u64 {
        // compute avail down moves
        // loop through BitBoard - skipping 1st and last rank since pawns cant exist there
        let mut res: u64 = 0;

        res = res & !mask_rank(1) & !mask_rank(8);


        res += self.one_step_forward(piece_pos, color);
        res += self.fast_moves(piece_pos, color);
        res += self.all_attacks(piece_pos, color);
        return res;
    }

    fn one_step_forward(&self, pawns: u64, color: &Color) -> u64 {
        if *color == Color::WHITE {
            return pawns << 8;
        }
        return pawns >> 8;
    }

    pub fn all_attacks(&self, pawns: u64, color: &Color) -> u64 {
        let mut res: u64 = 0;
        res += self.pawn_attack(pawns, color, true);
        res += self.pawn_attack(pawns, color, false);
        return res;
    }

    fn fast_moves(&self, piece_pos: u64, color: &Color) -> u64 {
        if *color == Color::WHITE {
            return (piece_pos & mask_rank(2)) << 16;
        }
        return (piece_pos & mask_rank(7)) >> 16;
    }

    fn pawn_attack(&self, masked_rank: u64, color: &Color, is_west_attack: bool) -> u64 {
        let valid_attack_squares = if is_west_attack {!mask_file(File::H)} else {!mask_file(File::A)};

        if *color == Color::WHITE {
            let shift = if is_west_attack { 7 } else { 9 };
            return (masked_rank << shift) & valid_attack_squares;
        }
        let shift = if is_west_attack { 9 } else { 7 };
        return (masked_rank >> shift) & valid_attack_squares;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::bdiff;
    use crate::utils::{tile_u64, tile_list_u64};
    use super::*;

    #[test]
    fn black_test_west_attacks() {
        let p = Pawn{};
        bdiff(p.pawn_attack(tile_u64("a4"), &Color::BLACK, true), 0);
        bdiff(p.pawn_attack(tile_u64("d4"), &Color::BLACK, true), tile_u64("c3"));
    }

    #[test]
    fn black_test_east_attacks() {
        let p = Pawn{};
        bdiff(p.pawn_attack(tile_u64("h2"), &Color::BLACK, false), 0);
        bdiff(p.pawn_attack(tile_u64("g4"), &Color::BLACK, false), tile_u64("h3"));
    }

    #[test]
    fn white_test_west_attacks() {
        let p = Pawn{};
        bdiff(p.pawn_attack(tile_u64("a4"), &Color::WHITE, true), 0);
        bdiff(p.pawn_attack(tile_u64("d4"), &Color::WHITE, true), tile_u64("c5"));
    }

    #[test]
    fn white_test_east_attacks() {
        let p = Pawn{};
        bdiff(p.pawn_attack(tile_u64("h2"), &Color::WHITE, false), 0);
        bdiff(p.pawn_attack(tile_u64("g4"), &Color::WHITE, false), tile_u64("h5"));
    }

    #[test]
    fn black_test_fast_moves() {
        let p = Pawn{};
        // black
        bdiff(p.fast_moves(tile_u64("a2"), &Color::BLACK), 0);
        bdiff(p.fast_moves(tile_u64("g6"), &Color::BLACK), 0);
        bdiff(p.fast_moves(tile_u64("g7"), &Color::BLACK), tile_u64("g5"));
    }

    #[test]
    fn white_test_fast_moves() {
        let p = Pawn{};

        bdiff(p.fast_moves(tile_u64("a2"), &Color::WHITE), tile_u64("a4"));
        bdiff(p.fast_moves(tile_u64("c3"), &Color::WHITE), 0);
        bdiff(p.fast_moves(tile_u64("g7"), &Color::WHITE), 0);
    }

    #[test]
    fn white_test_in_bounds() {
        let p = Pawn{};
        bdiff(p.all_moves_unbound(tile_u64("a2"), &Color::WHITE), 
            tile_list_u64(vec!["a3", "a4", "b3"]));

        bdiff(p.all_moves_unbound(tile_u64("d2"), &Color::WHITE), 
            tile_list_u64(vec!["c3", "d3", "e3", "d4"]));

        bdiff(p.all_moves_unbound(tile_u64("h3"), &Color::WHITE),
            tile_list_u64(vec!["g4", "h4"]));
    }

    #[test]
    fn black_test_in_bounds() {
        let p = Pawn{};
        bdiff(p.all_moves_unbound(tile_u64("h7"), &Color::BLACK),
            tile_list_u64(vec!["h6", "h5", "g6"]));
        
        bdiff(p.all_moves_unbound(tile_u64("c5"), &Color::BLACK),
            tile_list_u64(vec!["c4", "b4", "d4"]));

        bdiff(p.all_moves_unbound(tile_u64("a3"), &Color::BLACK),
            tile_list_u64(vec!["a2", "b2"]));
    }
}