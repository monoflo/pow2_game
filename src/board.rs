use crate::{Cell,Direction};

const BOARD_HEIGHT: usize = 4;
const BOARD_WIDTH: usize = 4;


pub struct Board {
    grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::new(); BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    // TODO: implement
    pub fn shift(&mut self, dir: Direction) {
        match dir {
            Direction::Down => println!("down"),
            Direction::Left => println!("left"),
            Direction::Right => println!("right"),
            Direction::Up => println!("up"),
        }
    }

    // TODO: implement
    pub fn spawn(&mut self) {
        todo!();
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in self.grid {
            result += &row
                .iter()
                .map(|cell| cell.to_string())
                .reduce(|a, b| {a + " " + &b})
                .unwrap();

            result += "\n";
        }

        write!(f, "{}", result)
    }
}
