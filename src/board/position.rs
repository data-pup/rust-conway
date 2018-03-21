pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl Clone for BoardPosition {
    fn clone(&self) -> BoardPosition {
        return BoardPosition { x:self.x, y:self.y };
    }
}
