use rand::seq::SliceRandom;

use crate::{Cell, Direction, Move};

/// Defines the number of columns in the board.
const BOARD_COLS: usize = 4;
/// Defines the number or rows in the board.
const BOARD_ROWS: usize = 4;
/// Defines the maximum number of undos the player can perform.
const HISTORY_SIZE: usize = 1;

/// Type representing a cell on the board.
type BoardCell = Option<Cell>;
/// Type representing the grid of cells on the board.
type BoardGrid = [[BoardCell; BOARD_COLS]; BOARD_ROWS];

/// Type representing a `BoardGrid` position (i.e. row, column indices).
type BoardCoord = (usize, usize);

/// The representation of a game board.
pub struct Board {
    /// The grid containing the cells of the board.
    grid: BoardGrid,
    /// The saved, past states of the board that can be .
    history: Vec<BoardGrid>,
}

/// Implementation of the `Default` trait for `Board`.
impl Default for Board {
    /// Create an empty grid and an empty, bound-vector of grid states.
    fn default() -> Self {
        Self {
            grid: Default::default(),
            history: Vec::with_capacity(HISTORY_SIZE),
        }
    }
}

/// Affirm that the default board is instantiated with `None` in each cell.
#[test]
fn test_default_grid() {
    assert!(Board::default()
        .grid
        .iter()
        .flatten()
        .all(|cell| cell.is_none()));
}

/// Affirm that the default board is instantiated with empty history, containing the maximum
/// capacity specified by `HISTORY_SIZE`.
#[test]
fn test_default_history() {
    let board = Board::default();
    assert_eq!(HISTORY_SIZE, board.history.capacity());
    assert!(board.history.is_empty());
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
    ) -> impl Iterator<Item = BoardCoord> + '_ {
        assert!(col < BOARD_COLS);
        (0..BOARD_ROWS)
            .filter(move |row| is_empty == self.grid[*row][col].is_none())
            .map(move |row| (row, col))
    }
}

/// Affirm that `Board::get_cells_by_emptiness_col()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_col_board_empty() {
    let board = Board::default();
    for col in 0..BOARD_COLS {
        let vec = board
            .get_cells_by_emptiness_col(true, col)
            .collect::<Vec<BoardCoord>>();
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
    ) -> impl Iterator<Item = BoardCoord> + '_ {
        assert!(row < BOARD_ROWS);
        (0..BOARD_COLS)
            .filter(move |col| is_empty == self.grid[row][*col].is_none())
            .map(move |col| (row, col))
    }
}

/// Affirm that `Board::get_cells_by_emptiness_row()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_row_board_empty() {
    let board = Board::default();
    for row in 0..BOARD_ROWS {
        let vec = board
            .get_cells_by_emptiness_row(true, row)
            .collect::<Vec<BoardCoord>>();
        assert_eq!(BOARD_COLS, vec.len());
    }
}

impl Board {
    /// Retrieves all cells matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    fn get_cells_by_emptiness(&self, is_empty: bool) -> impl Iterator<Item = BoardCoord> + '_ {
        (0..BOARD_ROWS).flat_map(move |row| self.get_cells_by_emptiness_row(is_empty, row))
    }
}

/// Affirm that `Board::get_cells_by_emptiness()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_board_empty() {
    let board = Board::default();
    let vec = board
        .get_cells_by_emptiness(true)
        .collect::<Vec<BoardCoord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
}

impl Board {
    /// Attempts to spawn a new cell on the game board at the specified location.
    ///
    /// # Arguments
    ///
    /// * `pos` - the grid coordinate at which to spawn
    fn spawn_at(&mut self, pos: BoardCoord) -> Result<(), ()> {
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
            board.spawn_at((row, col)).unwrap();
        }
    }
    board.spawn().unwrap_err();
}

/// Affirm that only the bottom-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_bottom_right() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(BOARD_ROWS - 1, BOARD_COLS - 1)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter()).rev();
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the bottom-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_bottom_left() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(BOARD_ROWS - 1, 0)]);
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
    board.spawn_at_many(vec![(0, 0)]);
    let mut cells = board.grid.iter().flat_map(|row| row.iter());
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the top-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn test_spawn_at_corner_top_right() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(0, BOARD_COLS - 1)]);
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
    board.spawn_at((0, col)).unwrap_err();
}

