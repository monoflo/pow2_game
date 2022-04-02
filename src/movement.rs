#[derive(PartialEq)]
/// The representation of each of game movement.
pub enum Move {
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    ShiftUp,
    Undo,
}
