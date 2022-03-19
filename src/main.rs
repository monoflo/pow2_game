mod board;
mod cell;
mod direction;

use board::Board;
use cell::Cell;
use direction::Direction;

fn main() {
    println!("BOARD TEST");
    let mut board = Board::new();
    board.shift(Direction::Up);
    board.shift(Direction::Down);
    board.shift(Direction::Left);
    board.shift(Direction::Right);

    board.spawn().unwrap();

    println!("{}", board.to_string());

    println!("CELL TEST");

    let mut cell = Cell::new();
    println!("empty: {}", cell.to_string());

    cell.spawn().unwrap();
    println!("spawn: {}", cell.to_string());

    cell.grow();
    println!("grow: {}", cell.to_string());

    cell.grow();
    println!("grow: {}", cell.to_string());
}
