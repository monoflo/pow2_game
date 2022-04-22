use rand::Rng;

/// The representation of a cell on the game board.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell(usize);

impl Cell {
    /// Returns the value held by the cell.
    #[inline(always)]
    pub fn value(&self) -> usize {
        self.0
    }
}

/// Implementation of the `Display` trait for `Cell`.
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Affirm that the implementation of the `Display` trait for `Cell` works properly.
#[test]
fn test_to_string() {
    assert_eq!("64", Cell(64).to_string());
}

/// Implementation of the `Drop` trait for `Cell`.
impl Drop for Cell {
    fn drop(&mut self) {}
}

/// Implementation of the `Default` trait for `Cell`.
impl Default for Cell {
    /// Randomly initializes the value of the cell to either two or four.
    fn default() -> Self {
        const CHANCE_OF_FOUR: f64 = 0.1;
        Self(match rand::thread_rng().gen_bool(CHANCE_OF_FOUR) {
            true => 4,
            false => 2,
        })
    }
}

/// Affirm that `Cell::default()` will initialize the value to either two or four.
#[test]
fn test_default() {
    assert!([2, 4].contains(&Cell::default().0));
}

impl Cell {
    /// Allows the instantiation of a cell with a specified value given that value is a power of
    /// two greater than one.
    ///
    /// # Arguments
    ///
    /// * `value` - the power of two value to initialize the new cell to
    pub fn new(value: usize) -> Self {
        if value.count_ones() != 1 || value < 2 {
            panic!();
        }
        Self(value)
    }
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of zero.
#[test]
#[should_panic]
fn test_new_0() {
    Cell::new(0);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of one.
#[test]
#[should_panic]
fn test_new_1() {
    Cell::new(1);
}

/// Affirm that `Cell::new()` will succeed for the default initalization values (two and four).
#[test]
fn test_new_defaults() {
    Cell::new(2);
    Cell::new(4);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of three.
#[test]
#[should_panic]
fn test_new_3() {
    Cell::new(3);
}

/// Affirm that `Cell::new()` will succeed for the largest possible power of two that can be
/// represented by the size.
#[test]
fn test_new_msb() {
    const V: usize = 1 << (usize::BITS - 1);
    Cell::new(V);
}

/// Affirm that `Cell::new()` will fail to initialize a cell with a value of `usize::MAX`.
#[test]
#[should_panic]
fn test_new_max() {
    Cell::new(usize::MAX);
}

impl Cell {
    /// Increases the value of the cell by a power of two.
    fn grow(&mut self) -> Result<(), ()> {
        const MSB_SET: usize = 1 << (usize::BITS - 1);

        // assert that the value is a power of two greater than one
        assert_eq!(1, self.0.count_ones());
        assert!(self.0 > 1);

        if MSB_SET == self.0 {
            return Err(());
        }

        self.0 <<= 1;
        Ok(())
    }
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value of zero.
#[test]
#[should_panic]
fn test_grow_0() {
    Cell(0).grow();
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value of one.
#[test]
#[should_panic]
fn test_grow_1() {
    Cell(1).grow();
}

/// Affirm that `Cell::grow()` will return four for a cell with the value of two.
#[test]
fn test_grow_2() {
    let mut cell = Cell(2);
    cell.grow().unwrap();
    assert_eq!(4, cell.0);
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value that is a non-power
/// of two.
#[test]
#[should_panic]
fn test_grow_3() {
    Cell(3).grow();
}

/// Affirm that `Cell::grow()` will return eight for a cell with the value of four.
#[test]
fn test_grow_4() {
    let mut cell = Cell(4);
    cell.grow().unwrap();
    assert_eq!(8, cell.0);
}

/// Affirm that `Cell::grow()` will succeed if the cell value is equal to the second
/// most-significant set bit.
#[test]
fn test_grow_second_MSB() {
    const V: usize = 1 << (usize::BITS - 2);
    const E: usize = 1 << (usize::BITS - 1);
    let mut cell = Cell(V);
    cell.grow().unwrap();
    assert_eq!(E, cell.0);
}

/// Affirm that `Cell::grow()` will result in an error if the cell value is equal to the
/// most-significant set bit.
#[test]
fn test_grow_MSB() {
    const V: usize = 1 << (usize::BITS - 1);
    Cell(V).grow().unwrap_err();
}

/// Affirm that `Cell::grow()` will panic if the cell unexpectedly has a value that is a non-power
/// of two.
#[test]
#[should_panic]
fn test_grow_max() {
    Cell(usize::MAX).grow();
}

impl Cell {
    /// Iff the cells have equal value, then `self` will grow whereas `other` will be dropped.
    ///
    /// # Arguments
    ///
    /// * `other` - the other cell to merge with (that will be dropped on merge)
    ///
    /// # Notes
    ///
    /// * `other` should be assigned to the result of the function call
    pub fn merge(&mut self, other: &mut Self) -> Result<(), ()> {
        match *self == *other {
            true => {
                self.grow().unwrap();
                drop(other);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

/// Affirm that `Cell::merge()` will succeed when performed on two non-empty cells of equal value,
/// and also that their values will update appropriately.
#[test]
fn test_merge_with_equal() {
    const V: usize = 2;

    let (mut mergee, mut merger) = (Cell(V), Cell(V));
    mergee.merge(&mut merger).unwrap();

    assert_eq!(V * 2, mergee.0);
}

/// Affirm that an error will occur if `Cell::merge()` is performed on two cells of unequal
/// value.
#[test]
fn test_merge_with_unequal() {
    const A: usize = 2;
    const B: usize = 4;

    assert_ne!(A, B);

    let (mut mergee, mut merger) = (Cell(A), Cell(B));
    mergee.merge(&mut merger).unwrap_err();

    assert_eq!(A, mergee.0);
    assert_eq!(B, merger.0);
}
