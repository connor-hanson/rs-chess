use crate::magic::rook_magics::{
    rook_magics,
    RookMagic,
};

pub struct SlidingMagic {
    pub rook_offsets: [u32; 64],
    pub rook_magics: [u64; 64],
    pub rook_attacks: Vec<Vec<u64>>,
}

/**
 * 
 * 1. Compute the magic for each tile, for each piece
 * 2. For each tile:
 *  Choose a random magic
 *  Get all the attack variations for that tile
 *  create idx: magic * variation >> 64 - tile bits
 *  check that there are no conflicts for each variation
 *  if none, generate attack set and store
 * 
 */
pub fn find_magics() -> SlidingMagic {
    // find rook magics
    let rmagic: RookMagic = rook_magics();

    // find bishop magics

    return SlidingMagic {
        rook_offsets: rmagic.offsets,
        rook_magics: rmagic.magics,
        rook_attacks: rmagic.attacks,
    }
}
