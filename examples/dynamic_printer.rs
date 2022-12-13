use rand::prelude::*;
use screen_printer::printer::*;
use std::{thread, time::Duration};

const WIDTH: usize = 10;
const HEIGHT: usize = 5;
const WAIT_TIME: u64 = 400;

/// Creates a random list of numbers, prints them, then clears the grid.
fn main() {
  println!("{}", "\n".repeat(HEIGHT + 1));

  let mut printer = Printer::new(WIDTH, HEIGHT);

  for _ in 0..25 {
    for _ in 0..4 {
      // Get the grid data
      let number_array = get_random_number_array(WIDTH * HEIGHT);
      // Create the grid with the data
      let grid =
        Printer::create_grid_from_full_character_list(&number_array, WIDTH, HEIGHT).unwrap();

      // Print the grid
      printer
        .dynamic_print(grid)
        .unwrap_or_else(|error| panic!("An error has occurred: '{error}'"));

      thread::sleep(Duration::from_millis(WAIT_TIME));
    }

    // Clear the grid every 4 prints for a flashing effect
    printer.clear_grid().unwrap();
    thread::sleep(Duration::from_millis(WAIT_TIME / 2));
  }
}

/// Retuns a list of random numbers 0-9
fn get_random_number_array(total_size: usize) -> Vec<u16> {
  let mut rng = rand::thread_rng();

  (0..total_size).fold(Vec::new(), |mut number_array, _| {
    number_array.push(rng.gen_range(0..9));

    number_array
  })
}
