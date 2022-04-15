use rand::seq::SliceRandom;

use crate::{Cell, Move};

/// Defines the number of columns in the board.
const BOARD_COLS: usize = 4;
/// Defines the number or rows in the board.
const BOARD_ROWS: usize = 4;

/// Represents a row and column on the game board.
#[derive(Copy, Clone)]
pub struct Coord(usize, usize);

/// The representation of a game board.
#[derive(Default)]
pub struct Board {
    /// The grid containing the cells of the board.
    grid: [[Option<Cell>; BOARD_COLS]; BOARD_ROWS],
}

/// Affirm that the default board is instantiated with `None` in each cell.
#[test]
fn test_default() {
    assert!(Board::default()
        .grid
        .iter()
        .flatten()
        .all(|cell| cell.is_none()));
}

/// Implementation of the `Display` trait for `Board`.
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row
                    .iter()
                    .map(|cell| match cell {
                        Some(c) => c.to_string(),
                        None => 0.to_string(),
                    })
                    .reduce(|a, b| a + " " + &b)
                    .unwrap())
                .reduce(|a, b| a + "\n" + &b)
                .unwrap()
        )
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
            .filter(move |row| is_empty == self.grid[*row][col].is_none())
            .map(move |row| Coord(row, col))
    }
}

/// Affirm that `Board::get_cells_by_emptiness_col()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_col_board_empty() {
    let board = Board::default();
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
            .filter(move |col| is_empty == self.grid[row][*col].is_none())
            .map(move |col| Coord(row, col))
    }
}

/// Affirm that `Board::get_cells_by_emptiness_row()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_row_board_empty() {
    let board = Board::default();
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
    let board = Board::default();
    let vec = board.get_cells_by_emptiness(true).collect::<Vec<Coord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
}

impl Board {
    /// Attempts to spawn a new cell on the game board at the specified location.
    ///
    /// # Arguments
    ///
    /// * `pos` - the grid coordinate at which to spawn
    fn spawn_at(&mut self, pos: Coord) -> Result<(), ()> {
        assert!(pos.0 < BOARD_ROWS);
        assert!(pos.1 < BOARD_COLS);

        let mut gridpos = &mut self.grid[pos.0][pos.1];

        if None == *gridpos {
            *gridpos = Some(Cell::default());
            return Ok(());
        }

        Err(())
    }
}

