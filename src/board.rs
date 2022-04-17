pub struct Board {
    pub w_king: u64,
    pub bb_king: u64,
    pub w_pawns: u64,
    pub b_pawns: u64, // ...
}

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
}
