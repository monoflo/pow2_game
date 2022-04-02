use rand::Rng;

/// The representation of a cell on the game board.
#[derive(Copy, Clone, Debug)]
pub struct Cell(usize);

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Affirm that the implementation of the `Display` trait for `Cell` works properly.
#[test]
fn test_to_string() {
    assert_eq!("64", Cell(64).to_string());
}

impl Cell {
    #[inline(always)]
    /// Returns whether the cell is empty.
    pub fn is_empty(&self) -> bool {
        self.value() == 0
    }
}

/// Affirm that `Cell::is_empty()` returns true for a cell having a value of zero.
#[test]
fn test_is_empty_on_empty() {
    assert!(Cell(0).is_empty());
}

/// Affirm that `Cell::is_empty()` returns false for a cell having a non-zero value.
#[test]
fn test_is_empty_on_nonempty() {
    assert!(!Cell(2).is_empty());
}

impl Cell {
    #[allow(dead_code)]
    #[inline(always)]
    /// Returns the value held by the cell.
    pub fn value(&self) -> usize {
        self.0
    }
}

/// Affirm that `Cell::value()` returns the value that the cell holds.
#[test]
fn test_value() {
    const EXPECTED: usize = 4;
    assert_eq!(EXPECTED, Cell(EXPECTED).value());
}

impl Cell {
    /// Returns a new, empty instance of a cell.
    pub fn new() -> Self {
        Self(0)
    }
}

/// Affirm that `Cell::new()` initializes cells with a zero value.
#[test]
fn test_new() {
    let cell = Cell::new();
    assert_eq!(0, cell.value());
}

impl Cell {
    #[allow(dead_code)]
    /// Attempts to spawn the specified value into the cell.
    /// Fails if the cell is already non-empty.
    ///
    /// # Arguments
    ///
    /// * `val` - the value to spawn into the cell
    pub fn spawn_value(&mut self, val: usize) -> Result<(), ()> {
        if !self.is_empty() {
            return Err(());
        }

        self.0 = val;
        Ok(())
    }
}

/// Affirm that `Cell::spawn_value()` will result in an error if the cell is non-empty.
#[test]
fn test_spawn_value_nonempty() {
    Cell(2).spawn_value(2).unwrap_err();
}

/// Affirm that `Cell::spawn_value()` will initialize an empty cell to that of the specified value.
#[test]
fn test_spawn_value_empty() {
    const EXPECTED: usize = 2;
    let mut cell = Cell(0);
    cell.spawn_value(EXPECTED).unwrap();
    assert_eq!(EXPECTED, cell.value());
}

impl Cell {
    /// Attempts to randomly spawn a value of either two or four into the cell.
    /// Fails if the cell is already non-empty.
    pub fn spawn(&mut self) -> Result<(), ()> {
        const CHANCE_OF_FOUR: f64 = 0.1;
        if !self.is_empty() {
            return Err(());
        }

        let val = match rand::thread_rng().gen_bool(CHANCE_OF_FOUR) {
            true => 4,
            false => 2,
        };

        self.0 = val;
        Ok(())
    }
}

/// Affirm that `Cell::spawn()` will result in an error if the cell is non-empty.
#[test]
fn test_spawn_nonempty() {
    Cell(2).spawn().unwrap_err();
}

/// Affirm that `Cell::spawn()` will initialize an empty cell to either two or four.
#[test]
fn test_spawn_empty() {
    static VALID: [usize; 2] = [2, 4];
    let mut cell = Cell(0);
    cell.spawn().unwrap();
    assert!(VALID.contains(&cell.value()));
}

impl Cell {
    #[allow(dead_code)]
    /// Reverts the cell back to an empty state.
    fn despawn(&mut self) -> Result<(), ()> {
        if self.is_empty() {
            return Err(());
        }
        self.0 = 0;
        Ok(())
    }
}

/// Affirm that `Cell::despawn()` will result in an error if the cell is already empty.
#[test]
fn test_despawn_empty() {
    Cell(0).despawn().unwrap_err();
}

/// Affirm that `Cell::despawn()` will reset the value of a non-empty cell to zero.
#[test]
fn test_despawn_nonempty() {
    let mut cell = Cell(2);
    cell.despawn().unwrap();
    assert_eq!(0, cell.value());
}

