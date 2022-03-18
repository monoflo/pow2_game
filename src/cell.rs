#[derive(Copy, Clone, Debug)]
pub struct Cell {
    value: usize,
}

impl Cell {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn grow(&mut self) {
        assert_ne!(0, self.value);
        self.value <<= 1;
    }

    // TODO: make 4's spawn with chance 10%
    pub fn spawn(&mut self) {
        self.value = 2;
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
        assert_eq!(0, cell.value());
    }

    #[test]
    fn spawn() {
        let mut cell = Cell::new();
        cell.spawn();

        let val = cell.value();
        assert!(val == 2 || val == 4);
    }

    #[test]
    fn grow() {
        let mut cell = Cell::new();
        cell.spawn();
        let val = cell.value();
        cell.grow();

        assert_eq!(val * 2, cell.value());
    }

}
