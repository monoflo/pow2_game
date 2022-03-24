use rand::seq::SliceRandom;

use crate::{Cell, Move};

/// Defines the number of columns in the board.
const BOARD_COLS: usize = 4;
/// Defines the number or rows in the board.
const BOARD_ROWS: usize = 4;

#[derive(Copy, Clone, Debug)]
/// Represents a row and column on the game board.
pub struct Coord(usize, usize);

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
        let empty_coords = self.get_cells_by_emptiness(true).collect::<Vec<Coord>>();
        let chosen = empty_coords.choose(&mut rng);

        match chosen {
            Some(coord) => self.spawn_at(*coord),
            None => Err(()),
        }
    }

    /// Spawns a new cell on the game board at the specified location.
    ///
    /// # Arguments
    ///
    /// * `pos` - the grid coordinate at which to spawn
    fn spawn_at(&mut self, pos: Coord) -> Result<(), ()> {
        assert!(pos.0 < BOARD_ROWS);
        assert!(pos.1 < BOARD_COLS);
        self.grid[pos.0][pos.1].spawn()
    }

    /// Retrieves all cells matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    fn get_cells_by_emptiness(&self, is_empty: bool) -> impl Iterator<Item = Coord> + '_ {
        (0..BOARD_ROWS).flat_map(move |row| self.get_cells_by_emptiness_row(is_empty, row))
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
    ) -> impl Iterator<Item = Coord> + '_ {
        assert!(col < BOARD_COLS);
        (0..BOARD_ROWS)
            .filter(move |row| is_empty == self.grid[*row][col].is_empty())
            .map(move |row| Coord(row, col))
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
    ) -> impl Iterator<Item = Coord> + '_ {
        assert!(row < BOARD_ROWS);
        (0..BOARD_COLS)
            .filter(move |col| is_empty == self.grid[row][*col].is_empty())
            .map(move |col| Coord(row, col))
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

    /// Initializes a board with cells spawned at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `iter` - an `impl` of `IntoIterator` containing `Coord`s to spawn cells at
    fn setup_with_spawn_at(iter: impl IntoIterator<Item = Coord>) -> Board {
        let mut board = Board::empty();
        for coord in iter.into_iter() {
            board.spawn_at(coord).unwrap();
        }
        board
    }

    #[test]
    /// Affirm that `Board::empty()` initializes a board with all cells having `value = 0`.
    fn empty() {
        let board = Board::empty();
        assert!(board
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .all(|cell| cell.is_empty()));
    }

    #[test]
    #[should_panic]
    fn spawn_at_invalid_col() {
        let coords = vec![Coord(0, usize::MAX)];
        setup_with_spawn_at(coords);
    }

    #[test]
    #[should_panic]
    fn spawn_at_invalid_row() {
        let coords = vec![Coord(usize::MAX, 0)];
        setup_with_spawn_at(coords);
    }

    #[test]
    fn spawn_at_0_0() {
        let mut board = Board::empty();
        board.spawn_at(Coord(0, 0)).unwrap();
        let mut cells = board.grid.iter().flat_map(|row| row.iter());
        assert!(!cells.next().unwrap().is_empty());
        assert!(cells.all(|cell| cell.is_empty()));
    }

    #[test]
    fn spawn_at_end_end() {
        let mut board = Board::empty();
        board
            .spawn_at(Coord(BOARD_ROWS - 1, BOARD_COLS - 1))
            .unwrap();
        let mut cells = board.grid.iter().flat_map(|row| row.iter()).rev();
        assert!(!cells.next().unwrap().is_empty());
        assert!(cells.all(|cell| cell.is_empty()));
    }

    #[test]
    fn spawn_all() {
        let mut board = Board::empty();
        let NUM_CELLS = BOARD_ROWS.checked_mul(BOARD_COLS).unwrap();

        for _ in 0..NUM_CELLS {
            board.spawn().unwrap();
        }

        board.spawn().unwrap_err();
    }

    #[test]
    fn spawn_each() {
        let mut board = Board::empty();
        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                board.spawn_at(Coord(row, col)).unwrap();
            }
        }
        board.spawn().unwrap_err();
    }

    #[test]
    fn get_iter_of_empty_all_on_empty() {
        let board = Board::empty();
        let vec = board.get_cells_by_emptiness(true).collect::<Vec<Coord>>();
        assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
    }

    #[test]
    fn get_iter_of_empty_col_on_empty() {
        let board = Board::empty();
        for col in 0..BOARD_COLS {
            let vec = board
                .get_cells_by_emptiness_col(true, col)
                .collect::<Vec<Coord>>();
            assert_eq!(BOARD_COLS, vec.len());
        }
    }

    #[test]
    fn get_iter_of_empty_row_on_empty() {
        let board = Board::empty();
        for row in 0..BOARD_ROWS {
            let vec = board
                .get_cells_by_emptiness_row(true, row)
                .collect::<Vec<Coord>>();
            assert_eq!(BOARD_COLS, vec.len());
        }
    }
}
