use crate::pieces::piece::{
    Piece, 
    Color,
};
use crate::board::{Board, File};
use crate::utils::{mask_rank, mask_file};

pub struct Knight{}

impl Piece for Knight {
    fn all_moves(&self, curr_pos: u64, color: &Color) -> u64 {
        return 1;
    }
}

impl Knight {
    fn all_moves_unbound(&self, curr_pos: u64) -> u64 {
        let mut res: u64 = 0;

        res |= self.attacks_east(curr_pos);
        res |= self.attacks_west(curr_pos);
        return res;
    }

    fn attacks_east(&self, curr_pos: u64) -> u64 {
        let tz = curr_pos.trailing_zeros();
        let mask;
        if (tz % 8) > 4 {
            mask = !(mask_file(File::A) | mask_file(File::B));
        } else {
            mask = !0;
        }
        
        return mask & (
            curr_pos.checked_shr(6).unwrap() | curr_pos.checked_shr(15).unwrap() |
            curr_pos.checked_shl(10).unwrap() | curr_pos.checked_shl(17).unwrap()
        );
    }

    fn attacks_west(&self, curr_pos: u64) -> u64 {
        let mask;
        let tz = curr_pos.trailing_zeros();
        if (tz % 8) > 4 {
            mask = !0;
        } else {
            mask = !(mask_file(File::G) | mask_file(File::H));
        }
        
        return mask & (
            curr_pos.checked_shr(10).unwrap() | curr_pos.checked_shr(17).unwrap() |
            curr_pos.checked_shl(6).unwrap() | curr_pos.checked_shl(15).unwrap());
    }

}

#[cfg(test)]
mod test {

    use crate::utils::{tile_u64, tile_list_u64};
    use crate::test_util::bdiff;
    use super::*;

    #[test]
    fn test_horsey_moves() {
        let k = Knight{};
        bdiff(k.all_moves_unbound(tile_u64("C4")), 
        tile_list_u64(vec!["A3", "A5", "B6", "D6", "E5", "E3", "D2", "B2"]));

        bdiff(k.all_moves_unbound(tile_u64("d6")), 
        tile_list_u64(vec!["c4", "e4", "f5", "f7", "e8", "c8", "b7", "b5"]));

        bdiff(k.all_moves_unbound(tile_u64("a1")), tile_list_u64(vec!["b3", "c2"]));
        bdiff(k.all_moves_unbound(tile_u64("H8")), tile_list_u64(vec!["f7", "g6"]));
    }

}