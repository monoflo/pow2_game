use super::*;

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for an empty collection.
#[test]
fn input_0_0_0_0() {
    let cells = vec![None, None, None, None];
    assert!(Board::get_mergeable(&cells).is_empty());
}

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for a singular non-empty cell.
#[test]
fn input_0_0_0_2() {
    let cells = vec![None, None, None, Some(Cell::new(2))];
    assert!(Board::get_mergeable(&cells).is_empty());
}

/// Affirm that `Board::get_mergeable` reports one mergeable pair for a contiguous pair of
/// same-valued cells.
#[test]
fn input_0_0_2_2() {
    let expect = vec![(2, 3)];
    let cells = vec![None, None, Some(Cell::new(2)), Some(Cell::new(2))];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

/// Affirm that `Board::get_mergeable` reports one mergeable pair for a pair of same-valued cells
/// separated by an empty cell.
#[test]
fn input_0_2_0_2() {
    let expect = vec![(1, 3)];
    let cells = vec![None, Some(Cell::new(2)), None, Some(Cell::new(2))];
    assert_eq!(expect, Board::get_mergeable(&cells));
}

/// Affirm that `Board::get_mergeable` reports no mergeable pairs for a pair of same-valued cells
/// separated by a cell of dissimilar value to the others.
#[test]
fn input_0_2_4_2() {
    let cells = vec![
        None,
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
    ];
    assert!(Board::get_mergeable(&cells).is_empty());
}

#[test]
fn input_2_2_2_2() {
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
fn input_2_4_4_2() {
    let expect = vec![(1, 2)];
    let cells = vec![
        Some(Cell::new(2)),
        Some(Cell::new(4)),
        Some(Cell::new(4)),
        Some(Cell::new(2)),
    ];
    assert_eq!(expect, Board::get_mergeable(&cells));
}
