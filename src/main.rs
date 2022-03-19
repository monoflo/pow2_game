mod board;
mod cell;
mod direction;

use board::Board;
use cell::Cell;
use direction::Direction;

fn main() {
    let mut board = Board::new();
    board.spawn().unwrap();

    print!("{}", board.to_string());
}
