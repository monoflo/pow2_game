use rand::Rng;

/// Defines the likelihood of spawning a four instead of a two.
const LIKELIHOOD_SPAWN_FOUR: f64 = 0.1;

/// The representation of a cell on the game board.
#[derive(Copy, Clone, Debug)]
pub struct Cell(usize);

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Cell {
    /// Returns whether the cell is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[test]
fn test_is_empty_after_new() {
    let cell = Cell::new();
    assert!(cell.is_empty());
}

#[test]
fn test_is_empty_after_spawn() {
    let mut cell = Cell::new();
    cell.spawn().unwrap();
    assert!(!cell.is_empty());
}

impl Cell {
    /// Returns the value held by the cell.
    pub fn value(&self) -> usize {
        self.0
    }
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
    assert_eq!(0, cell.0);
}

impl Cell {
    /// Generates a new, intial value for the cell.
    /// Either a two or a four.
    fn spawn_value(&self) -> usize {
        let mut rng = rand::thread_rng();
        return match rng.gen_bool(LIKELIHOOD_SPAWN_FOUR) {
            true => 4,
            false => 2,
        };
    }
}

impl Cell {
    /// Attempts to spawn a non-empty cell.
    /// Fails if the cell already stores a non-zero value.
    pub fn spawn(&mut self) -> Result<(), ()> {
        if self.0 == 0 {
            self.0 = self.spawn_value();
            return Ok(());
        }
        Err(())
    }
}

/// Affirm that `Cell::spawn()` can initialize the cell value to both two and four.
#[test]
fn test_spawn() {
    let mut cell = Cell::new();
    cell.spawn().unwrap();

    // test first value
    let val = cell.0;
    assert!(val == 2 || val == 4);

    // spawn until other value is reached
    while val == cell.0 {
        cell.despawn().unwrap();
        cell.spawn().unwrap();
    }

    // test other value
    assert!(val == 2 || val == 4);
}

/// Affirm that `Cell::spawn()` will result in an error if the cell has already been spawned.
#[test]
fn test_double_spawn() {
    let mut cell = Cell::new();
    cell.spawn().unwrap();
    cell.spawn().unwrap_err();
}

impl Cell {
    /// Reverts the cell back to an empty state.
    fn despawn(&mut self) -> Result<(), ()> {
        if self.0 == 0 {
            return Err(());
        }
        self.0 = 0;
        Ok(())
    }
}

/// Affirm that `Cell::despawn()` will reset the value of the cell to zero.
#[test]
fn test_despawn() {
    let mut cell = Cell(2);
    cell.despawn().unwrap();
    assert_eq!(0, cell.0);
}

impl Cell {
    /// Increases the value of the cell by a power of two.
    fn grow(&mut self) {
        assert_ne!(0, self.0);
        self.0 <<= 1;
    }
}

/// Affirm that `Cell::grow()` will double the value of the cell.
#[test]
fn test_grow() {
    let mut cell = Cell(2);
    cell.grow();
    assert_eq!(4, cell.0);
    cell.0 = 8;
    cell.grow();
    assert_eq!(16, cell.0);
    cell.0 = 32;
    cell.grow();
    assert_eq!(64, cell.0);
}

impl Cell {
    /// Merge the current cell with another.
    /// If successful, `self` will grow while the `other` will be despawned.
    /// Fails if the cell values are not equal to each other or if one is empty.
    ///
    /// # Arguments
    ///
    /// * `other` - the other cell to merge with (that will be despawned)
    pub fn merge(&mut self, other: &mut Self) -> Result<(), ()> {
        if self.is_empty() || self.0 != other.0 {
            return Err(());
        }

        self.grow();
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
