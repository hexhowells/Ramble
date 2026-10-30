#![allow(dead_code)]
mod engine;

use bitvec::prelude::*;
use std::io::{self, Write};

fn main() {
    let mut board = engine::Board::new();
    
    board.draw(7, 0);
    board.draw(7, 1);

    println!("Commands:");
    println!("  exchange <player> <rack_idx1> <rack_idx2> ...");
    println!("  play <player> <board_pos>,<rack_idx> <board_pos>,<rack_idx> ...");
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
                let mut mask = bitarr![u64, Lsb0; 0; 100];
                
                for idx_str in &parts[2..] {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        mask.set(idx, true);
                    }
                }
                
                if let Err(e) = board.exchange(player, mask) {
                    println!("Error: {}", e);
                }
            }
            "play" if parts.len() >= 3 => {
                let player: usize = parts[1].parse().unwrap_or(0);
                let mut mask = bitarr![u64, Lsb0; 0; 100];
                let mut pos_tiles = Vec::new();
                
                for pair in &parts[2..] {
                    let split: Vec<&str> = pair.split(',').collect();
                    if split.len() == 2 {
                        if let (Ok(pos), Ok(idx)) = (split[0].parse::<usize>(), split[1].parse::<usize>()) {
                            mask.set(idx, true);
                            pos_tiles.push((pos, idx));
                        }
                    }
                }
                
                if let Err(e) = board.play(player, mask, &pos_tiles) {
                    println!("Error: {}", e);
                }
            }
            _ => println!("Invalid command syntax."),
        }
    }
}