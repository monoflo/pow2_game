use crate::{Cell, Direction};

/// Defines the number of columns in the board.
const BOARD_COLS: usize = 4;
/// Defines the number or rows in the board.
const BOARD_ROWS: usize = 4;

/// The representation of a game board.
pub struct Board {
    /// The grid containing the cells of the board
    grid: [[Cell; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    /// Returns a new, empty instance of a game board.
    pub fn new() -> Self {
        Self {
            grid: [[Cell::new(); BOARD_COLS]; BOARD_ROWS],
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
            Direction::Down => todo!(),
            Direction::Left => todo!(),
            Direction::Right => todo!(),
            Direction::Up => todo!(),
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
        assert!(col < BOARD_COLS);
        assert!(row < BOARD_ROWS);
        self.grid[row][col].spawn()
    }

    fn get_empty_cells_col(&self, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        assert!(col < BOARD_COLS);
        (0..BOARD_ROWS)
            .filter(move |row| self.grid[*row][col].is_empty())
            .map(move |row| (row, col))
    }

    fn get_empty_cells_row(&self, row: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        assert!(row < BOARD_ROWS);
        (0..BOARD_COLS)
            .filter(move |col| self.grid[row][*col].is_empty())
            .map(move |col| (row, col))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Affirm that `Board::new()` initializes a board with all cells having `value = 0`.
    fn new() {
        let board = Board::new();
        assert!(board
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .all(|cell| cell.is_empty()));
    }
}
