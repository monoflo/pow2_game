use super::*;

/// Affirm that each cell on the board can be spawned-at randomly until the board is full and
/// then no more.
#[test]
fn exhaustive() {
    let mut board = Board::default();
    let num_cells = BOARD_ROWS.checked_mul(BOARD_COLS).unwrap();

    for _ in 0..num_cells {
        board.spawn().unwrap();
    }

    board.spawn().unwrap_err();
}
