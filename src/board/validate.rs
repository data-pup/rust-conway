use board::Board;
use board::position::BoardPosition;

fn validate(b:&Board) -> bool {
    let BoardPosition  { x:width,  y:height } = b.dims;
    for &BoardPosition { x:curr_x, y:curr_y } in b.living {
        if curr_x >= width || curr_y >= height { return false; }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use board::Board;
    use board::position::BoardPosition;
    use board::validate::validate;

    #[test]
    pub fn run_tests() {
        for case in test_cases.iter() { run_test(&case); }
    }

    fn run_test(test_case:&ValidateTestCase) {
        let &ValidateTestCase {ref dims, living, expected_result, desc} = test_case;
        let b = Board { dims:dims.clone(), living:living.clone() };
        let actual_result = validate(&b);
        assert_eq!(actual_result, expected_result, "Test Failed: {}", desc);
    }

    struct ValidateTestCase<'a> {
        dims:            BoardPosition,
        living:          &'a [BoardPosition],
        expected_result: bool,
        desc:            &'a str,
    }

    static test_cases:[ValidateTestCase; 4] = [
        ValidateTestCase {
            dims:   BoardPosition { x:10, y:10 },
            living: &[BoardPosition { x:0, y:0 }],
            expected_result: true,
            desc: "10x10 Board with a single living square at origin.",
        },
        ValidateTestCase {
            dims:   BoardPosition { x:5, y:5 },
            living: &[BoardPosition { x:5, y:5 }],
            expected_result: false,
            desc: "5x5 Board with a single living square completely out of bounds.",
        },
        ValidateTestCase {
            dims:   BoardPosition { x:5, y:5 },
            living: &[BoardPosition { x:5, y:4 }],
            expected_result: false,
            desc: "5x5 Board with a single living square out of bounds on the x axis.",
        },
        ValidateTestCase {
            dims:   BoardPosition { x:5, y:5 },
            living: &[BoardPosition { x:4, y:5 }],
            expected_result: false,
            desc: "5x5 Board with a single living square out of bounds on the y axis.",
        }
    ];
}
