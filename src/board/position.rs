/// Represents a position on the Conway's Game of Life Board.
/// BoardPosition.x: The coordinate's position on the x axis.
/// BoardPosition.y: The coordinate's position on the y axis.
pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl BoardPosition {
    pub fn new(x:u32, y:u32) -> BoardPosition {
        return BoardPosition { x:x, y:y };
    }
}

impl Clone for BoardPosition {
    fn clone(&self) -> BoardPosition {
        return BoardPosition { x:self.x, y:self.y };
    }
}
