pub fn hello() {
    println!("Hello, world!");
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct BoardState {
    pub dims:   Position,
    pub living: Vec<Position>,
}

#[cfg(test)]
mod tests {
    use board_state::*;

    #[test]
    fn test_positions() {
        let p = Position { x:0, y:0 };
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn simple_board_tests() {
        let b = BoardState {
            dims: Position { x:10, y:10 },
            living: [
                Position { x:0, y:0}
            ],
        };
    }
}
