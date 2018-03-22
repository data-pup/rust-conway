use board::Board;
use board::position::BoardPosition;
use board::validate::validate;

impl Board {
    /// Updates the board.
    pub fn update(&mut self) {
        if !validate(&self) { // If the board isn't valid, re-initialize.
            let living = Board::init_living(&self.dims);
            self.living = living;
        }

        // Destructure the board dimensions and borrow the vector of live squares.
        let orig_living = &self.living;
        let new_living = Board::get_neighbor_count_matrix(&self.dims);
        for pos in orig_living {
        }
    }

    const NEIGHBOR_COORDINATES:[(i32, i32); 8] = [
        (0, 1), (0, -1), (1, 0), (-1, 0), // Up, down, right, left.
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    fn get_neighbors(pos:&BoardPosition, dims:&BoardPosition) -> Vec<BoardPosition> {
        let mut neighbors = vec![];

        // Do Stuff.

        return neighbors;
    }

    fn get_neighbor_count_matrix(dims:&BoardPosition) -> Vec<Vec<u32>> {
        let &BoardPosition {x:width, y:height} = dims;
        let (width_usize, height_usize) = (width as usize, height as usize);
        let new_living = vec![ vec![0; width_usize]; height_usize as usize ];
        return new_living;
    }
}
