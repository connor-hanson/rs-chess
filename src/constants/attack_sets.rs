// order a1-> ... -> h1 -> a2 -> ... -> h8:
// 0, ..., 7, 8, ..., 63 


/**
 * 2^12 = 4096
 * 2^11 = 2048
 * 2^10 = 1024
 * 2^9 = 512
 * 2^8 = 256
 * 2^7 = 128
 * 2^6 = 64
 * 2^5 = 32 
 * 2^4 = 16
 * 2^3 = 8
 * 2^2 = 4
 * 2^1 = 2
 */
pub struct AttackSets {
    pub rook_attacks: [Vec<u64>; 64],
    pub rook_magics: [u64; 64],

    pub bishop_attacks: [Vec<u64>; 64],
    pub bishop_offsets: [u64; 64],
    pub bishop_magics: [u64; 64],

    pub knight_attacks: [u64; 64],
    pub b_pawn_moves: [u64; 64],
    pub b_pawn_attacks: [u64; 64],
    pub w_pawn_moves: [u64; 64],
    pub w_pawn_attacks: [u64; 64],

    pub king_attacks: [u64; 64],
}

impl Default for AttackSets {
    fn default() -> Self {
        return Self {
            rook_attacks: [
                vec![0; 4096], // a1
                vec![0; 2048], 
                vec![0; 2048],
                vec![0; 2048],
                vec![0; 2048], 
                vec![0; 2048],
                vec![0; 2048],
                vec![0; 4096], // h1
                vec![0; 2048], // a2
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h2
                vec![0; 2048], // a3
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h3
                vec![0; 2048], // a4
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h4
                vec![0; 2048], // a5
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h5
                vec![0; 2048], // a6
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h6
                vec![0; 2048], // a7
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 1024],
                vec![0; 2048], // h7
                vec![0; 4096], // a8
                vec![0; 2048], 
                vec![0; 2048],
                vec![0; 2048],
                vec![0; 2048], 
                vec![0; 2048],
                vec![0; 2048],
                vec![0; 4096], // h8
            ],
            rook_magics: [0; 64],

            bishop_attacks: [
                vec![0; 64], // a1
                vec![0; 32],
                vec![0; 32], 
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 64], // h1
                vec![0; 32], // a2
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32], // h2
                vec![0; 32], // a3
                vec![0; 32],
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 32],
                vec![0; 32], // h3
                vec![0; 32], // a4
                vec![0; 32], 
                vec![0; 128],
                vec![0; 512], // d4
                vec![0; 512], // e4
                vec![0; 128],
                vec![0; 32],
                vec![0; 32], // h4
                vec![0; 32], // a5
                vec![0; 32], 
                vec![0; 128],
                vec![0; 512], // d5
                vec![0; 512], // e5
                vec![0; 128],
                vec![0; 32],
                vec![0; 32], // h5
                vec![0; 32], // a6
                vec![0; 32],
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 128], 
                vec![0; 32],
                vec![0; 32], // h6
                vec![0; 32], // a7
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32], // h7
                vec![0; 64], // a8
                vec![0; 32],
                vec![0; 32], 
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 32],
                vec![0; 64], // h8
            ],
            bishop_offsets: [
                6, 5, 5, 5, 5, 5, 5, 6,
                5, 5, 5, 5, 5, 5, 5, 5,
                5, 5, 7, 7, 7, 7, 5, 5,
                5, 5, 7, 9, 9, 7, 5, 5, 
                5, 5, 7, 9, 9, 7, 5, 5, 
                5, 5, 7, 7, 7, 7, 5, 5,
                5, 5, 5, 5, 5, 5, 5, 5,
                6, 5, 5, 5, 5, 5, 5, 5,
            ],
            bishop_magics: [0; 64],
    
            knight_attacks: [0; 64],
            b_pawn_moves: [0; 64],
            b_pawn_attacks: [0; 64],
            w_pawn_attacks: [0; 64],
            w_pawn_moves: [0; 64],
            king_attacks: [0; 64],
        }
      
    }
}