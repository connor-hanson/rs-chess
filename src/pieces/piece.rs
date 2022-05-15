// pub mod king;

use crate::game::{File, Board, Color};

pub trait Piece {    
    /// Bound by the presence of other pieces. 
    /// Consider: 
    /// - Checks
    /// - Pins
    /// - Checked Squares (King)
    /// - Pawn 
    fn all_moves(&self, board: &Board, color: &Color) -> u64;

    /// Used for generation of a piece's movesets
    /// The presence of which drastically reduces the amount of calculations needed for a move
    fn all_moves_pseudolegal_no_blocks(position: u64) -> u64;

    /// The amount of points provided on Piece capture
    fn get_points(&self) -> i32;

    // fn move(&self, board: GameBoard)
}