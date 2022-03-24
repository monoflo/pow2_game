mod board;
mod cell;
mod coordinate;
mod movement;

use std::io::Write;

use board::Board;
use cell::Cell;
use coordinate::Coordinate;
use movement::Move;

fn parse_input(inp: &str) -> Result<Move, ()> {
    return match inp {
        "w" => Ok(Move::Up),
        "a" => Ok(Move::Left),
        "s" => Ok(Move::Down),
        "d" => Ok(Move::Right),
        "u" => Ok(Move::Undo),
        _ => Err(()),
    };
}

fn main() {
    let mut input = String::new();
    let mut board = Board::new();

    println!("{}", board.to_string());

    loop {
        print!("move: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input);
        input.truncate(1);
        let mut input = input.to_lowercase();
        let mov: Move = parse_input(&input).expect("invalid move type");
        let mut input = String::new();
        board.movement(mov);
        println!("{}", board.to_string());
    }
}
