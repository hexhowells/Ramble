#![allow(dead_code)]
mod engine;
mod trie;

use bitvec::prelude::*;
use std::io::{self, Write};

fn main() {
    let dictionary = trie::load_dictionary("data/bin/NWL2020.bin");
    let mut board = engine::Board::new(dictionary);
    
    board.draw(7, 0);
    board.draw(7, 1);

    println!("\nCommands:");
    println!("  exchange <player> <rack_idx1> <rack_idx2> ...");
    println!("  play <player> <loc> <word> (e.g., play 0 H8 GINGER for vertical, play 0 8H GINGER for horizontal)");
    println!("  play blank tiles by preceding the chosen tile with * (e.g. play 0 H8 GING*ER - to use the blank tile as E)");
    println!("  quit\n");

    loop {
        board.state();
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "quit" => break,
            "exchange" if parts.len() >= 2 => {
                let player: usize = parts[1].parse().unwrap_or(0);

                let result = parts[2..]
                    .iter()
                    .try_fold(bitarr![u64, Lsb0; 0; 100], |mut mask, s| {
                        let tile = s.parse::<char>().map_err(|_| "Expected single character")?;
                        let idx = board.get_tile_index(player, tile, &mask)?;
                        mask.set(idx, true);
                        Ok(mask)
                    })
                    .and_then(|mask| board.exchange(player, mask));
                
                if let Err(e) = result {
                    println!("Error: {}", e);
                }
            }
            "play" if parts.len() == 4 => {
                let player: usize = parts[1].parse().unwrap_or(0);
                let location = parts[2];
                let word = parts[3];
                
                if let Err(e) = board.play_notation(player, location, word) {
                    println!("Error: {}", e);
                }
                board.draw(7-board.racks[player].count_ones(), player);
            }
            _ => println!("Invalid command syntax. Use: play <player> <loc> <word>"),
        }
    }
}