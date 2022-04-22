use super::*;

/// Affirm that `Cell::default()` will initialize the value to either two or four.
#[test]
fn default() {
    assert!([2, 4].contains(&Cell::default().0));
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of zero.
#[test]
#[should_panic]
fn new_0() {
    Cell::new(0);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of one.
#[test]
#[should_panic]
fn new_1() {
    Cell::new(1);
}

/// Affirm that `Cell::new()` will succeed for the default initalization values (two and four).
#[test]
fn new_defaults() {
    Cell::new(2);
    Cell::new(4);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of three.
#[test]
#[should_panic]
fn new_3() {
    Cell::new(3);
}

/// Affirm that `Cell::new()` will succeed for the largest possible power of two that can be
/// represented by the size.
#[test]
fn new_msb() {
    const V: usize = 1 << (usize::BITS - 1);
    Cell::new(V);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of `usize::MAX`.
#[test]
#[should_panic]
fn new_max() {
    Cell::new(usize::MAX);
}
