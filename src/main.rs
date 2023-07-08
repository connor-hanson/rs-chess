use std::env;

mod pieces;
use pieces::{pawn, king, knight, piece::{Piece}};

mod constants;
use constants::attack_sets::AttackSets;

mod game;
use game::{Board, Color, Rank, File};

mod utils;
use utils::{tile_u64, print_moves};

mod magic;
use magic::magic::{
    find_magics,
    SlidingMagic,
};


fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    // let board = Board {
    //     w_king:  utils::tile_u64("e1"),
    //     bb_king: utils::tile_u64("e8"),
    //     b_pawns: 0x00ff000000000000,
    //     w_pawns: 0x000000000000ff00,
    // };

    // let king = king::King{};
    // let pawn = pawn::Pawn{};
    // print_moves(pawn.attacks_west(tile_u64("a4"), Color::WHITE));
    // print_moves(king.all_moves(board.w_king, Color::WHITE));
    // print_moves(king.all_moves(board.bb_king, Color::BLACK));

    let mut all_attacks: AttackSets = AttackSets::default();
    init_move_sets(&mut all_attacks);

    let magicmoves: SlidingMagic = find_magics();
    // print_moves(&all_attacks.king_attacks[7])

    // let precomputed = rook_all_occupancies(tile_u64("A3"), Rank::ONE.mask(), File::C.mask());
    // for i in precomputed {
    //     print_moves(&i);
    //     println!("\n")
    // }
}

fn init_move_sets(empty_attack_sets: &mut AttackSets) {
    let each_tile: [u64; 64] = Board::each_tile();

    // precompute the easy ones
    precompute_knight(each_tile, &mut empty_attack_sets.knight_attacks);
    precompute_king(each_tile, &mut empty_attack_sets.king_attacks);
    precompute_pawns(each_tile, empty_attack_sets);

    // precompute sliding pieces
}

fn precompute_knight(each_tile: [u64; 64], knight_attacks: &mut [u64; 64]) {   
    let mut count: usize = 0;
    let k = knight::Knight{};
    for t in each_tile {
        knight_attacks[count] = k.all_moves_unbound(t);
        count += 1;
    }
}

fn precompute_king(each_tile: [u64; 64], king_attacks: &mut [u64; 64]) {
    let mut count: usize = 0;
    let k = king::King{};
    for t in each_tile {
        king_attacks[count] = k.all_moves_unbound(t);
        count += 1;
    }
}

fn precompute_pawns(each_tile: [u64; 64], empty_attack_sets: &mut AttackSets) {
    let mut count = 0;
    let p = pawn::Pawn{};
    for t in each_tile {
        empty_attack_sets.w_pawn_moves[count] = p.all_moves_unbound(t, &Color::WHITE);
        empty_attack_sets.w_pawn_attacks[count] = p.all_attacks(t, &Color::WHITE);

        empty_attack_sets.b_pawn_moves[count] = p.all_moves_unbound(t, &Color::BLACK);
        empty_attack_sets.b_pawn_attacks[count] = p.all_attacks(t, &Color::BLACK);
        count += 1;
    }
}

#[cfg(test)]
pub mod test_util {
    // assert eq with print board
    pub fn board_string(b: u64) -> String {
        let mut formatted_str: String = "".to_string();
        let mut mask: u64 = 0xff00000000000000;
        for i in (0..8).rev() {
            formatted_str.push_str(&format!("{}|{:08b}\n", i+1, (((b & mask) >> (8*i)).reverse_bits() >> 56) ));
            mask = mask >> 8;
        }
        formatted_str.push_str("----------\n");
        formatted_str.push_str(" |ABCDEFGH\n");

        return formatted_str;
    }

    /// board diff
    pub fn bdiff(b: u64, expected: u64) {
        return assert_eq!(expected, b, "\nexpected: \n{}\nreceived:\n{}", board_string(expected), board_string(b));
    }
}
