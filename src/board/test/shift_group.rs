use super::*;

/// Affirm that `Board::shift_group` will return nothing on a group of empty cells.
#[test]
fn input_0_0_0_0() {
    let mut row: Vec<BoardCell> = vec![None, None, None, None];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will return nothing on a group containing one cell positioned at the
/// start.
#[test]
fn input_2_0_0_0() {
    let row = vec![Some(Cell::new(2)), None, None, None];
    let result = Board::shift_group(row);
    assert!(result.is_none());
}

/// Affirm that `Board::shift_group` will successfully shift a group containing one cell positioned at
/// the end to the start.
#[test]
fn input_0_0_0_2() {
    let mut row = vec![None, None, None, Some(Cell::new(2))];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(2)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

/// Affirm that `Board::shift_group` will successfully merge two cells at the start of the group.
#[test]
fn input_2_2_0_0() {
    let mut row = vec![Some(Cell::new(2)), Some(Cell::new(2)), None, None];
    let result = Board::shift_group(row).unwrap();
    let mut iter = result.iter();
    assert_eq!(Some(Cell::new(4)), *iter.next().unwrap());
    assert!(iter.all(|cell| cell.is_none()));
}

/// Affirm that `Board::shift_group` will return nothing on a group where none of the cells have like
/// values, as neither a shift or merge can be performed.
#[test]
fn input_2_4_8_16() {
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
fn input_2_4_2_4() {
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
fn input_2_4_4_2() {
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
fn input_2_2_2_2() {
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
