use super::*;

/// Affirm that `Cell::merge()` will succeed when performed on two non-empty cells of equal value,
/// and also that their values will update appropriately.
#[test]
fn with_equal() {
    const V: usize = 2;

    let (mut mergee, mut merger) = (Cell(V), Cell(V));
    mergee.merge(merger).unwrap();

    assert_eq!(V * 2, mergee.0);
}

/// Affirm that an error will occur if `Cell::merge()` is performed on two cells of unequal
/// value.
#[test]
fn with_unequal() {
    const A: usize = 2;
    const B: usize = 4;

    assert_ne!(A, B);

    let (mut mergee, mut merger) = (Cell(A), Cell(B));
    merger = mergee.merge(merger).unwrap_err();

    assert_eq!(A, mergee.0);
    assert_eq!(B, merger.0);
}