/// Affirm that attempting to spawn in an invalid row will fail.
#[test]
#[should_panic]
fn test_spawn_at_invalid_row() {
    let row = usize::MAX;
    assert!(row < BOARD_ROWS);
    let mut board = Board::default();
    board.spawn_at((row, 0)).unwrap_err();
}

impl Board {
    #[allow(dead_code)]
    /// Spawns new cells on the game board at the specified location(s).
    ///
    /// # Arguments
    ///
    /// * `iter` - an `impl IntoIterator` of `BoardCoord`s to spawn cells at
    fn spawn_at_many(&mut self, iter: impl IntoIterator<Item = BoardCoord>) -> Result<(), ()> {
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
        let empty_coords = self
            .get_cells_by_emptiness(true)
            .collect::<Vec<BoardCoord>>();
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
    fn get_mergeable<'a>(cells: impl IntoIterator<Item = &'a BoardCell>) -> Vec<(usize, usize)> {
        struct RefCell {
            index: Option<usize>,
            value: Option<usize>,
        }

        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut rc = RefCell {
            index: None,
            value: None,
        };

        let mut iter = cells.into_iter().enumerate();

        for (idx, cell_opt) in iter {
            if let Some(cc) = cell_opt {
                let val = cc.value();

                if rc.index.is_some() && rc.value.is_some() && val == rc.value.unwrap() {
                    result.push((rc.index.unwrap(), idx));
                    rc.index = None;
                    rc.value = None;
                } else {
                    rc.index = Some(idx);
                    rc.value = Some(val);
                }
            }
        }
        result
    }
}

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for an empty collection.
#[test]
fn test_get_mergeable_0_0_0_0() {
    let cells = vec![None, None, None, None];
    assert!(Board::get_mergeable(&cells).is_empty());
}

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for a singular non-empty cell.
#[test]
fn test_get_mergeable_0_0_0_2() {
    let cells = vec![None, None, None, Some(Cell::new(2))];
    assert!(Board::get_mergeable(&cells).is_empty());
}

