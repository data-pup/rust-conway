mod board_state {
    pub struct Position {
        pub x: u32,
        pub y: u32,
    }

    pub struct BoardState {
        pub height: u32,
        pub width:  u32,
        pub living: Vec<u32>,
    }
}
