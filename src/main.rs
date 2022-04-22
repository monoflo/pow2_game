mod board;
mod cell;
mod movement;

use std::io::Write;

use board::Board;
use cell::Cell;
use movement::{Direction, Move};

fn parse_input(inp: &str) -> Result<Move, ()> {
    match inp {
        "w" => Ok(Move::Shift(Direction::Up)),
        "a" => Ok(Move::Shift(Direction::Left)),
        "s" => Ok(Move::Shift(Direction::Down)),
        "d" => Ok(Move::Shift(Direction::Right)),
        "u" => Ok(Move::Undo),
        _ => Err(()),
    }
}

fn main() {
    let mut board = Board::new();

    loop {
        println!("{}\n", board);

        let mut mov: Option<Move> = None;
        while mov.is_none() {
            print!("move: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.truncate(1);
            let input = input.to_lowercase();

            match parse_input(&input) {
                Ok(m) => mov = Some(m),
                _ => println!("invalid input"),
            };
        }

        if let Err(()) = board.movement(mov.unwrap()) {
            println!("invalid move");
        }
        println!();
    }
}
