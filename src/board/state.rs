use board::position::BoardPosition;

pub struct BoardState {
    pub dims:   BoardPosition,
    pub living: Vec<BoardPosition>,
}
