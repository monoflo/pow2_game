use super::*;

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value of zero.
#[test]
#[should_panic]
fn input_0() {
    Cell(0).grow();
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value of one.
#[test]
#[should_panic]
fn input_1() {
    Cell(1).grow();
}

/// Affirm that `Cell::grow()` will return four for a cell with the value of two.
#[test]
fn input_2() {
    let mut cell = Cell(2);
    cell.grow().unwrap();
    assert_eq!(4, cell.0);
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value that is a non-power
/// of two.
#[test]
#[should_panic]
fn input_3() {
    Cell(3).grow();
}

/// Affirm that `Cell::grow()` will return eight for a cell with the value of four.
#[test]
fn input_4() {
    let mut cell = Cell(4);
    cell.grow().unwrap();
    assert_eq!(8, cell.0);
}

/// Affirm that `Cell::grow()` will succeed if the cell value is equal to the second
/// most-significant set bit.
#[test]
fn input_second_msb() {
    const V: usize = 1 << (usize::BITS - 2);
    const E: usize = 1 << (usize::BITS - 1);
    let mut cell = Cell(V);
    cell.grow().unwrap();
    assert_eq!(E, cell.0);
}

/// Affirm that `Cell::grow()` will result in an error if the cell value is equal to the
/// most-significant set bit.
#[test]
fn input_msb() {
    const V: usize = 1 << (usize::BITS - 1);
    Cell(V).grow().unwrap_err();
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value that is a non-power
/// of two.
#[test]
#[should_panic]
fn input_max() {
    Cell(usize::MAX).grow();
}
