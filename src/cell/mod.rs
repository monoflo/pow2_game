use rand::Rng;

/// The representation of a cell on the game board.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell(usize);

/// Implementation of the `Display` trait for `Cell`.
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

impl Cell {
    /// Returns the value held by the cell.
    #[inline(always)]
    pub fn value(&self) -> usize {
        self.0
    }

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

#[cfg(test)]
mod test;
