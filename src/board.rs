use crate::{Cell, Direction};

/// Defines the height of the board.
const BOARD_HEIGHT: usize = 4;
/// Defines the width of the board.
const BOARD_WIDTH: usize = 4;

/// The representation of a game board.
pub struct Board {
    /// The grid containing the cells of the board
    grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    /// Returns a new, empty instance of a game board.
    pub fn new() -> Self {
        Self {
            grid: [[Cell::new(); BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    /// Handles movement on the game board.
    ///
    /// # Arguments
    ///
    /// * `dir` - the direction in which to shift the game board
    pub fn shift(&mut self, dir: Direction) {
        // TODO: implement
        match dir {
            Direction::Down => println!("down"),
            Direction::Left => println!("left"),
            Direction::Right => println!("right"),
            Direction::Up => println!("up"),
        }
    }

    /// Spawns a new cell on the game board.
    pub fn spawn(&mut self) -> Result<(), ()> {
        // TODO: implement
        self.spawn_at(0, 0)
    }

    /// Spawns a new cell on the game board at the specified location.
    ///
    /// # Arguments
    ///
    /// * `col` - the grid column at which to spawn
    /// * `row` - the grid row at which to spawn
    fn spawn_at(&mut self, col: usize, row: usize) -> Result<(), ()> {
        assert!(col < BOARD_HEIGHT);
        assert!(row < BOARD_WIDTH);
        self.grid[row][col].spawn()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in self.grid {
            result += &row
                .iter()
                .map(|cell| cell.to_string())
                .reduce(|a, b| a + " " + &b)
                .unwrap();

            result += "\n";
        }

        write!(f, "{}", result)
    }
}
