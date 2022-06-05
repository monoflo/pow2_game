use super::*;

/// Affirm that each cell on the board can be spawned-at sequentially until the board is full
/// and then no more.
#[test]
fn exhaustive() {
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
fn corner_bottom_right() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(BOARD_ROWS - 1, BOARD_COLS - 1)]);
    let mut cells = board.grid.elements_row_major_iter();
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the bottom-leftmost cell will be non-empty if a cell is spawned there.
#[test]
fn corner_bottom_left() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(BOARD_ROWS - 1, 0)]);
    let mut cells = board.grid.elements_row_major_iter();
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
fn corner_top_left() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(0, 0)]);
    let mut cells = board.grid.elements_row_major_iter();
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that only the top-rightmost cell will be non-empty if a cell is spawned there.
#[test]
fn corner_top_right() {
    let mut board = Board::default();
    board.spawn_at_many(vec![(0, BOARD_COLS - 1)]);
    let mut cells = board.grid.elements_row_major_iter();
    for _ in 0..BOARD_COLS - 1 {
        assert!(cells.next().unwrap().is_none());
    }
    assert!(cells.next().unwrap().is_some());
    assert!(cells.all(|cell| cell.is_none()));
}

/// Affirm that attempting to spawn in an invalid column will fail.
#[test]
#[should_panic]
fn invalid_col() {
    let col = usize::MAX;
    assert!(col < BOARD_COLS);
    let mut board = Board::default();
    board.spawn_at((0, col)).unwrap_err();
}

/// Affirm that attempting to spawn in an invalid row will fail.
#[test]
#[should_panic]
fn invalid_row() {
    let row = usize::MAX;
    assert!(row < BOARD_ROWS);
    let mut board = Board::default();
    board.spawn_at((row, 0)).unwrap_err();
}
