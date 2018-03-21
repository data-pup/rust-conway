use board::Board;
use board::position::BoardPosition;

impl<'a> ToString for Board<'a> {
    fn to_string(&self) -> String {
        // Create a 2-dimensional array of characters.
        let BoardPosition {x:width, y:height} = self.dims;
        let (x_usize, y_usize) = (width as usize, height as usize);
        let mut chars = vec![vec![' '; x_usize]; y_usize];

        // Change the character stored at each of the living squares.
        for &BoardPosition {x:curr_x, y:curr_y} in self.living {
            chars[curr_y as usize][curr_x as usize] = 'X';
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
    pub fn run_tests() {
        for case in test_cases.iter() { run_test(&case); }
    }

    fn run_test(test_case:&ToStringTestCase) {
        let &ToStringTestCase { ref dims, living, expected_string, desc } = test_case;
        let b = Board { dims:dims.clone(), living:living.clone() };
        let actual_string = b.to_string();
        assert_eq!(actual_string, expected_string, "Test Failed: {}", desc);
    }

    struct ToStringTestCase<'a> {
        dims:            BoardPosition,
        living:          &'a [BoardPosition],
        expected_string: &'a str,
        desc:            &'a str,
    }

    static test_cases:[ToStringTestCase; 1] = [
        ToStringTestCase {
            dims: BoardPosition { x:3, y:3 },
            living: &[BoardPosition { x:0, y:0}],
            expected_string: "   \n   \nX  ",
            desc: "3x3 Board with single living square at origin.",
        },
    ];
}
