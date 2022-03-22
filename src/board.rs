use crate::{Cell, Coordinate, Direction};

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
        self.spawn_at(Coordinate { row: 0, col: 0 })
    }

    /// Spawns a new cell on the game board at the specified location.
    ///
    /// # Arguments
    ///
    /// * `pos` - the grid coordinate at which to spawn
    fn spawn_at(&mut self, pos: Coordinate) -> Result<(), ()> {
        assert!(pos.col < BOARD_COLS);
        assert!(pos.row < BOARD_ROWS);
        self.grid[pos.row][pos.col].spawn()
    }

    /// Retrieves all cells matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    fn get_cells_by_emptiness_board(
        &self,
        is_empty: bool,
    ) -> impl Iterator<Item = Coordinate> + '_ {
        (0..BOARD_ROWS)
            .zip(0..BOARD_COLS)
            .filter(move |pos| is_empty == self.grid[pos.0][pos.1].is_empty())
            .map(move |pos| Coordinate {
                row: pos.0,
                col: pos.1,
            })
    }

    /// Retrieves cells in the given col matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    /// * `col` - the board col in which to search
    fn get_cells_by_emptiness_col(
        &self,
        is_empty: bool,
        col: usize,
    ) -> impl Iterator<Item = Coordinate> + '_ {
        assert!(col < BOARD_COLS);
        (0..BOARD_ROWS)
            .filter(move |row| is_empty == self.grid[*row][col].is_empty())
            .map(move |row| Coordinate { row, col })
    }

    /// Retrieves cells in the given row matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    /// * `row` - the board row in which to search
    fn get_cells_by_emptiness_row(
        &self,
        is_empty: bool,
        row: usize,
    ) -> impl Iterator<Item = Coordinate> + '_ {
        assert!(row < BOARD_ROWS);
        (0..BOARD_COLS)
            .filter(move |col| is_empty == self.grid[row][*col].is_empty())
            .map(move |col| Coordinate { row, col })
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
