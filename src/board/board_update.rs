use board::Board;
use board::position::BoardPosition;
use board::validate::validate;

struct UpdateCell {
    was_alive: bool,
    neighbors: u32,
}

impl Board {
    /// Updates the board.
    pub fn update(&mut self) {
        if !validate(&self) { // If the board isn't valid, re-initialize.
            let living = Board::init_living(&self.dims);
            self.living = living;
            return;
        }
        let cell_lives = |update_info:&UpdateCell| {
            match update_info.was_alive {
                true => 2 <= update_info.neighbors && update_info.neighbors <= 3,
                false => update_info.neighbors == 3,
            }
        };
        let update_cells = self.get_update_cells();
        let mut new_living:Vec<BoardPosition> = vec![];
        let BoardPosition {x:width, y:height} = self.dims;
        for x in 0..width { for y in 0..height {
                if cell_lives(&update_cells[y as usize][x as usize]) {
                    new_living.push(BoardPosition {x, y});
                }
            }
        }
        self.living = new_living;
    }

    fn get_update_cells(&self) -> Vec<Vec<UpdateCell>> {
        let form_update_row = |row:&Vec<u32>| -> Vec<UpdateCell> {
            row.iter()
                .map(|&n_count| UpdateCell {
                    was_alive:false,
                    neighbors:n_count})
                .collect()
        };
        let mut update_cells: Vec<Vec<UpdateCell>> =
            self.get_neighbor_counts().iter()
            .map(|row| form_update_row(row))
            .collect();
        for &BoardPosition{x, y} in &self.living {
            update_cells[y as usize][x as usize].was_alive = true;
        }
        update_cells
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
        // Destructure the position and dimensions into signed integers.
        let (x, y) = (*&pos.x as i32, *&pos.y as i32);
        let dims_i = (*&dims.x as i32, *&dims.y as i32);

        // Closures used for bounds checking positions and creating a BoardPosition.
        let in_bounds = |(x,y):(i32, i32), (width, height):(i32, i32)| -> bool {
            return x >= 0 && x < width && y >= 0 && y < height
        };
        let pos_from_tuple = |(x_i, y_i):(i32, i32)| BoardPosition { x:x_i as u32, y:y_i as u32 };

        Board::NEIGHBOR_COORDS.iter()                                 // Iterate through neighbors.
            .map   (|&(x_delta, y_delta)| (x + x_delta, y + y_delta)) // Find neighbor coords.
            .filter(|&pos| in_bounds(pos, dims_i))                    // Check is in bounds.
            .map   (|pos|  pos_from_tuple(pos))                       // Form a BoardPosition.
            .collect()                                                // Collect into a vector.
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
