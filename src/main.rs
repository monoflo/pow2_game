use std::fmt;

const BOARD_HEIGHT: usize = 4;
const BOARD_WIDTH: usize = 4;

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    value: usize,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn grow(&mut self) {
        self.value <<= 1;
    }

    // TODO: make 4's spawn with chance 10%
    pub fn spawn(&mut self) {
        self.value = 2;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


struct Board {
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

// TODO: fix implementation
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for row in 0..self.grid.len() {
            // for col in 0..self.grid.len() {
                // write!(f, "{}", self.grid[row][col].value)
            // }
        // }

        write!(f, "{:?}", self.grid)
    }
}


fn main() {
    println!("BOARD TEST");
    let mut board = Board::new();
    board.shift(Direction::Up);
    board.shift(Direction::Down);
    board.shift(Direction::Left);
    board.shift(Direction::Right);

    println!("{}", board.to_string());

    println!("CELL TEST");

    let mut cell = Cell::new();
    println!("empty: {}", cell.to_string());

    cell.spawn();
    println!("spawn: {}", cell.to_string());

    cell.grow();
    println!("grow: {}", cell.to_string());

    cell.grow();
    println!("grow: {}", cell.to_string());
}
