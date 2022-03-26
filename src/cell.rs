use rand::Rng;

/// Defines the likelihood of spawning a four instead of a two.
const LIKELIHOOD_SPAWN_FOUR: f64 = 0.1;

/// The representation of a cell on the game board.
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    /// The value represented by the cell.
    value: usize,
}

impl Cell {
    /// Returns a new, empty instance of a cell.
    pub fn new() -> Self {
        Self { value: 0 }
    }

    /// Returns whether the cell is empty.
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    /// Returns the value held by the cell.
    pub fn value(&self) -> usize {
        self.value
    }

    /// Merge the current cell with another.
    /// If successful, `self` will grow while the `other` will be despawned.
    /// Fails if the cell values are not equal to each other or if one is empty.
    ///
    /// # Arguments
    ///
    /// * `other` - the other cell to merge with (that will be despawned)
    pub fn merge(&mut self, other: &mut Self) -> Result<(), ()> {
        if self.is_empty() || self.value != other.value {
            return Err(());
        }

        self.grow();
        other.despawn();

        Ok(())
    }

    /// Attempts to spawn a non-empty cell.
    /// Fails if the cell already stores a non-zero value.
    pub fn spawn(&mut self) -> Result<(), ()> {
        if self.value == 0 {
            self.value = self.spawn_value();
            return Ok(());
        }
        Err(())
    }

    /// Generates a new, intial value for the cell.
    /// Either a two or a four.
    fn spawn_value(&self) -> usize {
        let mut rng = rand::thread_rng();
        return match rng.gen_bool(LIKELIHOOD_SPAWN_FOUR) {
            true => 4,
            false => 2,
        };
    }

    /// Reverts the cell back to an empty state.
    fn despawn(&mut self) {
        self.value = 0;
    }

    /// Increases the value of the cell by a power of two.
    fn grow(&mut self) {
        assert_ne!(0, self.value);
        self.value <<= 1;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Affirm that `Cell::new()` initializes cells with `value = 0`.
    fn new() {
        let cell = Cell::new();
        assert_eq!(0, cell.value);
    }

    #[test]
    /// Affirm that `Cell::spawn()` can initialize the cell `value` to both two and four.
    fn spawn() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();

        // test first value
        let val = cell.value;
        assert!(val == 2 || val == 4);

        // spawn until other value is reached
        while val == cell.value {
            cell.despawn();
            cell.spawn().unwrap();
        }

        // test other value
        assert!(val == 2 || val == 4);
    }

    #[test]
    /// Affirm that `Cell::spawn()` will result in an error if the cell has already been spawned.
    fn double_spawn() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();
        cell.spawn().unwrap_err();
    }

    #[test]
    /// Affirm that `Cell::grow()` will double the `value` of the cell.
    fn grow() {
        let mut cell = Cell { value: 2 };
        cell.grow();
        assert_eq!(4, cell.value);
        cell.value = 8;
        cell.grow();
        assert_eq!(16, cell.value);
        cell.value = 32;
        cell.grow();
        assert_eq!(64, cell.value);
    }

    #[test]
    /// Affirm that `Cell::despawn()` will reset the `value` of the cell to zero.
    fn despawn() {
        let mut cell = Cell { value: 2 };
        cell.despawn();
        assert_eq!(0, cell.value);
    }

    #[test]
    /// Affirm that an error will occur if `Cell::merge()` is performed on two cells that are both
    /// empty.
    fn merge_both_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();
        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    /// Affirm that an error will occur if `Cell::merge()` is performed *on* a cell that has not
    /// been spawned.
    fn merge_as_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();
        merger.spawn().unwrap();
        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    /// Affirm that an error will occur if `Cell::merge()` is performed *against* a cell that has
    /// not been spawned.
    fn merge_with_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();
        mergee.spawn().unwrap();
        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    /// Affirm that `Cell::merge()` will succeed when performed on two cells of equal `value`, and
    /// also that their `value`s will update appropriately.
    fn merge_with_equal() {
        let mut merger = Cell { value: 2 };
        let mut mergee = Cell { value: 2 };
        mergee.merge(&mut merger).unwrap();
        assert_eq!(4, mergee.value);
        assert_eq!(0, merger.value);
    }

    #[test]
    /// Affirm that an error will occur if `Cell::merge()` is performed on two cells of unequal
    /// `value`.
    fn merge_with_unequal() {
        let mut merger = Cell { value: 2 };
        let mut mergee = Cell { value: 4 };
        mergee.merge(&mut merger).unwrap_err();
    }
}
