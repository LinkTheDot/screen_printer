use screen_printer::printer::*;
use std::{thread, time::Duration};

const WIDTH: usize = 5;
const HEIGHT: usize = 4;
/// In milliseconds.
const WAIT_TIME: u64 = 1000;

fn main() {
  print!("{}", "\n".repeat(10));

  grid_from_single_character();
  thread::sleep(Duration::from_millis(WAIT_TIME));

  grid_from_full_character_list();
  thread::sleep(Duration::from_millis(WAIT_TIME));
}

/// Prints a grid of 'a's
fn grid_from_single_character() {
  let grid = Printer::create_grid_from_single_character('a', WIDTH, HEIGHT);

  Printer::print_over_previous_grid(grid, HEIGHT);
}

/// Prints a grid from a-t
fn grid_from_full_character_list() {
  let character_list = vec![
    "a", "b", "c", "d", "e", //
    "f", "g", "h", "i", "j", //
    "k", "l", "m", "n", "o", //
    "p", "q", "r", "s", "t",
  ];

  let grid = Printer::create_grid_from_full_character_list(&character_list, WIDTH, HEIGHT).unwrap();

  Printer::print_over_previous_grid(grid, HEIGHT);
}
