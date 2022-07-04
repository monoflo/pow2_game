use super::*;

/// Affirm that `Board::get_cells_by_emptiness()` functions properly on an empty board state.
#[test]
fn board_empty() {
    let board = Board::default();
    let vec = board
        .get_cells_by_emptiness(true)
        .collect::<Vec<BoardCoord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
}
