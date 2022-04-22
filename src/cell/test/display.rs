use super::*;

/// Affirm that the implementation of the `Display` trait for `Cell` works properly.
#[test]
fn to_string() {
    assert_eq!("64", Cell(64).to_string());
}
