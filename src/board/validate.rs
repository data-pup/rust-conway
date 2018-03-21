use board::Board;
use board::position::BoardPosition;

fn validate(b:&Board) -> bool {
    return true;
}

#[cfg(test)]
mod tests {
    use board::Board;
    use board::position::BoardPosition;
    use board::validate::validate;

    struct ValidateTestCase<'a> {
        dims:            BoardPosition,
        living:          &'a [BoardPosition],
        expected_result: bool,
        desc:            &'a str,
    }

    static test_cases:[ValidateTestCase; 1] = [
        ValidateTestCase {
            dims:   BoardPosition { x:10, y:10 },
            living: &[
                BoardPosition { x:0, y:0 },
            ],
            expected_result: true,
            desc: "10x10 Board with a single living square at origin.",
        }
    ];

    fn run_test(test_case:&ValidateTestCase) {
        let &ValidateTestCase {ref dims, living, expected_result, desc} = test_case;
        let b = Board { dims:dims.clone(), living:living.clone() };
        let actual_result = validate(&b);
        assert_eq!(actual_result, expected_result, "Test Failed: {}", desc);
    }

    #[test]
    pub fn run_tests() {
        for case in test_cases.iter() { run_test(&case) }
    }
}
