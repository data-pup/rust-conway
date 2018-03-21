extern crate rand;

mod board;
use board::Board;
use std::time::Duration;
use std::thread::sleep;

fn main() { // Begin by initializing the board.
    let (width, height) = (80, 30);
    let mut b = Board::new(width, height);
    loop { // Loop indefinitely. Clear screen, print, wait, and update.
        clear_screen();
        print_board(&b);
        wait_n_secs(1);
        b.update();
    }
}

// Helper functions.
fn clear_screen()        { print!("{}[2J", 27 as char);  }
fn print_board(b:&Board) { print!("{}", &b.to_string()); }
fn wait_n_secs(n:u64)    { sleep(Duration::new(n, 0));   }
