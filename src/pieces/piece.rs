// pub mod king;

use crate::board::{File, Board};

#[derive(PartialEq)]
pub enum Color {
    WHITE, BLACK
}

pub trait Piece {
    
    /// Bound by the presence of other pieces. 
    /// Consider: 
    /// - Checks
    /// - Pins
    /// - Checked Squares (King)
    /// - Pawn 
    fn all_moves(&self, curr_pos: u64, color: &Color) -> u64;
    // fn move(&self, board: GameBoard)
}