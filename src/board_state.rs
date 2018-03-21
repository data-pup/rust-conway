mod board_state {
    pub struct Position {
        pub x: u32,
        pub y: u32,
    }

    pub struct BoardState {
        pub dims:   Position,
        pub living: Vec<Position>,
    }
}
