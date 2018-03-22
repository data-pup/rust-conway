pub mod position;
pub mod validate;
pub mod board_to_string;
pub mod board_update;

use rand;
use rand::Rng;

use board::position::BoardPosition;

pub struct Board {
    pub dims:   BoardPosition,
    pub living: Vec<BoardPosition>,
}

impl Board {
    /// Create a new Conway's Game of Life board of a given height and width.
    pub fn new(width: u32, height:u32) -> Board {
        let dims = BoardPosition::new(width, height);
        let living = Board::init_living(&dims);
        return Board { dims:dims, living:living };
    }

    /// Initializes the board, randomly generating which squares should begin
    /// in a living state. Returns a vector containing the positions of each
    /// living square on the board.
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