pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Clone for Position {
    fn clone(&self) -> Position {
        return Position { x:self.x, y:self.y };
    }
}

#[cfg(test)]
mod tests {
    use board::position::*;

    #[test]
    fn test_positions() {
        let p = Position { x:0, y:0 };
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }

}
