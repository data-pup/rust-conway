use board::Board;
use board::position::BoardPosition;
use board::validate::validate;

/// Creates a string representation of the current board state. If the board
/// is not in a valid state, then this function will panic.
impl<'a> ToString for Board {
    fn to_string(&self) -> String {
        // Check that the board is valid first.
        if !validate(&self) { panic!("Could not create string for invalid Board!"); }

        // Create a 2-dimensional array of characters.
        let BoardPosition {x:width, y:height} = self.dims;
        let (x_usize, y_usize) = (width as usize, height as usize);
        let mut chars = vec![vec![' '; x_usize]; y_usize];

        // Change the character stored at each of the living squares.
        for &BoardPosition {x:curr_x, y:curr_y} in &self.living {
            let x_index = curr_x as usize;
            let y_index = (height - curr_y - 1) as usize;
            chars[y_index][x_index] = 'X';
        }

        return chars.iter()         // Fold each row vector into a string,
            .map(|row| row.iter()   // join the rows together and return.
                .fold(String::new(), |row_s, val| row_s + &val.to_string()))
            .fold(String::new(), |board_s, row_s| board_s + &row_s + "\n");
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use board::position::BoardPosition;

    #[test]
    /// This function will run each of the test cases, and print the test's
    /// description if a test fails.
    pub fn run_tests() {
        for case in TEST_CASES.iter() { run_test(&case); }
    }

    /// Private helper function used to test a single test case.
    fn run_test(test_case:&ToStringTestCase) {
        let &ToStringTestCase { ref dims, living, expected_string, desc } = test_case;
        let b = Board { dims:dims.clone(), living:living.clone() };
        let actual_string = b.to_string();
        assert_eq!(actual_string, expected_string, "Test Failed: {}", desc);
    }

    // Test case type containing board dimensions, living squares positions,
    // expected string representation, and a test description field.
    struct ToStringTestCase<'a> {
        dims:            BoardPosition,
        living:          &'a [BoardPosition],
        expected_string: &'a str,
        desc:            &'a str,
    }

    // Test cases for the board.
    static TEST_CASES:[ToStringTestCase; 2] = [
        ToStringTestCase {
            dims: BoardPosition { x:3, y:3 },
            living: &[BoardPosition { x:0, y:0}],
            expected_string: "   \n   \nX  \n",
            desc: "3x3 Board with single living square at origin.",
        },
        ToStringTestCase {
            dims: BoardPosition { x:3, y:3 },
            living: &[
                BoardPosition { x:0, y:0},
                BoardPosition { x:0, y:2},
                BoardPosition { x:2, y:0},
                BoardPosition { x:2, y:2},
            ],
            expected_string: "X X\n   \nX X\n",
            desc: "3x3 Board with living squares at each corner.",
        },
    ];
}
