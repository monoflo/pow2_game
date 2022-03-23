use rand::Rng;

use crate::{Cell, Coordinate, Move};

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
    fn empty() -> Self {
        Self {
            grid: [[Cell::new(); BOARD_COLS]; BOARD_ROWS],
        }
    }

    /// Returns a new instance of a game board.
    pub fn new() -> Self {
        let mut inst = Board::empty();
        inst.spawn();
        inst
    }

    /// Handles movement on the game board.
    ///
    /// # Arguments
    ///
    /// * `mov` - the movement type to handle
    pub fn movement(&mut self, mov: Move) -> Result<(), ()> {
        return match mov {
            Move::Down => todo!(),
            Move::Left => todo!(),
            Move::Right => todo!(),
            Move::Up => todo!(),
            Move::Undo => todo!(),
        };
    }

    /// Spawns a new cell on the game board.
    fn spawn(&mut self) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        let r: usize = rng.gen_range(0..BOARD_ROWS);
        let c: usize = rng.gen_range(0..BOARD_COLS);
        self.spawn_at(Coordinate { row: r, col: c })
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

    #[test]
    #[should_panic]
    fn spawn_at_invalid_col() {
        let mut board = Board::new();
        board.spawn_at(Coordinate {
            row: 0,
            col: usize::MAX,
        });
    }

    #[test]
    #[should_panic]
    fn spawn_at_invalid_row() {
        let mut board = Board::new();
        board.spawn_at(Coordinate {
            row: usize::MAX,
            col: 0,
        });
    }

    #[test]
    fn spawn_at_0_0() {
        let mut board = Board::new();
        board.spawn_at(Coordinate { row: 0, col: 0 }).unwrap();
        let mut cells = board.grid.iter().flat_map(|row| row.iter());
        assert!(!cells.next().unwrap().is_empty());
        assert!(cells.all(|cell| cell.is_empty()));
    }

    #[test]
    fn spawn_at_end_end() {
        let mut board = Board::new();
        board
            .spawn_at(Coordinate {
                row: BOARD_ROWS - 1,
                col: BOARD_COLS - 1,
            })
            .unwrap();
        let mut cells = board.grid.iter().flat_map(|row| row.iter()).rev();
        assert!(!cells.next().unwrap().is_empty());
        assert!(cells.all(|cell| cell.is_empty()));
    }
}
