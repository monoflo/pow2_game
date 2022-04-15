#[derive(PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(PartialEq)]
/// The representation of each of game movement.
pub enum Move {
    Shift(Direction),
    Undo,
}
