use super::*;

/// Affirm that `Board::new()` initializes a board with all cells having `value = 0`,
/// except one.
#[test]
fn test_new() {
    let board = Board::new();
    let mut found = false;
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let is_empty = board.grid.get(row, col).unwrap().is_none();
            assert!(is_empty || !found);
            found = found || !is_empty;
        }
    }
}
