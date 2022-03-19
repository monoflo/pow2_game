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
    fn init() {
        let cell = Cell::new();
        assert_eq!(0, cell.value);
    }

    #[test]
    fn spawn() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();

        let val = cell.value;
        assert!(val == 2 || val == 4);
    }

    #[test]
    fn double_spawn() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();
        cell.spawn().unwrap_err();
    }

    #[test]
    fn grow() {
        let mut cell = Cell::new();
        cell.spawn().unwrap();

        let val = cell.value;
        cell.grow();

        assert_eq!(val * 2, cell.value);
    }

    #[test]
    fn despawn() {
        let mut cell = Cell::new();

        cell.spawn().unwrap();
        assert_ne!(0, cell.value);

        cell.despawn();
        assert_eq!(0, cell.value);
    }

    #[test]
    fn merge_both_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();

        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    fn merge_as_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();

        merger.spawn();

        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    fn merge_with_empty() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();

        mergee.spawn();

        mergee.merge(&mut merger).unwrap_err();
    }

    #[test]
    fn merge_with_equal() {
        let mut merger = Cell::new();
        let mut mergee = Cell::new();

        merger.spawn();
        mergee.spawn();

        // ensure merger's value is two (most likely value)
        while merger.value != 2 {
            merger.despawn();
            merger.spawn();
        }

        // ensure mergee's value is equal to merger's
        while mergee.value != merger.value {
            merger.despawn();
            merger.spawn();
        }

        // record value of mergee before merge
        let val = mergee.value;

        // merge cells
        mergee.merge(&mut merger).unwrap();

        // ensure that the mergee has grown
        assert_eq!(val * 2, mergee.value);

        // ensure that the merger has despawned
        assert_eq!(0, merger.value);
    }

    #[test]
    fn merge_with_unequal() {
        // TODO: implement once the spawn value of a cell can be four
        todo!();
    }
}
