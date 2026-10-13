use bitvec::prelude::*;
use rand::seq::IndexedRandom;


type BitField = BitArr!(for 100, in u64, Lsb0);

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
    TW, N,  N,  DL, N,  N,  N,  DL, N,  N,  N,  DL, N,  N,  TW,
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

pub const TILE_MAP: [u8; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0,          // A (9)
    1, 1,                               // B (2)
    2, 2,                               // C (2)
    3, 3, 3, 3,                         // D (4)
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // E (12)
    5, 5,                               // F (2)
    6, 6, 6,                            // G (3)
    7, 7,                               // H (2)
    8, 8, 8, 8, 8, 8, 8, 8, 8,          // I (9)
    9,                                  // J (1)
    10,                                 // K (1)
    11, 11, 11, 11,                     // L (4)
    12, 12,                             // M (2)
    13, 13, 13, 13, 13, 13,             // N (6)
    14, 14, 14, 14, 14, 14, 14, 14,     // O (8)
    15, 15,                             // P (2)
    16,                                 // Q (1)
    17, 17, 17, 17, 17, 17,             // R (6)
    18, 18, 18, 18,                     // S (4)
    19, 19, 19, 19, 19, 19,             // T (6)
    20, 20, 20, 20,                     // U (4)
    21, 21,                             // V (2)
    22, 22,                             // W (2)
    23,                                 // X (1)
    24, 24,                             // Y (2)
    25,                                 // Z (1)
    26, 26                              // Blank (2)
];


pub struct Board {
    board: [u8; 225],
    p1_score: u16,
    p2_score: u16,
    bag: BitField,
    pub racks: [BitField; 2]
}


impl Board {
    pub fn new() -> Self {
        Self {
            board: [0; 225],
            p1_score: 0,
            p2_score: 0,
            bag: bitarr![u64, Lsb0; 1; 100],
            racks: [bitarr![u64, Lsb0; 0; 100]; 2],
        }
    }


    pub fn state(&self) {
        println!("\n--Board State--");
        for row in self.board.chunks(15) {
            println!("{:?}", row);
        }
        println!("bag\t {:b}", self.bag);
        println!("p1 rack\t {:b}", self.racks[0]);
        println!("p2 rack\t {:b}", self.racks[1]);
    }


    pub fn draw(&mut self, n: usize, player: usize) {
        let set_indices: Vec<usize> = self.bag.iter()
            .enumerate()
            .filter_map(|(idx, bit)| (*bit).then_some(idx))
            .collect();

        let move_count = n.min(set_indices.len());
        let mut rng = rand::rng();

        for &idx in set_indices.sample(&mut rng, move_count) {
            self.bag.set(idx, false);
            self.racks[player].set(idx, true);
        }
    }


    pub fn exchange(&mut self, player: usize, exchange_tiles: BitField) -> Result<(), &'static str> {
        if self.racks[player] & exchange_tiles == exchange_tiles {
            self.draw(exchange_tiles.count_ones(), player);
            self.racks[player] ^= exchange_tiles;
            self.bag |= exchange_tiles;

            Ok(())
        } else {
            Err("Invalid exchange mask")
        }
    }


    pub fn play(&mut self, player: usize, play_tiles: BitField, pos_tiles: &[(usize, usize)]) -> Result<(), &'static str> {
        if self.racks[player] & play_tiles == play_tiles {
            for &(tile_pos, tile_idx) in pos_tiles.iter() {
                let tile_val = TILE_MAP[tile_idx];
                //let board_copy = self.board;
                self.board[tile_pos] = tile_val;
                //let valid = validate();
                // if valid == true {update rack, OK}
                // else {unmake board, Err}
            }
            self.racks[player] ^= play_tiles;
            Ok(())
        } else {
            Err("Invalid play mask")
        }
    }
}
