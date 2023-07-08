use rand::Rng;

use crate::magic::constants::{
    ROOK_OFFSETS,
    DB_SIZE_FACTOR,
};

/// mask value against n bottom bits
pub fn n_lower_bits(value: u64, n: u32) -> u64 {
    if n >= 64 {
        return u64::MAX;
    }
    let shift_mask = match u64::MAX.checked_shl(n) {
        None => panic!("Could not shl {} bits", n),
        Some(mask) => mask,
    };
    return value & !shift_mask
}

/// mask value against n top bits
pub fn n_higher_bits(value: u64, mut n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n > 64 {
        n = 64;
    }
    n = 64 - n;
    let shift_mask = match u64::MAX.checked_shl(n) {
        None => panic!("Could not shl {} bits", n),
        Some(mask) => mask,
    };
    return value & shift_mask
}

pub fn rook_magics_mib() {
    let mut total = 0;
    for i in ROOK_OFFSETS {
        total += i;
    }

    total *= 64; // number of bits per db slot
    total += total * DB_SIZE_FACTOR;
    println!("Rook lookup tables using {total}b of memory, or {}MiB", total as f32 / 1048576.0);
}

/// calculate the index in the magic array for a given tile
pub fn idx_from_magic(magic: u64, occupancy: u64, tile_index: usize) -> usize {
    let (multres, _): (u64, bool) = magic.overflowing_mul(occupancy);
    let idx: usize = match multres.checked_shr(64 - ROOK_OFFSETS[tile_index] - DB_SIZE_FACTOR) {
        None => panic!("Index shifting failed with shift of {}", tile_index),
        Some(idx) => idx as usize,
    };
    return idx;
}

pub fn search_for_magic_without_conflicts(
    table_size: usize, 
    tile_index: usize,
    occupancy_attack_tuple: Vec<(u64, u64)>
) -> (u64, Vec<u64>){

    let mut rng = rand::thread_rng();
    let mut magic_found: bool = false;


    while !magic_found {
        magic_found = true;
        let magic = rng.gen::<u64>();
        let mut magic_hash_table: Vec<u64> = vec![0; table_size];

        // if magic fails at any case, retry
        for &(occ, attack) in &occupancy_attack_tuple {
            let idx: usize = idx_from_magic(magic, occ, tile_index);
            if magic_hash_table[idx] != 0 && magic_hash_table[idx] != attack {
                magic_found = false;
                // println!("Clash at i = {}, between ", clashcount);
                // print_moves(&attack);
                // print_moves(&hashed_vec[idx]);
                break;
            }
            magic_hash_table[idx] = attack;
        }

        if !magic_found {
            continue;
        }


        println!("Magic found: {}", magic);
        return (magic, magic_hash_table);
    }

    panic!("Failing searching for magic.")
}


// /// bits to the west of file inclusive of file
// pub fn bits_west_of_file(file: File) {

// }

