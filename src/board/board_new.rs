use board::Board;
use board::position::BoardPosition;

impl<'a> Board<'a> {
    pub fn new(width: u32, height:u32) -> Board<'a> {
        let dims = BoardPosition::new(width, height);
        let living = [];
        return Board { dims:dims.clone(), living:&living.clone() };
    }
}
