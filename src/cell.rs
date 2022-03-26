use rand::Rng;

/// Defines the likelihood of spawning a four instead of a two.
const LIKELIHOOD_SPAWN_FOUR: f64 = 0.1;

#[derive(Copy, Clone, Debug)]
/// The representation of a cell on the game board.
pub struct Cell(usize);

impl Cell {
    /// Returns a new, empty instance of a cell.
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns whether the cell is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns the value held by the cell.
    pub fn value(&self) -> usize {
        self.0
    }

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

    /// Attempts to spawn a non-empty cell.
    /// Fails if the cell already stores a non-zero value.
    pub fn spawn(&mut self) -> Result<(), ()> {
        if self.0 == 0 {
            self.0 = self.spawn_value();
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
    fn despawn(&mut self) -> Result<(), ()> {
        if self.0 == 0 {
            return Err(());
        }
        self.0 = 0;
        Ok(())
    }

    /// Increases the value of the cell by a power of two.
    fn grow(&mut self) {
        assert_ne!(0, self.0);
        self.0 <<= 1;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Affirm that `Cell::new()` initializes cells with a zero value.
    fn new() {
        let cell = Cell::new();
        assert_eq!(0, cell.0);
    }

    #[test]
    /// Affirm that `Cell::spawn()` can initialize the cell value to both two and four.
    fn spawn() {
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

    #[test]
    /// Affirm that `Cell::spawn()` will result in an error if the cell has already been spawned.
    fn double_spawn() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();
        cell.spawn().unwrap_err();
    }

    #[test]
    /// Affirm that `Cell::grow()` will double the value of the cell.
    fn grow() {
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

    #[test]
    /// Affirm that `Cell::despawn()` will reset the value of the cell to zero.
    fn despawn() {
        let mut cell = Cell(2);
        cell.despawn().unwrap();
        assert_eq!(0, cell.0);
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
    /// Affirm that `Cell::merge()` will succeed when performed on two cells of equal value, and
    /// also that their values will update appropriately.
    fn merge_with_equal() {
        let mut merger = Cell(2);
        let mut mergee = Cell(2);
        mergee.merge(&mut merger).unwrap();
        assert_eq!(4, mergee.0);
        assert_eq!(0, merger.0);
    }

    #[test]
    /// Affirm that an error will occur if `Cell::merge()` is performed on two cells of unequal
    /// value.
    fn merge_with_unequal() {
        let mut merger = Cell(2);
        let mut mergee = Cell(4);
        mergee.merge(&mut merger).unwrap_err();
    }
}
