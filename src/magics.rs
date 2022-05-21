use crate::game::{Rank, File};
use crate::utils::tile_u64;



/**
 * 
 * 1. Compute the magic for each tile, for each piece
 * 2. For each tile:
 *  Choose a random magic
 *  Get all the attack variations for that tile
 *  create idx: magic * variation >> 64 - tile bits
 *  check that there are no conflicts for each variation
 * 
 */
pub fn find_magics() {
    // find rook magics

    // find bishop magics
}

fn rook_magics() {

}

pub fn rook_all_occupancies() -> Vec<u64> {

    let pos = tile_u64("a1");

    let p_rank_mask: u64 = match Rank::rank_from_tile(pos) {
        Ok(rank) => rank.mask(),
        Err(e) => panic!("{:?}", e),
    };

    let p_file_mask = match File::file_from_tile(pos) {
        Ok(f) => f.mask(),
        Err(e) => panic!("{:?}", e),
    };

    let mut rank_occupancies: Vec<u64> = Vec::new();
    rank_occupancies.push(0);

    for r in Rank::all() {
        // magic bitboards don't compute the 1st or 8th ranks, ever
        // they also ignore the tile that the piece is on, since the piece can't attack its own square
        if r == Rank::ONE || r == Rank::EIGHT || r.mask() & pos != 0 {
            continue;
        }

        let mut tmp: Vec<u64> = Vec::new();
        // borrow the list elements instead of owning
        for &rank_combo in &rank_occupancies {
            tmp.push((rank_combo | r.mask()) & p_file_mask);
        }

        rank_occupancies.append(&mut tmp);
    }

    assert_eq!(rank_occupancies.len(), 64);

    let mut file_occupancies: Vec<u64> = Vec::new();
    file_occupancies.push(0); // empty occupancy is also valid

    for f in File::all() {
        // they also ignore the a and h file
        if f == File::A || f == File::H || f.mask() & pos != 0 {
            continue;
        }

        let mut tmp: Vec<u64> = Vec::new();

        for &file_occ in &file_occupancies {
            tmp.push((file_occ | f.mask()) & p_rank_mask);
        }

        file_occupancies.append(&mut tmp);
    }

    assert_eq!(file_occupancies.len(), 64);

    let mut final_occs = Vec::new();
    for &f in &file_occupancies {
        for &r in &rank_occupancies {
            final_occs.push(f | r);
        }
    }

    assert_eq!(final_occs.len(), 4096);
    return final_occs;
}


