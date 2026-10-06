
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplier {
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

const N: Multiplier = Multiplier::Normal;
const DL: Multiplier = Multiplier::DoubleLetter;
const TL: Multiplier = Multiplier::TripleLetter;
const DW: Multiplier = Multiplier::DoubleWord;
const TW: Multiplier = Multiplier::TripleWord;


pub const BOARD_MULTIPLIERS: [Multiplier; 225] = [
    TW, N,  N,  DL, N,  N,  N,  TW, N,  N,  N,  DL, N,  N,  TW,
    N,  DW, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  DW, N, 
    N,  N,  DW, N,  N,  N,  DL, N,  DL, N,  N,  N,  DW, N,  N, 
    DL, N,  N,  DW, N,  N,  N,  DL, N,  N,  N,  DW, N,  N,  DL,
    N,  N,  N,  N,  DW, N,  N,  N,  N,  N,  DW, N,  N,  N,  N, 
    N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N, 
    N,  N,  DL, N,  N,  N,  DL, N,  DL, N,  N,  N,  DL, N,  N, 
    TW, N,  N,  DL, N,  N,  N,  DL,  N,  N,  N,  DL, N,  N,  TW,
    N,  N,  DL, N,  N,  N,  DL, N,  DL, N,  N,  N,  DL, N,  N, 
    N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N, 
    N,  N,  N,  N,  DW, N,  N,  N,  N,  N,  DW, N,  N,  N,  N, 
    DL, N,  N,  DW, N,  N,  N,  DL, N,  N,  N,  DW, N,  N,  DL,
    N,  N,  DW, N,  N,  N,  DL, N,  DL, N,  N,  N,  DW, N,  N, 
    N,  DW, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  DW, N, 
    TW, N,  N,  DL, N,  N,  N,  TW, N,  N,  N,  DL, N,  N,  TW,
];

// TILE_SCORES[(char as u8 - b'A') as usize]
pub const TILE_SCORES: [u8; 27] = [
    1,  // A
    3,  // B
    3,  // C
    2,  // D
    1,  // E
    4,  // F
    2,  // G
    4,  // H
    1,  // I
    8,  // J
    5,  // K
    1,  // L
    3,  // M
    1,  // N
    1,  // O
    3,  // P
    10, // Q
    1,  // R
    1,  // S
    1,  // T
    1,  // U
    4,  // V
    4,  // W
    8,  // X
    4,  // Y
    10, // Z
    0,  // [Blank]
];

pub struct Board {
    board: [u8; 225],
    p1_score: u8,
    p2_score: u8
}


impl Board {
    pub fn new() -> Self {
        Self {
            board: [0; 225],
            p1_score: 0,
            p2_score: 0
        }
    }


    pub fn state(&self) {
        for row in self.board.chunks(15) {
            println!("{:?}", row);
        }
    }
}