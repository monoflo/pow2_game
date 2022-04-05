mod board;
mod cell;
mod movement;

use std::io::Write;

use board::Board;
use cell::Cell;
use movement::Move;

fn parse_input(inp: &str) -> Result<Move, ()> {
    match inp {
        "w" => Ok(Move::ShiftUp),
        "a" => Ok(Move::ShiftLeft),
        "s" => Ok(Move::ShiftDown),
        "d" => Ok(Move::ShiftRight),
        "u" => Ok(Move::Undo),
        _ => Err(()),
    }
}

fn main() {
    let mut board = Board::default();

    println!("{}", board);

    loop {
        let mut input = String::new();
        print!("move: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input.truncate(1);
        let input = input.to_lowercase();
        let mov: Move = parse_input(&input).expect("invalid move type");
        board.movement(mov).unwrap();
        println!("{}", board);
    }
}
