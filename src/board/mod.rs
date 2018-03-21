pub mod position;
pub mod validate;

pub struct Board<'a> {
    pub dims:   position::BoardPosition,
    pub living: &'a [position::BoardPosition],
}

pub fn hello() {
    println!("Hello, world!");
}
