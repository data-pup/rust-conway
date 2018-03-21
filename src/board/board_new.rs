use board::Board;
use board::position::BoardPosition;
use rand;
use rand::Rng;

impl Board {
    pub fn new(width: u32, height:u32) -> Board {
        let dims = BoardPosition::new(width, height);
        let living = Board::init_living(&dims);
        return Board { dims:dims, living:living };
    }

    pub fn update(&mut self) {
        let living = Board::init_living(&self.dims);
        self.living = living;
    }

    fn init_living(dims:&BoardPosition) -> Vec<BoardPosition> {
        let &BoardPosition {x:width, y:height} = dims;
        let mut living = vec![];
        let mut rng = rand::thread_rng();
        for curr_x in 0..width {
            for curr_y in 0..height {
                if rng.gen() { &living.push(BoardPosition::new(curr_x, curr_y)); }
            }
        }
        return living;
    }
}
