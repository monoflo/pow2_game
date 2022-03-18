use std::fmt;


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
        self.value <<= 1;
    }

    // TODO: make 4's spawn with chance 10%
    pub fn spawn(&mut self) {
        self.value = 2;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
