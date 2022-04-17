// pub mod king;

use crate::board::{File, Board};

pub trait Piece {
    fn _all_moves_unbound(&self, piece_pos: u64, pos: Vec<u32>) -> u64;
    fn all_moves(&self, curr_pos: u64) -> u64;
    fn legal_moves(&self, board: Board) -> u64;
    // fn move(&self, board: GameBoard)
}

// up-down
pub fn mask_file(f: File) -> u64 {
    let mut res: u64 = 0;    
    let exp_val: u64 = u64::pow(2, f.value());

    for i in 0 .. 8 {
        res += exp_val.checked_shl(8 * i).unwrap();
    }
    return res;
}

pub fn mask_rank(r: u32) -> u64 {
    let res: u64 = 255; // 2^8 - 1
    return res.checked_shl(8 * (r - 1)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_file() {
        assert_eq!(0x0101010101010101, mask_file(File::A));
        assert_eq!(0x0202020202020202, mask_file(File::B));
        assert_eq!(0x0404040404040404, mask_file(File::C));
        assert_eq!(0x0808080808080808, mask_file(File::D));
        assert_eq!(0x1010101010101010, mask_file(File::E));
        assert_eq!(0x2020202020202020, mask_file(File::F));
        assert_eq!(0x4040404040404040, mask_file(File::G));
        assert_eq!(0x8080808080808080, mask_file(File::H));
    }

    #[test]
    fn test_mask_rank() {
        assert_eq!(0x00000000000000ff, mask_rank(1));
        assert_eq!(0x000000000000ff00, mask_rank(2));
        assert_eq!(0x0000000000ff0000, mask_rank(3));
        assert_eq!(0x00000000ff000000, mask_rank(4));
        assert_eq!(0x000000ff00000000, mask_rank(5));
        assert_eq!(0x0000ff0000000000, mask_rank(6));
        assert_eq!(0x00ff000000000000, mask_rank(7));
        assert_eq!(0xff00000000000000, mask_rank(8));
    }
}