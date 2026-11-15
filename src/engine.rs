use bitvec::prelude::*;
use rand::seq::IndexedRandom;
use crate::trie::Trie;


type BitField = BitArr!(for 100, in u64, Lsb0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplier {
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}


enum Direction {
    Vertical,
    Horizontal,
}


const N: Multiplier = Multiplier::Normal;
const DL: Multiplier = Multiplier::DoubleLetter;
const TL: Multiplier = Multiplier::TripleLetter;
const DW: Multiplier = Multiplier::DoubleWord;
const TW: Multiplier = Multiplier::TripleWord;

const EMPTY: u8 = 27;


pub const BOARD_MULTIPLIERS: [Multiplier; 225] = [
    TW, N,  N,  DL, N,  N,  N,  TW, N,  N,  N,  DL, N,  N,  TW,
    N,  DW, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  DW, N, 
    N,  N,  DW, N,  N,  N,  DL, N,  DL, N,  N,  N,  DW, N,  N, 
    DL, N,  N,  DW, N,  N,  N,  DL, N,  N,  N,  DW, N,  N,  DL,
    N,  N,  N,  N,  DW, N,  N,  N,  N,  N,  DW, N,  N,  N,  N, 
    N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N,  N,  N,  TL, N, 
    N,  N,  DL, N,  N,  N,  DL, N,  DL, N,  N,  N,  DL, N,  N, 
    TW, N,  N,  DL, N,  N,  N,  DW, N,  N,  N,  DL, N,  N,  TW,
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
    pub racks: [BitField; 2],
    first_turn: bool,
    dictionary: Trie,
}


impl Board {
    pub fn new(dict: Trie) -> Self {
        Self {
            board: [EMPTY; 225],
            p1_score: 0,
            p2_score: 0,
            bag: bitarr![u64, Lsb0; 1; 100],
            racks: [bitarr![u64, Lsb0; 0; 100]; 2],
            first_turn: true,
            dictionary: dict
        }
    }


    pub fn state(&self) {
        println!("\n    A B C D E F G H I J K L M N O");
        println!("  +-------------------------------+");
        for (i, row) in self.board.chunks(15).enumerate() {
            let row_str: String = row.iter()
                .map(|&x| match x {
                    EMPTY => '.',
                    26 => '*',
                    _ => (x + b'A') as char,
                })
                .flat_map(|c| vec![' ', c])
                .collect();
            println!("{:2}|{}", i + 1, row_str);
        }
        println!("  +-------------------------------+\n");
        
        for p in 0..2 {
            let mut rack_str = String::new();
            for idx in 0..100 {
                if self.racks[p][idx] {
                    let val = TILE_MAP[idx];
                    let ch = if val == 26 { '*' } else { (val + b'A') as char };
                    rack_str.push(ch);
                    rack_str.push(' ');
                }
            }
            println!("P{} Rack: {}", p + 1, rack_str.trim_end());
        }
    }


    pub fn draw(&mut self, n: usize, player: usize) {
        let set_indices: Vec<usize> = (0..100)
            .filter(|&idx| self.bag[idx])
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


    fn compute_score(&self, word_positions: &Vec<Vec<usize>>) -> u8 {
        let mut score = 0;
        
        for word_pos in word_positions {
            let mut word_score = 0;
            let mut word_mul = 1;

            for tile_pos in word_pos {
                let mut letter_score = TILE_SCORES[self.board[*tile_pos] as usize];

                match BOARD_MULTIPLIERS[*tile_pos] {
                    N => {},
                    DL => letter_score *= 2,
                    TL => letter_score *= 3,
                    DW => word_mul *= 2,
                    TW => word_mul *= 3,
                }

                word_score += letter_score;
            }

            score += word_score * word_mul;
        }

        score
    }


    pub fn play(&mut self, player: usize, play_tiles: BitField, pos_tiles: &[(usize, usize)]) -> Result<(), &'static str> {
        if self.racks[player] & play_tiles == play_tiles {
            let board_copy = self.board;

            for &(tile_pos, tile_idx) in pos_tiles.iter() {
                let tile_val = TILE_MAP[tile_idx];
                self.board[tile_pos] = tile_val;
            }

            let valid = self.validate(pos_tiles);

            if valid == true {
                self.racks[player] ^= play_tiles;
                if self.first_turn == true {
                    self.first_turn = false;
                }
                Ok(())
            }
            else {
                self.board = board_copy;
                Err("Board validation failed")
            }
        } else {
            Err("Invalid play mask")
        }
    }


    fn scan_word(&self, mut tile_pos: usize, direc: Direction) -> (Vec<u8>, Vec<usize>) {
        let mut word: Vec<u8> = Vec::new();
        let mut word_pos: Vec<usize> = Vec::new();
        
        match direc {
            Direction::Horizontal => {
                while tile_pos % 15 != 0 && self.board[tile_pos-1] != EMPTY {
                    tile_pos -= 1;
                }

                loop {
                    word.push(self.board[tile_pos]);
                    word_pos.push(tile_pos);
                    tile_pos += 1;
                    if tile_pos % 15 == 0 || self.board[tile_pos] == EMPTY {
                        break;
                    }
                }
            }

            Direction::Vertical => {
                while tile_pos > 14 && self.board[tile_pos-15] != EMPTY {
                    tile_pos -= 15;
                }

                loop {
                    word.push(self.board[tile_pos]);
                    word_pos.push(tile_pos);
                    tile_pos += 15;
                    if tile_pos >= 225 || self.board[tile_pos] == EMPTY {
                        break;
                    }
                }
            }
        }

        (word, word_pos)
    }


    fn all_words_valid(&self, words: &Vec<Vec<u8>>) -> bool {
        words.iter().all(|word| self.dictionary.contains(word))
    }


    fn validate(&self, pos_tiles: &[(usize, usize)]) -> bool {
        if self.first_turn == true && self.board[112] == EMPTY {
            println!("[ERR] - First move not played in the center!");
            return false;
        }

        if pos_tiles.is_empty() {
            println!("[ERR] - No tiles played!");
            return false;
        }

        let first_pos = pos_tiles[0].0;
        let target_row = first_pos / 15;
        let target_col = first_pos % 15;

        let same_row = pos_tiles.iter().all(|&(pos, _)| pos / 15 == target_row);
        let same_col = pos_tiles.iter().all(|&(pos, _)| pos % 15 == target_col);

        if !same_row && !same_col {
            println!("[ERR] - Tiles are not played in a line!");
            return false;
        }
        
        let mut words: Vec<Vec<u8>> = Vec::new();
        let mut words_pos: Vec<Vec<usize>> = Vec::new();
        let mut word: Vec<u8> = Vec::new();
        let mut word_pos: Vec<usize> = Vec::new();

        let mut start_pos = pos_tiles.iter().map(|&(p, _)| p).min().unwrap();
        let end_pos = pos_tiles.iter().map(|&(p, _)| p).max().unwrap();
        let step = if same_row {1} else {15};

        if (start_pos..end_pos).step_by(step).any(|p| self.board[p] == EMPTY) {
            println!("[ERR] - There are blank tiles between the tiles played!");
            return false;
        }

        if same_row {
            while start_pos % 15 != 0 && self.board[start_pos-1] != EMPTY {
                start_pos -= 1;
            }

            loop {
                word.push(self.board[start_pos]);
                word_pos.push(start_pos);
                if (start_pos >= 15 && self.board[start_pos-15] != EMPTY) || (start_pos < 210 && self.board[start_pos+15] != EMPTY) {
                    if pos_tiles.iter().any(|&(pos, _)| pos == start_pos) {
                        let (adj_word, adj_word_pos) = self.scan_word(start_pos, Direction::Vertical);
                        words.push(adj_word);
                        words_pos.push(adj_word_pos);
                    }
                }
                start_pos += 1;
                if start_pos % 15 == 0 || self.board[start_pos] == EMPTY {
                    break;
                }
            }
        } else if same_col {
            while start_pos > 14 && self.board[start_pos-15] != EMPTY {
                start_pos -= 15;
            }

            loop {
                word.push(self.board[start_pos]);
                word_pos.push(start_pos);
                if (start_pos % 15 != 0 && self.board[start_pos-1] != EMPTY) || (start_pos % 15 != 14 && self.board[start_pos+1] != EMPTY) {
                    if pos_tiles.iter().any(|&(pos, _)| pos == start_pos) {
                        let (adj_word, adj_word_pos) = self.scan_word(start_pos, Direction::Horizontal);
                        words.push(adj_word);
                        words_pos.push(adj_word_pos);
                    }
                }
                start_pos += 15;
                if start_pos >= 225 || self.board[start_pos] == EMPTY {
                    break;
                }
            }
        }

        words.push(word);
        words_pos.push(word_pos);

        if !self.all_words_valid(&words) {
            println!("[ERR] - Not all words are valid!");
            return false;
        }

        let is_connected = pos_tiles.iter().any(|&(pos, _)| {
            (pos >= 15 && self.board[pos-15] != EMPTY && pos_tiles.iter().all(|&(p, _)| p != pos-15)) ||
            (pos < 210 && self.board[pos+15] != EMPTY && pos_tiles.iter().all(|&(p, _)| p != pos+15)) ||
            (pos % 15 != 0 && self.board[pos-1] != EMPTY && pos_tiles.iter().all(|&(p, _)| p != pos-1)) ||
            (pos % 15 != 14 && self.board[pos+1] != EMPTY && pos_tiles.iter().all(|&(p, _)| p != pos+1))
        });

        if !self.first_turn && !is_connected {
            println!("[ERR] - Tiles are not connected to graph!");
            return false;
        }

        println!("play score: {}", self.compute_score(&words_pos));

        true
    }


    fn get_start_pos(&self, location: &str) -> Result<(bool, usize, usize), &'static str> {
        let mut chars = location.chars().peekable();
        if chars.peek().is_none() { 
            return Err("Empty location"); 
        }

        let is_vertical: bool;
        let row: usize;
        let col: usize;

        let first = *chars.peek().unwrap();
        
        if first.is_ascii_alphabetic() {
            is_vertical = true;
            col = (first.to_ascii_uppercase() as u8 - b'A') as usize;
            
            chars.next(); 
            let num_str: String = chars.collect();
            row = num_str.parse::<usize>().map_err(|_| "Invalid row")? - 1;
        } else if first.is_ascii_digit() {
            is_vertical = false;
            let mut num_str = String::new();
            
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            row = num_str.parse::<usize>().map_err(|_| "Invalid row")? - 1;
            let letter = chars.next().ok_or("Missing col")?;
            col = (letter.to_ascii_uppercase() as u8 - b'A') as usize;
        } else {
            return Err("Invalid location format");
        }

        return Ok((is_vertical, row, col));
    }


    pub fn play_notation(&mut self, player: usize, location: &str, word: &str) -> Result<(), &'static str> {
        let (is_vertical, row, col) = self.get_start_pos(location)?;

        if row > 14 || col > 14 { 
            return Err("Location out of bounds"); 
        }

        let mut current_pos = row * 15 + col;
        let step = if is_vertical { 15 } else { 1 };
        
        let mut mask = bitarr![u64, Lsb0; 0; 100];
        let mut pos_tiles = Vec::new();
        let rack = self.racks[player];

        for ch in word.to_ascii_uppercase().chars() {
            if current_pos >= 225 || (!is_vertical && current_pos / 15 != row) {
                return Err("Word exceeds board boundaries");
            }

            let letter_val = ch as u8 - b'A';
            
            if self.board[current_pos] != EMPTY {
                if self.board[current_pos] != letter_val {
                    return Err("Word conflicts with existing tiles on board");
                }
            } else {
                let mut found = false;
                for idx in 0..100 {
                    if rack[idx] && !mask[idx] && TILE_MAP[idx] == letter_val {
                        mask.set(idx, true);
                        pos_tiles.push((current_pos, idx));
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    return Err("Required tile not found in rack");
                }
            }
            current_pos += step;
        }

        self.play(player, mask, &pos_tiles)
    }
}
