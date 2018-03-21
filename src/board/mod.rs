pub mod position;

pub struct Board {
    pub dims:   position::BoardPosition,
    pub living: Vec<position::BoardPosition>,
}

pub fn hello() {
    println!("Hello, world!");
}
