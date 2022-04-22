use std::collections::HashMap;

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
    /// The saved, past states of the board that can be.
    history: Vec<BoardGrid>,
    /// The calculated boards for shifts in each direction.
    next: HashMap<Direction, Option<BoardGrid>>,
}

/// Implementation of the `Default` trait for `Board`.
impl Default for Board {
    /// Create an empty grid and an empty, bound-vector of grid states.
    fn default() -> Self {
        Self {
            grid: Default::default(),
            history: Vec::with_capacity(HISTORY_SIZE),
            next: HashMap::new(),
        }
    }
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

    /// Retrieves all cells matching the specified emptiness.
    ///
    /// # Arguments
    ///
    /// * `is_empty` - whether the cell should be empty; search criteria
    fn get_cells_by_emptiness(&self, is_empty: bool) -> impl Iterator<Item = BoardCoord> + '_ {
        (0..BOARD_ROWS).flat_map(move |row| self.get_cells_by_emptiness_row(is_empty, row))
    }

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

    /// Returns a new instance of a game board.
    pub fn new() -> Self {
        let mut inst = Board::default();
        inst.spawn()
            .expect("failed to spawn a cell on the empty board");
        inst
    }

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

        for pair in mergeable {
            let (ls, rs) = result.split_at_mut(pair.1);
            let mergee = ls[pair.0].as_mut().unwrap();

            // note: `take` replaces the value with `None`
            let merger = rs[0].take().unwrap();

            mergee.merge(merger).unwrap();
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

    // TODO: test `Board::board_undo`, probably rename the method to remove "board_" prefix
    /// Attempt to undo the board to the previous move state.
    fn board_undo(&mut self) -> Result<(), ()> {
        let state = self.history.pop().ok_or(())?;
        self.grid = state;
        Ok(())
    }

    fn board_shift(&mut self, dir: Direction) -> Result<(), ()> {
        todo!();
    }

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

#[cfg(test)]
mod test;
