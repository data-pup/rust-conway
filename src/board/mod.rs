pub mod position;
pub mod validate;
pub mod board_to_string;
pub mod board_new;
// pub mod update;

use board::position::BoardPosition;

pub struct Board {
    pub dims:   BoardPosition,
    pub living: Vec<BoardPosition>,
}