impl Cell {
    #[allow(dead_code)]
    /// Increases the value of the cell by a power of two.
    fn grow(&mut self) -> Result<(), ()> {
        if self.is_empty() {
            return Err(());
        }

        let (result, overflow) = self.0.overflowing_mul(2);

        if overflow {
            return Err(());
        }

        self.0 = result;
        Ok(())
    }
}

#[test]
/// Affirm that `Cell::grow()` will result in an error for an empty cell.
fn test_grow_from_empty() {
    Cell(0).grow().unwrap_err();
}

#[test]
/// Affirm that `Cell::grow()` will return four for a cell with the value of two.
fn test_grow_from_two() {
    const INIT: usize = 2;
    const EXPECTED: usize = 4;
    let mut cell = Cell(INIT);
    cell.grow().unwrap();
    assert_eq!(EXPECTED, cell.value());
}

#[test]
/// Affirm that `Cell::grow()` will return eight for a cell with the value of four.
fn test_grow_from_four() {
    const INIT: usize = 4;
    const EXPECTED: usize = 8;
    let mut cell = Cell(INIT);
    cell.grow().unwrap();
    assert_eq!(EXPECTED, cell.value());
}

#[test]
/// Affirm that `Cell::grow()` will succeed if the cell value is equal to the second
/// most-significant set bit.
fn test_grow_from_second_highest_bit_set() {
    const INIT: usize = 1 << (usize::BITS - 2);
    Cell(INIT).grow().unwrap();
}

#[test]
/// Affirm that `Cell::grow()` will result in an error if the cell value is equal to the
/// most-significant set bit.
fn test_grow_from_highest_bit_set() {
    const INIT: usize = 1 << (usize::BITS - 1);
    Cell(INIT).grow().unwrap_err();
}

#[test]
/// Affirm that `Cell::grow()` will result in an error if the cell value is equal to the type
/// maximum.
fn test_grow_from_max_val() {
    const INIT: usize = usize::MAX;
    Cell(INIT).grow().unwrap_err();
}

impl Cell {
    #[allow(dead_code)]
    /// Merge the current cell with another.
    /// If successful, `self` will grow while the `other` will be despawned.
    /// Fails if the cell values are not equal to each other or if one is empty.
    ///
    /// # Arguments
    ///
    /// * `other` - the other cell to merge with (that will be despawned)
    pub fn merge(&mut self, other: &mut Self) -> Result<(), ()> {
        if self.is_empty() || self.value() != other.value() {
            return Err(());
        }

        self.grow().unwrap();
        other.despawn().unwrap();

        Ok(())
    }
}

/// Affirm that an error will occur if `Cell::merge()` is performed on two cells that are both
/// empty.
#[test]
fn test_merge_both_empty() {
    let mut merger = Cell::new();
    let mut mergee = Cell::new();
    mergee.merge(&mut merger).unwrap_err();
}

/// Affirm that an error will occur if `Cell::merge()` is performed *on* a cell that has not
/// been spawned.
#[test]
fn test_merge_as_empty() {
    let mut merger = Cell::new();
    let mut mergee = Cell::new();
    merger.spawn().unwrap();
    mergee.merge(&mut merger).unwrap_err();
}

/// Affirm that an error will occur if `Cell::merge()` is performed *against* a cell that has
/// not been spawned.
#[test]
fn test_merge_with_empty() {
    let mut merger = Cell::new();
    let mut mergee = Cell::new();
    mergee.spawn().unwrap();
    mergee.merge(&mut merger).unwrap_err();
}

/// Affirm that `Cell::merge()` will succeed when performed on two cells of equal value, and
/// also that their values will update appropriately.
#[test]
fn test_merge_with_equal() {
    let mut merger = Cell(2);
    let mut mergee = Cell(2);
    mergee.merge(&mut merger).unwrap();
    assert_eq!(4, mergee.0);
    assert_eq!(0, merger.0);
}

/// Affirm that an error will occur if `Cell::merge()` is performed on two cells of unequal
/// value.
#[test]
fn test_merge_with_unequal() {
    let mut merger = Cell(2);
    let mut mergee = Cell(4);
    mergee.merge(&mut merger).unwrap_err();
}
