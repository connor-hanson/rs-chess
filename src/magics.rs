use std::hash::Hash;
use std::vec;

use crate::game::{Rank, File, Board};
use crate::utils::print_moves;

use std::collections::HashSet;
use rand::Rng;


const DB_SIZE_FACTOR: u32 = 2;

struct RookMagic {
    offsets: [u32; 64],
    magics: [u64; 64],
    attacks: Vec<Vec<u64>>,
}

pub struct SlidingMagic {
    rook_offsets: [u32; 64],
    rook_magics: [u64; 64],
    rook_attacks: Vec<Vec<u64>>,
}


const ROOK_OFFSETS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, // a1 -> h1
    11, 10, 10, 10, 10, 10, 10, 11, // a2 -> h2
    11, 10, 10, 10, 10, 10, 10, 11, // a3 -> h3
    11, 10, 10, 10, 10, 10, 10, 11, // a4 -> h4
    11, 10, 10, 10, 10, 10, 10, 11, // a5 -> h5
    11, 10, 10, 10, 10, 10, 10, 11, // a6 -> h6
    11, 10, 10, 10, 10, 10, 10, 11, // a7 -> h7
    12, 11, 11, 11, 11, 11, 11, 12, // a8 -> h8
];

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

fn n_lower_bits(value: u64, mut n: u32) -> u64 {
    if n >= 64 {
        return u64::MAX;
    }
    let shift_mask = match u64::MAX.checked_shl(n) {
        None => panic!("Could not shl {} bits", n),
        Some(mask) => mask,
    };
    return value & !shift_mask
}

