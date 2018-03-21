extern crate rand;

mod board;

fn main() {
    let b = board::Board::new(80, 30);
    print!("{}", b.to_string());
}
