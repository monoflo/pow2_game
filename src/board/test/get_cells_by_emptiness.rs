use super::*;

/// Affirm that `Board::get_cells_by_emptiness_col()` functions properly on an empty board state.
#[test]
fn col_board_empty() {
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
fn row_board_empty() {
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
fn board_empty() {
    let board = Board::default();
    let vec = board
        .get_cells_by_emptiness(true)
        .collect::<Vec<BoardCoord>>();
    assert_eq!(BOARD_ROWS.checked_mul(BOARD_COLS).unwrap(), vec.len());
}
