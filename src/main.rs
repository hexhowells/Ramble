#![allow(dead_code)]
mod engine;


fn main() {
    let mut board = engine::Board::new();

    board.draw(7);

    board.state();
}
