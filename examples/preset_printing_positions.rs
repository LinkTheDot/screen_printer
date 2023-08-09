// This example shows changing the printing options to the 9 possible positions, and printing a grid
// at each of them.

use screen_printer::printer::*;
use std::{thread, time::Duration};

const WIDTH: usize = 10;
const HEIGHT: usize = 5;
const WAIT_TIME: u64 = 400;

fn main() {
  print!("{}", termion::clear::All);
  let _cursor_hider = termion::cursor::HideCursor::from(std::io::stdout());

  let mut printer = Printer::new(); // Defaults with a printing position of Bottom Left.
  let grid_1 = Printer::create_grid_from_single_character('|', WIDTH, HEIGHT);
  let grid_2 = Printer::create_grid_from_single_character('=', WIDTH, HEIGHT);

  // Y defaults to bottom of the screen.
  print_grids_left_to_right(&mut printer, &grid_1, &grid_2);

  // Change Y to middle of the screen
  let _ = printer.replace_y_printing_position(YPrintingPosition::Middle);
  print_grids_left_to_right(&mut printer, &grid_1, &grid_2);

  // Change Y to top of the screen
  let _ = printer.replace_y_printing_position(YPrintingPosition::Top);
  print_grids_left_to_right(&mut printer, &grid_1, &grid_2);

  print!("{}", termion::clear::All);
}

/// Prints the grids Left, Middle, then Right on the screen
fn print_grids_left_to_right(printer: &mut Printer, grid_1: &str, grid_2: &str) {
  // Left
  let _ = printer.replace_x_printing_position(XPrintingPosition::Left);
  print_grids(printer, grid_1, grid_2);

  // Middle
  let _ = printer.replace_x_printing_position(XPrintingPosition::Middle);
  print_grids(printer, grid_1, grid_2);

  // Right
  let _ = printer.replace_x_printing_position(XPrintingPosition::Right);
  print_grids(printer, grid_1, grid_2);
}

/// Prints both passed in grids one after after the other.
fn print_grids(printer: &mut Printer, grid_1: &str, grid_2: &str) {
  printer.dynamic_print(grid_1.to_owned()).unwrap();
  thread::sleep(Duration::from_millis(WAIT_TIME));

  printer.dynamic_print(grid_2.to_owned()).unwrap();
  thread::sleep(Duration::from_millis(WAIT_TIME));
}
