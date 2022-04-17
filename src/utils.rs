mod utils {
    pub fn tile_u64(tile: String) -> u64 {
        if tile.chars.count != 2 {
            panic("input {} is not len 2!", tile);
        }

        let file: char = tile.chars().next().unwrap().to_ascii_uppercase();
        let rank: u64 = tile.chars().next().unwrap().to_digit(10).unwrap();

        let fileNum = file as u64 - 65;

        return 1 << fileNum << (rank * 8);
    }
}

mod test {
    use super::*;

    #[test]
    fn test_tile_u64() {
        assert_eq!(tile_u64("A1"), 0x0000000000000001);
    }
}
