mod board;
mod cell;
mod coordinate;
mod movement;

use board::Board;
use cell::Cell;
use coordinate::Coordinate;
use movement::Move;

fn main() {
    let mut board = Board::new();
    board.spawn().unwrap();

    print!("{}", board.to_string());
}
