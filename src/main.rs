extern crate rand;

mod board;

fn main() {
    let b = board::Board::new(80, 30);
    // print!("{}[2J", 27 as char); // Clear the screen?
    print!("{}", b.to_string());
}