/// Affirm that `Board::get_mergeable` reports one mergeable pair for a contiguous pair of
/// same-valued cells.
#[test]
fn test_get_mergeable_0_0_2_2() {
    let expect = vec![(2, 3)];
    let cells = vec![None, None, Some(Cell::new(2)), Some(Cell::new(2))];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

/// Affirm that `Board::get_mergeable` reports one mergeable pair for a pair of same-valued cells
/// separated by an empty cell.
#[test]
fn test_get_mergeable_0_2_0_2() {
    let expect = vec![(1, 3)];
    let cells = vec![None, Some(Cell::new(2)), None, Some(Cell::new(2))];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for a pair of same-valued cells
/// separated by a cell of dissimilar value to the others.
#[test]
fn test_get_mergeable_0_2_4_2() {
    let cells = vec![
        None,
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
    ];
    assert!(Board::get_mergeable(&cells).is_empty());
}

#[test]
fn test_get_mergeable_2_2_2_2() {
    let expect = vec![(0, 1), (2, 3)];
    let cells = vec![
        Some(Cell::new(2)),
        Some(Cell::new(2)),
        Some(Cell::new(2)),
        Some(Cell::new(2)),
    ];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

#[test]
fn test_get_mergeable_2_4_4_2() {
    let expect = vec![(1, 2)];
    let cells = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
    ];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

impl Board {
    /// Attempts to shift each cell over to the beginning of the vector.
    ///
    /// # Arguments
    /// * `cells` - a group of `BoardCell`s that can be
    ///
    /// # Returns
    /// * `None` - neither a shift or merge was able to be performed on the group
    /// * `Some(Vec<BoardCell>)` - otherwise
    fn shift_group(cells: impl IntoIterator<Item = BoardCell>) -> Option<Vec<BoardCell>> {
        let mut result = cells.into_iter().collect::<Vec<BoardCell>>();
        let mergeable = Board::get_mergeable(result.iter());
        let mut valid = !mergeable.is_empty();

        /* merge pairs */

        // foo bar baz

        for pair in mergeable {
            let (ls, rs) = result.split_at_mut(pair.1);
            let mergee = ls[pair.0].as_mut().unwrap();
            let merger = rs[0].as_mut().unwrap();

            mergee.merge(merger).unwrap();
            rs[0] = None; // after merge, `merger` is dropped, so set to `None`
        }

        /* shift cells */

        let mut swpidx: Option<usize> = None;

        for idx in 0..result.len() {
            match (swpidx.is_some(), result[idx].is_some()) {
                // if `swpidx` isn't set and value is `None`, set the `swpidx`
                (false, false) => {
                    swpidx = Some(idx);
                }
                // if `swpidx` is set and value is `Some(...)`, perform swap
                (true, true) => {
                    result.swap(swpidx.unwrap(), idx);
                    swpidx = None;
                    valid = true;
                }
                _ => {}
            }
        }

        valid.then(|| result)
    }
}

/// Affirm that `Board::shift_group` will return nothing on a group of empty cells.
#[test]
fn test_shift_group_0_0_0_0() {
    let mut row: Vec<BoardCell> = vec![None, None, None, None];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will return nothing on a group containing one cell positioned at the
/// start.
#[test]
fn test_shift_group_2_0_0_0() {
    let row = vec![Some(Cell::new(2)), None, None, None];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will successfully shift a group containing one cell positioned at
/// the end to the start.
#[test]
fn test_shift_group_0_0_0_2() {
    let mut row = vec![None, None, None, Some(Cell::new(2))];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(2)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

/// Affirm that `Board::shift_group` will successfully merge two cells at the start of the group.
#[test]
fn test_shift_group_2_2_0_0() {
    let mut row = vec![Some(Cell::new(2)), Some(Cell::new(2)), None, None];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(4)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

/// Affirm that `Board::shift_group` will return nothing on a group where none of the cells have like
/// values, as neither a shift or merge can be performed.
#[test]
fn test_shift_group_2_4_8_16() {
    let mut row = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(8)),
        Some(Cell::new(16)),
    ];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will return nothing on a group where none of the cells are adjacent to
/// cells with like values, as no merging or shifting can be performed.
#[test]
fn test_shift_group_2_4_2_4() {
    let mut row = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
        Some(Cell::new(4)),
    ];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will succeed on a group with two like values separated by a pair of
/// like values. Only one merge should be performed, and the group will shift.
#[test]
fn test_shift_group_2_4_4_2() {
    let mut row = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
    ];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(2)), *iter.next().unwrap());
    assert_eq!(Some(Cell::new(8)), *iter.next().unwrap());
    assert_eq!(Some(Cell::new(2)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

/// Affirm that `Board::shift_group` will succeed on a group with all like values. Only two merges should
/// be performed, and the group will shift.
#[test]
fn test_shift_group_2_2_2_2() {
    let mut row = vec![
        Some(Cell::new(2)),
        Some(Cell::new(2)),
        Some(Cell::new(2)),
        Some(Cell::new(2)),
    ];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(4)), *iter.next().unwrap());
    assert_eq!(Some(Cell::new(4)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

impl Board {
    /// Attempt to undo the board to the previous move state.
    fn board_undo(&mut self) -> Result<(), ()> {
        if let Some(state) = self.history.pop() {
            self.grid = state;
            return Ok(());
        }
        Err(())
    }
}

// TODO: test `Board::board_undo`, probably rename the method to remove "board_" prefix

impl Board {
    fn board_shift(&mut self, dir: Direction) -> Result<(), ()> {
        todo!();
    }
}

impl Board {
    /// Handles movement on the game board.
    ///
    /// # Arguments
    ///
    /// * `mov` - the movement type to handle
    pub fn movement(&mut self, mov: Move) -> Result<(), ()> {
        match mov {
            Move::Shift(dir) => self.board_shift(dir),
            Move::Undo => self.board_undo(),
        }
    }
}
