use super::*;

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

/// Affirm that `Board::get_cells_by_emptiness()` functions properly on an empty board state.
#[test]
fn test_get_cells_by_emptiness_board_empty() {
    let board = Board::default();
    let vec = board
        .get_cells_by_emptiness(true)
        .collect::<Vec<BoardCoord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
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