/// Affirm that each cell on the board can be spawned-at sequentially until the board is full
/// and then no more.
#[test]
fn test_spawn_at_exhaustive() {
    let mut board = Board::default();
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
    let mut board = Board::default();
    board.spawn_at_many(vec![Coord(BOARD_ROWS - 1, BOARD_COLS - 1)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter()).rev();
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the bottom-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_bottom_left() {
    let mut board = Board::default();
    board.spawn_at_many(vec![Coord(BOARD_ROWS - 1, 0)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    for _ in 0..BOARD_ROWS - 1 {
        for _ in 0..BOARD_COLS {
            assert!(cells.next().unwrap().is_none());
        }
    }
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the top-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_top_left() {
    let mut board = Board::default();
    board.spawn_at_many(vec![Coord(0, 0)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the top-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_top_right() {
    let mut board = Board::default();
    board.spawn_at_many(vec![Coord(0, BOARD_COLS - 1)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    for _ in 0..BOARD_COLS - 1 {
        assert!(cells.next().unwrap().is_none());
    }
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that attempting to spawn in an invalid column will fail.
#[test]
#[should_panic]
fn test_spawn_at_invalid_col() {
    let col = usize::MAX;
    assert!(col < BOARD_COLS);
    let mut board = Board::default();
    board.spawn_at(Coord(0, col)).unwrap_err();
}

/// Affirm that attempting to spawn in an invalid row will fail.
#[test]
#[should_panic]
fn test_spawn_at_invalid_row() {
    let row = usize::MAX;
    assert!(row < BOARD_ROWS);
    let mut board = Board::default();
    board.spawn_at(Coord(row, 0)).unwrap_err();
}

impl Board {
    #[allow(dead_code)]
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
    /// Randomly spawns a new cell on the game board.
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
    let mut board = Board::default();
    let num_cells = BOARD_ROWS.checked_mul(BOARD_COLS).unwrap();

    for _ in 0..num_cells {
        board.spawn().unwrap();
    }

    board.spawn().unwrap_err();
}

impl Board {
    /// Returns a new instance of a game board.
    pub fn new() -> Self {
        let mut inst = Board::default();
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
            let is_empty = board.grid[row][col].is_none();
            assert!(is_empty || !found);
            found = found || !is_empty;
        }
    }
}

impl Board {
    /// Attempts to shift each cell over to the end of the vector.
    ///
    /// # Arguments
    /// * `vec` - the vector of cells to shift
    fn shift(vec: &mut Vec<Option<Cell>>) -> Result<(), ()> {
        /*
         * some test code:
        for idx in (vec.len() - 1)..1 {
            if let Some(mut cell) = &vec[idx] {
                if let Some(mut other) = &vec[idx-1] {
                    cell.merge(other);
                }
            }
        }

        let mut iter = cells.iter().rev();
        for (idx, cell) in iter.enumerate() {
            if let Some(mut c) = cell {
                if let Some(mut n) = cells.get_mut(idx+1) {
                    if n.is_none() {
                    }
                }
            }
        }
        */
        Ok(())
    }
}

/// Affirm that `Board::shift` will fail on a collection of empty cells.
#[test]
fn test_shift_0_0_0_0() {
    let mut row: Vec<Option<Cell>> = vec![None, None, None, None];
    Board::shift(&mut row).unwrap_err();
}

/// Affirm that `Board::shift` will fail on a collection where only one cell is at the end, as the
/// collection did not shift.
#[test]
fn test_shift_0_0_0_2() {
    let mut row: Vec<Option<Cell>> = vec![None, None, None, Some(Cell::new(2))];
    Board::shift(&mut row).unwrap_err();
}

/// Affirm that `Board::shift` will successfully push one cell from the start to the end of the
/// collection.
#[test]
fn test_shift_2_0_0_0() {
    let mut row: Vec<Option<Cell>> = vec![Some(Cell::new(2)), None, None, None];
    Board::shift(&mut row).unwrap();
    assert_eq!(vec![None, None, None, Some(Cell::new(2))], row);
}

/// Affirm that `Board::shift` will successfully merge two cells at the end of the collection.
#[test]
fn test_shift_0_0_2_2() {
    let mut row: Vec<Option<Cell>> = vec![None, None, Some(Cell::new(2)), Some(Cell::new(2))];
    Board::shift(&mut row).unwrap();
    assert_eq!(vec![None, None, None, Some(Cell::new(4))], row);
}

/// Affirm that `Board::shift` will fail on a collection where none of the cells are equal in
/// value, as a shift cannot be performed.
#[test]
fn test_shift_2_4_8_16() {
    let mut row: Vec<Option<Cell>> = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(8)),
        Some(Cell::new(16)),
    ];
    Board::shift(&mut row).unwrap_err();
}

/// Affirm that `Board::shift` will fail on a collection where none of the cells are adjacent to
/// cells with like values, as no merging or shifting can be performed.
#[test]
fn test_shift_2_4_2_4() {
    let mut row: Vec<Option<Cell>> = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
        Some(Cell::new(4)),
    ];
    Board::shift(&mut row).unwrap_err();
}

impl Board {
    /// Handles movement on the game board.
    ///
    /// # Arguments
    ///
    /// * `mov` - the movement type to handle
    pub fn movement(&mut self, mov: Move) -> Result<(), ()> {
        // TODO: replace above `shift_{horizontal,vertical}` with common shift function,
        // handle directionality in this function
        match mov {
            Move::ShiftDown => todo!("shift down"), // self.shift_vertical(mov),
            Move::ShiftLeft => todo!("shift left"), // self.shift_horizontal(mov),
            Move::ShiftRight => todo!("shift right"), // self.shift_horizontal(mov),
            Move::ShiftUp => todo!("shift up"),     // self.shift_vertical(mov),
            Move::Undo => todo!("undo move"),
        }
    }
}
