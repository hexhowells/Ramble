#![allow(dead_code)]
mod engine;


fn main() {
    let mut board = engine::Board::new();

    board.draw(7, 0);
    board.state();

    board.draw(7, 1);
    board.state();
}
