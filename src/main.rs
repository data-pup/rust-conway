extern crate rand;

mod board;
use board::Board;
use std::time::Duration;
use std::thread::sleep;

fn main() { // Begin by initializing the board.
    let delay: u64 = 100;
    let mut counter: u64 = 0;
    let (width, height) = (80, 30);
    let mut board = Board::new(width, height);
    loop { // Loop indefinitely. Clear screen, print, wait, and update.
        clear_screen();
        print_board(&board, counter);
        wait_n_ms(delay);
        board.update();
        counter += 1;
    }
}

/// Clear the screen.
fn clear_screen() {
    print!("{}[2J", 27 as char);
}

/// Print the board and the iteration number.
fn print_board(b:&Board, c:u64) {
    print!("Iteration: {0}\n\n{1}",
        c.to_string(),
        &b.to_string(),
    );
}

/// Wait a given number of milliseconds.
fn wait_n_ms(n:u64) {
    sleep(Duration::from_millis(n));
}
