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
        let new_living: Vec<BoardPosition> =
            if validate(&self) {  // If the board is valid, calculate new state.
                let update_cells = self.get_update_cells();
                Board::get_new_living(update_cells)
            } else {              // If the board isn't valid, re-initialize.
                Board::init_living(&self.dims)
            };
        self.living = new_living; // Assign the new state to the board.
    }

    /// This function will use the update information for each position and
    /// return a vector containing the positions of the cells that should be
    /// alive in the next tick.
    fn get_new_living(update_cells: Vec<Vec<UpdateCell>>) -> Vec<BoardPosition> {
        let mut new_living:Vec<BoardPosition> = vec![]; // Allocate return vector.
        let cell_lives = |update_info:UpdateCell| {     // This closure checks if
            match update_info.was_alive {               // a cell should be alive.
                true => 2 <= update_info.neighbors && update_info.neighbors <= 3,
                false => update_info.neighbors == 3,
            }
        };
        let mut y_u:usize = 0;
        for curr_row in update_cells {     // Iterate through all of the positions.
            let mut x_u:usize = 0;
            for curr_elem in curr_row {    // If the position should be alive in the
                if cell_lives(curr_elem) { // new state, add to the results vector.
                    new_living.push(BoardPosition {x:x_u as u32, y:y_u as u32});
                } x_u += 1; // Increment the x position after processing an element.
            } y_u += 1;     // Increment the x position after processing a row.
        }
        return new_living;
    }

    /// This function will use the current board state to create a matrix of
    /// objects containing information for identifying the new state of each cell.
    fn get_update_cells(&self) -> Vec<Vec<UpdateCell>> {
        let form_update_row = |row:&Vec<u32>| -> Vec<UpdateCell> {
            row.iter()
                .map(|&n_count| UpdateCell {was_alive:false, neighbors:n_count})
                .collect()
        };
        let mut update_cells: Vec<Vec<UpdateCell>> =
            self.get_neighbor_counts().iter()
            .map(|row| form_update_row(row))
            .collect();
        Board::set_was_alive_flags(&mut update_cells, &self.living);
        return update_cells
    }

    /// This will iterate through the vector of previously living positions,
    /// and the `was_alive` flag accordingly in the cell update matrix.
    fn set_was_alive_flags(update_cells: &mut Vec<Vec<UpdateCell>>,
                           prev_living:  &Vec<BoardPosition>) {
        for &BoardPosition{x, y} in prev_living {
            update_cells[y as usize][x as usize].was_alive = true;
        }
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
            .filter(|&pos| in_bounds(pos, dims_i))                    // Check pos is in bounds.
            .map   (|pos|  pos_from_tuple(pos))                       // Form a BoardPosition.
            .collect()                                                // Collect into a vector.
    }

    /// Allocates a matrix of u32 values that will be used to calculate the
    /// number of neighbors that each position on the board has.
    fn init_neighbor_count_matrix(dims:&BoardPosition) -> Vec<Vec<u32>> {
        let &BoardPosition {x:width, y:height} = dims;
        let (width_usize, height_usize) = (width as usize, height as usize);
        let new_living = vec![ vec![0; width_usize]; height_usize as usize ];
        return new_living;
    }

    /// This const contains relative steps to find each neighbor for a given position.
    const NEIGHBOR_COORDS:[(i32, i32); 8] = [
        (0, 1), (0, -1), (1, 0), (-1, 0),   // Up, down, right, left.
        (1, 1), (1, -1), (-1, 1), (-1, -1), // Diagonal coordinates.
    ];
}
