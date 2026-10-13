#![allow(dead_code)]
mod engine;

use bitvec::prelude::*;


fn main() {
    let mut board = engine::Board::new();

    board.draw(7, 0);
    board.draw(7, 1);
    board.state();

    let mut mask = bitarr![u64, Lsb0; 0; 100];
    let indices: Vec<usize> = board.racks[0].iter_ones().collect();

    for i in indices {
        mask.set(i, true);
    }

    println!("mask: \t {:b}", mask);

    let _ = board.exchange(0, mask);

    board.state();
}