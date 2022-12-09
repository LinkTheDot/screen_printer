use screen_printer::printer::*;
use std::{thread, time::Duration};

const WIDTH: usize = 5;
const HEIGHT: usize = 4;
const WAIT_TIME: u64 = 1000;

fn main() {
  print!("{}", "\n".repeat(10));

  grid_from_single_character();
  thread::sleep(Duration::from_millis(WAIT_TIME));

  grid_from_single_row();
  thread::sleep(Duration::from_millis(WAIT_TIME));

  grid_from_multiple_rows();
  thread::sleep(Duration::from_millis(WAIT_TIME));

  grid_from_full_character_list();
  thread::sleep(Duration::from_millis(WAIT_TIME));
}

/// Prints a grid of 'a's
fn grid_from_single_character() {
  let character = "a";

  let grid = Printer::create_grid_from_single_character(&character, WIDTH, HEIGHT);

  Printer::print_over_previous_grid(grid, HEIGHT);
}

/// Prints a grid with rows of '-|-|-'
fn grid_from_single_row() {
  let row = "-|-|-";

  let grid = Printer::create_grid_from_single_row(&row, HEIGHT);

  Printer::print_over_previous_grid(grid, HEIGHT);
}

fn grid_from_multiple_rows() {
  let rows = vec![
    "aaaaa".to_string(),
    "bbbbb".to_string(),
    "ccccc".to_string(),
    "ddddd".to_string(),
  ];

  let grid = Printer::create_grid_from_multiple_rows(&rows).unwrap();

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
