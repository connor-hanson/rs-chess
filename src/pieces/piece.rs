// pub mod king;

use crate::board::{File, Board};

#[derive(PartialEq)]
pub enum Color {
    WHITE, BLACK
}

pub trait Piece {
    
    /// not bound by presence of other pieces. Borders are still computed here
    fn all_moves_unbound(&self, piece_pos: u64) -> u64;

    /// Bound by the presence of other pieces. 
    /// Consider: 
    /// - Checks
    /// - Pins
    /// - Checked Squares (King)
    /// - Pawn Movement
    fn all_moves(&self, curr_pos: u64, color: Color) -> u64;
    fn legal_moves(&self, board: Board) -> u64;
    // fn move(&self, board: GameBoard)
}