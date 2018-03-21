pub mod position;
pub mod validate;
pub mod board_to_string;
pub mod board_new;

use board::position::BoardPosition;

pub struct Board<'a> {
    pub dims:   BoardPosition,
    pub living: &'a [BoardPosition],
}

pub fn hello() {
    println!("Hello, world!");
}