fn n_higher_bits(value: u64, mut n: u32) -> u64 {
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

fn idx_from_magic(magic: u64, occupancy: u64, tile_index: usize) -> usize {
    let (multres, _): (u64, bool) = magic.overflowing_mul(occupancy);
    let idx: usize = match multres.checked_shr(64 - ROOK_OFFSETS[tile_index] - DB_SIZE_FACTOR) {
        None => panic!("Index shifting failed with shift of {}", tile_index),
        Some(idx) => idx as usize,
    };
    return idx;
}

fn rook_magics_mib() {
    let mut total = 0;
    for i in ROOK_OFFSETS {
        total += i;
    }

    total *= 64; // number of bits per db slot
    total += total * DB_SIZE_FACTOR;
    println!("Rook lookup tables using {total}b of memory, or {}MiB", total as f32 / 1048576.0);
}

fn rook_magics() -> RookMagic {
    let each_tile: [u64; 64] = Board::each_tile();

    let mut rng = rand::thread_rng();

    let mut tile_attack_database: Vec<Vec<u64>> = vec![Vec::new(); 64];
    let mut tile_magic_database: [u64; 64] = [0; 64];

    for (i, tile) in each_tile.iter().enumerate() {
        println!("Working on {}...", i);

        let vec_len: usize = match (2 as usize).checked_pow(ROOK_OFFSETS[i] + DB_SIZE_FACTOR) { // doubling avail size
            None => panic!("Could not raise 2 pow {}", ROOK_OFFSETS[i] + DB_SIZE_FACTOR), 
            Some(len) => len,
        };
    
        let p_rank_mask: u64 = match Rank::rank_from_tile(*tile) {
            Ok(rank) => rank.mask(),
            Err(e) => panic!("{:?}", e),
        };
    
        let p_file_mask = match File::file_from_tile(*tile) {
            Ok(f) => f.mask(),
            Err(e) => panic!("{:?}", e),
        };

        let tile_occupancies: Vec<u64> = rook_all_occupancies(*tile, p_rank_mask, p_file_mask);
        let mut occupancy_attack_tuple: Vec<(u64, u64)> = Vec::new();
        let mut attack_hash_set: HashSet<u64> = HashSet::new();

        // pack tile occs and attacks into tuples
        for &occ in &tile_occupancies {
            let attacks: u64 = rook_attacks(*tile, p_rank_mask, p_file_mask, occ);
            occupancy_attack_tuple.push((occ, attacks));
            attack_hash_set.insert(attacks);
        }

        println!("Distinct attack sets: {}", attack_hash_set.len());
        println!("Relevant occupancy DB size: {}", vec_len);

        assert_eq!(occupancy_attack_tuple.len(), tile_occupancies.len());

        let mut magic_found: bool = false;
        let mut count = 0;
        while !magic_found {
            count += 1;

            if count % 1000000 == 0 {
                println!("Still searching for magic at {}", i);
            }
            magic_found = true;
            let magic = rng.gen::<u64>();
            let mut hashed_vec: Vec<u64> = vec![0; vec_len];

            let mut clashcount = 0;
            // if magic fails at any case, retry
            for &(occ, attack) in &occupancy_attack_tuple {
                let idx: usize = idx_from_magic(magic, occ, i);
                // if count % 10000 == 0 {
                //     println!("{}",idx);
                // }
                if hashed_vec[idx] != 0 && hashed_vec[idx] != attack {
                    magic_found = false;
                    // println!("Clash at i = {}, between ", clashcount);
                    // print_moves(&attack);
                    // print_moves(&hashed_vec[idx]);
                    break;
                }
                clashcount += 1;

                hashed_vec[idx] = attack;
            }

            if !magic_found {
                continue;
            }

            tile_magic_database[i] = magic;
            tile_attack_database[i] = hashed_vec;


            println!("Magic found: {}", magic);
        }
    }

    // track how much memory it uses
    rook_magics_mib();

    return RookMagic { 
        offsets: ROOK_OFFSETS, 
        magics: tile_magic_database, 
        attacks: tile_attack_database, 
    }
}

fn rook_attacks(tile: u64, p_rank_mask: u64, p_file_mask: u64, occupancy: u64) -> u64 {

    let tz = tile.trailing_zeros();
    if tz >= 64 {
        panic!("Given rook tile is empty")
    }

    let lower_bitmask = n_lower_bits(u64::MAX, tz);
    let higher_bitmask = n_higher_bits(u64::MAX, 63 - tz); // don't mask curr_tile

    assert!(
        lower_bitmask ^ higher_bitmask ^ tile == u64::MAX,
        "{:08b} ^ \n{:08b} ^ \n{:08b} \nshould be u64 max", 
        lower_bitmask, higher_bitmask, tile
    );

    // smallest occ north is last attack square, if it exists
    // unset all bits up to piece pos
    // piece pos is already unset in occ
    let north: u64 = occupancy & p_file_mask & higher_bitmask;

    // mask from the first bit after first occ
    let mut north_attacks: u64 = n_lower_bits(north | p_file_mask, north.trailing_zeros() + 1) & higher_bitmask;
    if north == 0 {
        north_attacks = p_file_mask & higher_bitmask & !tile;
    }

    let east: u64 = occupancy & p_rank_mask & higher_bitmask;
    let mut east_attacks: u64 = n_lower_bits(east | p_rank_mask, east.trailing_zeros() + 1) & higher_bitmask;
    if east == 0 {
        east_attacks = p_rank_mask & higher_bitmask & !tile;
    }

    let west: u64 = occupancy & p_rank_mask & lower_bitmask;
    let mut west_attacks: u64 = n_higher_bits(west | p_rank_mask, 64 - west.trailing_zeros()) & lower_bitmask;
    if west == 0  {
        west_attacks = p_rank_mask & lower_bitmask & !tile;
    }

    let south: u64 = occupancy & p_file_mask & lower_bitmask;
    let mut south_attacks: u64 = n_higher_bits(south | p_file_mask, 64 - south.trailing_zeros() + 1) & lower_bitmask;
    if south == 0 {
        south_attacks = p_file_mask & lower_bitmask & !tile;
    }

    let attacks: u64 = north_attacks | south_attacks | east_attacks | west_attacks;

    if attacks == 0 {
        println!("ERROR! ATTACK SET SHOULD != 0");
        println!("North Attacks/Occ: \n");
        print_moves(&(n_lower_bits(north,  north.trailing_zeros() + 1)));
        print_moves(&north);
        println!("South Attacks/Occ: \n");
        print_moves(&(n_lower_bits(south, south.trailing_zeros() + 1)));
        print_moves(&south);
        println!("West Attacks/Occ: \n");
        print_moves(&(n_lower_bits(west,  west.trailing_zeros() + 1)));
        print_moves(&west);
        println!("East Attacks/Occ: \n");
        print_moves(&(n_lower_bits(east,  east.trailing_zeros() + 1)));
        print_moves(&east);
        println!("High/low bitmask:\n");
        print_moves(&higher_bitmask);
        print_moves(&lower_bitmask);

    }

    return attacks;
}

pub fn rook_all_occupancies(pos: u64, p_rank_mask: u64, p_file_mask: u64) -> Vec<u64> {
    

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

    // assert_eq!(rank_occupancies.len(), 64);

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

    // assert_eq!(file_occupancies.len(), 64);

    let mut final_occs = Vec::new();
    for &f in &file_occupancies {
        for &r in &rank_occupancies {
            final_occs.push(f | r);
        }
    }

    // assert_eq!(final_occs.len(), 4096);
    return final_occs;
}

#[cfg(test)]
mod test {
    use crate::magics::*;
    use crate::game::Board;
    use crate::test_util::board_string;

    use std::collections::HashSet;

    #[test]
    fn test_rook_occupancies_and_attacks_not_own_tile() {
        for tile in Board::each_tile() {

            let p_rank_mask: u64 = match Rank::rank_from_tile(tile) {
                Ok(rank) => rank.mask(),
                Err(e) => panic!("{:?}", e),
            };
        
            let p_file_mask = match File::file_from_tile(tile) {
                Ok(f) => f.mask(),
                Err(e) => panic!("{:?}", e),
            };

            let rook_tile_occupancies = rook_all_occupancies(tile, p_rank_mask, p_file_mask);

            for occ in rook_tile_occupancies {
                assert_eq!(
                    tile & occ, 0, 
                    "Rook occupancy set overlaps position: \n Pos: \n{}, \n Occ: \n{}",
                    board_string(tile), board_string(occ)
                ); // assert no occupancies on the tile

                let attacks = rook_attacks(tile, p_rank_mask, p_file_mask, occ);
                assert_eq!(
                    tile & attacks, 0, 
                    "Rook attacks its own tile: \n Pos: \n{} \n Attack Set: \n{} \n", 
                    board_string(tile), board_string(attacks));
                assert_ne!(
                    attacks, 0,
                    "Rook attack set is empty at: \n {} \nfor occupancy set\n{}", 
                    board_string(tile), board_string(occ)
                ); // there should never be an empty attack set
            } 
        }
    }

    #[test]
    fn test_rook_attack_cardinalities() {
        for tile in Board::each_tile() {
            // get the length from each side of the board
            let p_rank: Rank = match Rank::rank_from_tile(tile) {
                Ok(rank) => rank,
                Err(e) => panic!("{:?}", e),
            };
        
            let p_file: File = match File::file_from_tile(tile) {
                Ok(f) => f,
                Err(e) => panic!("{:?}", e),
            };

            let mut possible_n_attacks = match p_rank {
                Rank::ONE => 7,
                Rank::TWO => 6,
                Rank::THREE => 5,
                Rank::FOUR => 4,
                Rank::FIVE => 3,
                Rank::SIX => 2,
                Rank::SEVEN => 1,
                Rank::EIGHT => 0,
            };

            let mut possible_s_attacks = 7 - possible_n_attacks;

            if possible_n_attacks == 0 {
                possible_n_attacks = 1;
            } 
            if possible_s_attacks == 0 {
                possible_s_attacks = 1;
            }

            let mut possible_e_attacks = match p_file {
                File::A => 7,
                File::B => 6,
                File::C => 5,
                File::D => 4,
                File::E => 3,
                File::F => 2,
                File::G => 1,
                File::H => 0,
            };

            let mut possible_w_attacks = 7 - possible_e_attacks;

            if possible_e_attacks == 0 {
                possible_e_attacks = 1;
            }
            if possible_w_attacks == 0 {
                possible_w_attacks = 1;
            }

            let mut attack_set: HashSet<u64> = HashSet::new();

            let rook_occs = rook_all_occupancies(
                tile, p_rank.mask(), p_file.mask()
            );

            for occ in rook_occs {
                let attacks = rook_attacks(
                    tile, p_rank.mask(), p_file.mask(), occ
                );

                attack_set.insert(attacks);   
            }

            for &attack in &attack_set {
                //println!("{}", board_string(attack));
                let mut num_matches = 0;
                for &all_others in &attack_set {
                    if attack == all_others {
                        num_matches += 1;
                    }
                }

                // ensure all attacks are within rays
                assert!(
                    attack & !p_rank.mask() & !p_file.mask() == 0, 
                    "Rook is attacking square out of LOS, Attacks: {}",
                    board_string(attack),
                );

                // do any attacks have 'disjoint' rays
                // check the rank, then the file
                let mut disjoint: bool = false;
                let mut first_bit_found = false;
                let mut last_bit_found: bool = false;

                for file in File::all() {
                    let bit = p_rank.mask() & file.mask();
                    assert!(bit != 0);
                    if bit & attack != 0 {
                        // start counting
                        first_bit_found = true;
                    } else if bit & tile != 0 {
                        first_bit_found = true;
                    } else if first_bit_found && (bit & attack) == 0 {
                        last_bit_found = true;
                    } else if (first_bit_found || last_bit_found) && bit & attack != 0 {
                        disjoint = true;
                    }
                }

                first_bit_found = false;
                last_bit_found = false;
                for rank in Rank::all() {
                    let bit = p_file.mask() & rank.mask();
                    assert!(bit != 0);
                    if bit & attack != 0 {
                        // start counting
                        first_bit_found = true;
                    } else if bit & tile != 0 {
                        first_bit_found = true;
                    } else if first_bit_found && (bit & attack) == 0 {
                        last_bit_found = true;
                    } else if (first_bit_found || last_bit_found) && bit & attack != 0 {
                        disjoint = true;
                    }
                }

                assert!(
                    !disjoint, 
                    "Attack set is disjoint. Tile is {}, Attack set is {}", 
                    board_string(tile), board_string(attack)
                );
                assert!(num_matches < 2 && num_matches > 0);
            }

            let cardinality = possible_e_attacks * possible_n_attacks * possible_w_attacks * possible_s_attacks;

            if cardinality != attack_set.len() {
                for a in &attack_set {
                    print_moves(a);
                }
            }

            assert_eq!(
                attack_set.len(), 
                cardinality, 
                "Expected {} attacks, but got {} for the tile \n{}", 
                cardinality, attack_set.len(), board_string(tile)
            );
        }
    }

    #[test]
    fn test_generate_magic_nums() {
        find_magics();
    }
}

