use super::*;

/// Affirm that the default board is instantiated with `None` in each cell.
#[test]
fn grid() {
    assert!(Board::default()
        .grid
        .elements_row_major_iter()
        .all(|cell| cell.is_none()));
}

/// Affirm that the default board is instantiated with empty history, containing the maximum
/// capacity specified by `HISTORY_SIZE`.
#[test]
fn history() {
    let board = Board::default();
    assert_eq!(HISTORY_SIZE, board.history.capacity());
    assert!(board.history.is_empty());
}
