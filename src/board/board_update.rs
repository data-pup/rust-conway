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

        let neigh_counts = self.get_neighbor_counts();
    }

    /// Creates a matrix of unsigned integers representing the number of
    /// living neighbors at each position on the board.
    fn get_neighbor_counts(&self) -> Vec<Vec<u32>> {
        self.living.iter()
        .flat_map(|pos| Board::get_pos_neighbors(pos, &self.dims))
        .fold(Board::init_neighbor_count_matrix(&self.dims),
            |mut counts, BoardPosition{x, y}| {
                let (x_u, y_u) = (x as usize, y as usize);
                counts[y_u][x_u] += 1;
                counts
            }
        )
    }

    /// Map the position relative neighbor coordinate collections, filter out
    /// dimensions that are out of bounds, and collect the results into a
    /// vector of board position objects.
    fn get_pos_neighbors(pos:&BoardPosition, dims:&BoardPosition) -> Vec<BoardPosition> {
        let (x, y) = (*&pos.x as i32, *&pos.y as i32);
        let (width, height) = (*&dims.x as i32, *&dims.y as i32);
        let neighbors = Board::NEIGHBOR_COORDS.iter()
            .map(|&(x_delta, y_delta)| (x as i32 + x_delta, y as i32 + y_delta))
            .filter(
                |&(neigh_x, neigh_y)| neigh_x >= 0 && neigh_x < width  as i32
                                   && neigh_y >= 0 && neigh_y < height as i32)
            .map(|(new_x, new_y)| BoardPosition { x:new_x as u32, y:new_y as u32 })
            .collect();
        return neighbors;
    }

    /// This const contains relative steps to find each neighbor for a given position.
    const NEIGHBOR_COORDS:[(i32, i32); 8] = [
        (0, 1), (0, -1), (1, 0), (-1, 0),   // Up, down, right, left.
        (1, 1), (1, -1), (-1, 1), (-1, -1), // Diagonal coordinates.
    ];

    /// Allocates a matrix of u32 values that will be used to calculate the
    /// number of neighbors that each position on the board has.
    fn init_neighbor_count_matrix(dims:&BoardPosition) -> Vec<Vec<u32>> {
        let &BoardPosition {x:width, y:height} = dims;
        let (width_usize, height_usize) = (width as usize, height as usize);
        let new_living = vec![ vec![0; width_usize]; height_usize as usize ];
        return new_living;
    }
}
