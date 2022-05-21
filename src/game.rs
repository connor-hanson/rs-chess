use std::io::{Error, ErrorKind};

use crate::utils::{tile_u64, tile_list_u64, mask_rank};

pub struct Board {
    pub w_king: u64,
    pub w_queens: u64,
    pub w_bishops: u64,
    pub w_knights: u64,
    pub w_rooks: u64,
    pub w_pawns: u64,

    pub bb_king: u64,
    pub b_queens: u64,
    pub b_bishops: u64,
    pub b_knights: u64,
    pub b_rooks: u64,
    pub b_pawns: u64,

    /// Pieces owned by white
    pub w_pieces: u64, 
    /// Pieces white can capture minus black's king
    pub w_can_capture: u64, 
    /// Pieces owned by black
    pub b_pieces: u64, 
    ///Pieces black can capture minus white's king
    pub b_can_capture: u64,
}

impl Default for Board {
    fn default() -> Self {
        return Self {
            w_king: tile_u64("e1"),
            w_queens: tile_u64("d1"),
            w_bishops: tile_list_u64(vec!["c1", "f1"]),
            w_knights: tile_list_u64(vec!["b1", "g1"]),
            w_rooks: tile_list_u64(vec!["a1", "h1"]),
            w_pawns: mask_rank(2),

            bb_king: tile_u64("e8"),
            b_queens: tile_u64("d8"),
            b_bishops: tile_list_u64(vec!["c8", "f8"]),
            b_knights: tile_list_u64(vec!["b8", "g8"]),
            b_rooks: tile_list_u64(vec!["a8", "h8"]),
            b_pawns: mask_rank(7),

            w_pieces: mask_rank(1) | mask_rank(2),
            w_can_capture: mask_rank(7) | mask_rank(8) & (!tile_u64("e8")),
            b_pieces: mask_rank(7) | mask_rank(8),
            b_can_capture: mask_rank(1) | mask_rank(2) & (!tile_u64("e1")),
        }
    }
}

impl Board {
    // expensive in memory, but gets cleaned up after program initiation
    pub fn each_tile() -> [u64; 64] {
        let mut each_tile: [u64; 64] = [0; 64];
        let mut count: usize = 0;
        for r in Rank::all() {
            for f in File::all() {
                each_tile[count] = f.mask() & r.mask();
                count += 1;
            }
        }

        return each_tile;
    }
}

#[derive(PartialEq)]
pub enum Color {
    WHITE, BLACK
}

#[derive(PartialEq)]
pub enum File {
    A, B, C, D, E, F, G, H
}

impl File {
    pub fn value(&self) -> u32 {
        match *self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
    
    pub fn all() -> [File; 8] {
        return [
            File::A, 
            File::B, 
            File::C,
            File::D, 
            File::E, 
            File::F, 
            File::G,
            File::H,
        ];
    }

    pub fn mask(&self) -> u64 {
        match *self {
            File::A => 0x0101010101010101,
            File::B => 0x0202020202020202,
            File::C => 0x0404040404040404,
            File::D => 0x0808080808080808,
            File::E => 0x1010101010101010,
            File::F => 0x2020202020202020,
            File::G => 0x4040404040404040,
            File::H => 0x8080808080808080,
        }
    }

    pub fn file_from_tile(tile: u64) -> Result<File, String> {
        for file in File::all() {
            if (file.mask() & tile) != 0 {
                return Ok(file);
            }
        }

        return Err("No file could be found".to_string());
    }
}

#[derive(PartialEq)]
pub enum Rank {
    ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT
}

impl Rank {
    pub fn all() -> [Rank; 8] {
        return [
            Rank::ONE,
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE,
            Rank::SIX,
            Rank::SEVEN, 
            Rank::EIGHT,
        ];
    }

    pub fn mask(&self) -> u64 {
        match *self {
            Rank::ONE => 0x00000000000000ff,
            Rank::TWO => 0x000000000000ff00,
            Rank::THREE => 0x0000000000ff0000,
            Rank::FOUR => 0x00000000ff000000,
            Rank::FIVE => 0x000000ff00000000,
            Rank::SIX => 0x0000ff0000000000,
            Rank::SEVEN => 0x00ff000000000000,
            Rank::EIGHT => 0xff00000000000000,
        }
    }

    pub fn rank_from_tile(tile: u64) -> Result<Rank, String> {
        for rank in Rank::all() {
            if (rank.mask() & tile) != 0 {
                return Ok(rank);
            }
        }

        return Err("No rank could be found".to_string());
    }
}
