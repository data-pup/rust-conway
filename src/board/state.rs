use board::position::Position;

pub struct BoardState {
    pub dims:   Position,
    pub living: Vec<Position>,
}

#[cfg(test)]
mod tests {
    use board::state::BoardState;
    use board::position::Position;

    #[test]
    fn simple_board_test() {
        let b = BoardState {
            dims: Position { x:10, y:10 },
            living: [
                Position { x:0, y:0}
            ].to_vec(),
        };

        assert_eq!(b.dims.x, 10);
        assert_eq!(b.dims.y, 10);
        assert_eq!(b.living.len(), 1);

        let live_square = b.living[0].clone();
        assert_eq!(live_square.x, 0);
        assert_eq!(live_square.y, 0);
    }
}
