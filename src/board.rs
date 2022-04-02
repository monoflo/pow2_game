use rand::seq::SliceRandom;

use crate::{Cell, Move};

/// Defines the number of columns in the board.
const BOARD_COLS: usize = 4;
/// Defines the number or rows in the board.
const BOARD_ROWS: usize = 4;

/// Represents a row and column on the game board.
#[derive(Copy, Clone, Debug)]
pub struct Coord(usize, usize);

/// The representation of a game board.
pub struct Board {
    /// The grid containing the cells of the board
    grid: [[Cell; BOARD_COLS]; BOARD_ROWS],
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

impl Board {
    /// Returns a new, empty instance of a game board.
    fn empty() -> Self {
        Self {
            grid: [[Cell::new(); BOARD_COLS]; BOARD_ROWS],
        }
    }
}

/// Affirm that `Board::empty()` initializes a board with all cells having `value = 0`.
#[test]
fn test_empty() {
    let board = Board::empty();
    assert!(board
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .all(|cell| cell.is_empty()));
}

impl Board {
    /// Returns a new instance of a game board.
    pub fn new() -> Self {
        let mut inst = Board::empty();
        inst.spawn()
            .expect("failed to spawn a cell on the empty board");
        inst
    }
}

/// Affirm that `Board::new()` initializes a board with all cells having `value = 0`,
/// except one.
#[test]
fn test_new() {
    let board = Board::new();
    let mut found = false;
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let is_empty = board.grid[row][col].is_empty();
            assert!(is_empty || !found);
            found = found || !is_empty;
        }
    }
}

impl Board {
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
}

/// Affirm that `Board::get_cells_by_emptiness_col()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_col_board_empty() {
    let board = Board::empty();
    for col in 0..BOARD_COLS {
        let vec = board
            .get_cells_by_emptiness_col(true, col)
            .collect::<Vec<Coord>>();
        assert_eq!(BOARD_COLS, vec.len());
    }
}

impl Board {
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

/// Affirm that `Board::get_cells_by_emptiness_row()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_row_board_empty() {
    let board = Board::empty();
    for row in 0..BOARD_ROWS {
        let vec = board
            .get_cells_by_emptiness_row(true, row)
            .collect::<Vec<Coord>>();
        assert_eq!(BOARD_COLS, vec.len());
    }
}

impl Board {
    /// Retrieves all cells matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    fn get_cells_by_emptiness(&self, is_empty: bool) -> impl Iterator<Item = Coord> + '_ {
        (0..BOARD_ROWS).flat_map(move |row| self.get_cells_by_emptiness_row(is_empty, row))
    }
}

/// Affirm that `Board::get_cells_by_emptiness()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_board_empty() {
    let board = Board::empty();
    let vec = board.get_cells_by_emptiness(true).collect::<Vec<Coord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
}

impl Board {
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
}

/// Affirm that each cell on the board can be spawned-at sequentially until the board is full
/// and then no more.
#[test]
fn test_spawn_at_exhaustive() {
    let mut board = Board::empty();
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            board.spawn_at(Coord(row, col)).unwrap();
        }
    }
    board.spawn().unwrap_err();
}

/// Affirm that only the bottom-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_bottom_right() {
    let mut board = Board::empty();
    board.spawn_at_many(vec![Coord(BOARD_ROWS - 1, BOARD_COLS - 1)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter()).rev();
    assert!(!cells.next().unwrap().is_empty());
    assert!(cells.all(|cell| cell.is_empty()));
}

/// Affirm that only the bottom-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_bottom_left() {
    let mut board = Board::empty();
    board.spawn_at_many(vec![Coord(BOARD_ROWS - 1, 0)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    for _ in 0..BOARD_ROWS - 1 {
        for _ in 0..BOARD_COLS {
            assert!(cells.next().unwrap().is_empty());
        }
    }
    assert!(!cells.next().unwrap().is_empty());
    assert!(cells.all(|cell| cell.is_empty()));
}

/// Affirm that only the top-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_top_left() {
    let mut board = Board::empty();
    board.spawn_at_many(vec![Coord(0, 0)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    assert!(!cells.next().unwrap().is_empty());
    assert!(cells.all(|cell| cell.is_empty()));
}

/// Affirm that only the top-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_top_right() {
    let mut board = Board::empty();
    board.spawn_at_many(vec![Coord(0, BOARD_COLS - 1)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    for _ in 0..BOARD_COLS - 1 {
        assert!(cells.next().unwrap().is_empty());
    }
    assert!(!cells.next().unwrap().is_empty());
    assert!(cells.all(|cell| cell.is_empty()));
}

/// Affirm that attempting to spawn in an invalid column will fail.
#[test]
#[should_panic]
fn test_spawn_at_invalid_col() {
    let col = usize::MAX;
    assert!(col < BOARD_COLS);
    let mut board = Board::empty();
    board.spawn_at(Coord(0, col)).unwrap_err();
}

/// Affirm that attempting to spawn in an invalid row will fail.
#[test]
#[should_panic]
fn test_spawn_at_invalid_row() {
    let row = usize::MAX;
    assert!(row < BOARD_ROWS);
    let mut board = Board::empty();
    board.spawn_at(Coord(row, 0)).unwrap_err();
}

impl Board {
    /// Spawns new cells on the game board at the specified location(s).
    ///
    /// # Arguments
    ///
    /// * `iter` - an `impl IntoIterator` of `Coord`s to spawn cells at
    fn spawn_at_many(&mut self, iter: impl IntoIterator<Item = Coord>) -> Result<(), ()> {
        for coord in iter.into_iter() {
            self.spawn_at(coord)?;
        }
        Ok(())
    }
}

impl Board {
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
}

/// Affirm that each cell on the board can be spawned-at randomly until the board is full and
/// then no more.
#[test]
fn test_spawn_exhaustive() {
    let mut board = Board::empty();
    let num_cells = BOARD_ROWS.checked_mul(BOARD_COLS).unwrap();

    for _ in 0..num_cells {
        board.spawn().unwrap();
    }

    board.spawn().unwrap_err();
}

impl Board {
    /// Handles movement on the game board.
    ///
    /// # Arguments
    ///
    /// * `mov` - the movement type to handle
    pub fn movement(&mut self, mov: Move) -> Result<(), ()> {
        match mov {
            Move::ShiftDown => self.shift_vertical(mov),
            Move::ShiftLeft => self.shift_horizontal(mov),
            Move::ShiftRight => self.shift_horizontal(mov),
            Move::ShiftUp => self.shift_vertical(mov),
            Move::Undo => todo!("undo move"),
        }
    }
}

impl Board {
    fn shift_horizontal(&mut self, dir: Move) -> Result<(), ()> {
        const VALID: [Move; 2] = [Move::ShiftLeft, Move::ShiftRight];

        if !VALID.contains(&dir) {
            return Err(());
        }

        todo!("horizontal shifts have not been implemented");
    }
}

impl Board {
    fn shift_vertical(&mut self, dir: Move) -> Result<(), ()> {
        const VALID: [Move; 2] = [Move::ShiftUp, Move::ShiftDown];

        if !VALID.contains(&dir) {
            return Err(());
        }

        todo!("vertical shifts have not been implemented");
    }
}
