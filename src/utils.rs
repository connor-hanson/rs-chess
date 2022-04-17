
use std::str::Chars;
use super::board::File;

pub fn mask_rank(r: u32) -> u64 {
    let res: u64 = 255; // 2^8 - 1
    return res.checked_shl(8 * (r - 1)).unwrap();
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

pub fn tile_u64(t: &str) -> u64 {
    let tile: String = t.to_string();
    if tile.len() != 2 {
        panic!("input {} is not len 2!", tile);
    }

    let mut chars: Chars = tile.chars();

    let file: char = chars.next().unwrap().to_ascii_uppercase();
    let rank: u64 = chars.next().unwrap().to_digit(10).unwrap() as u64;

    let file_num: u64 = file as u64 - 65;

    return 1 << file_num << ((rank - 1) * 8);
}

pub fn tile_list_u64(t: Vec<&str>) -> u64 {
    let mut res: u64 = 0;
    for i in t {
        res += tile_u64(i);
    }

    return res;
}

fn print_moves(b: u64) {
    let mut mask: u64 = 0xff00000000000000;
    for i in (0..8).rev() {
        println!("{:08b}", (((b & mask) >> (8 * i)).reverse_bits()) >> 56);
        mask = mask >> 8;
    }
    println!("{:064b}", b);
    println!("");
}

#[cfg(test)]
mod test {
    use crate::utils::*;

    #[test]
    fn test_tile_u64() {
        assert_eq!(tile_u64("A1"), 0x0000000000000001);
        assert_eq!(tile_u64("H1"), 0x0000000000000080);
        
        assert_eq!(tile_u64("a8"), 0x0100000000000000);
        assert_eq!(tile_u64("h8"), 0x8000000000000000);

        // test center
        assert_eq!(tile_u64("d4"), 0x0000000008000000);

    }
    
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
