use rand::prelude::*;
use screen_printer::printer::*;
use std::{thread, time::Duration};

const WIDTH: usize = 10;
const HEIGHT: usize = 5;
const WAIT_TIME: u64 = 400;

/// Creates a random list of numbers, prints them, then clears the grid.
fn main() {
  println!("{}", "\n".repeat(HEIGHT + 1));

  let mut printer = Printer::with_printing_options(PrintingOptions::default());
  let mut rng = rand::thread_rng();
  let mut list_of_numbers: Vec<u8> = [0; WIDTH * HEIGHT].into(); //Vec::with_capacity(WIDTH * HEIGHT);

  for _ in 0..25 {
    for _ in 0..4 {
      // Update the grid data
      update_random_number_array(&mut rng, &mut list_of_numbers);

      // Create a grid with the data
      let grid =
        Printer::create_grid_from_full_character_list(&list_of_numbers, WIDTH, HEIGHT).unwrap();

      // Print the grid
      printer
        .dynamic_print(grid)
        .unwrap_or_else(|error| panic!("An error has occurred: '{error}'"));

      thread::sleep(Duration::from_millis(WAIT_TIME));
    }
  }
}

/// Retuns a list of random numbers 0-9
fn update_random_number_array(rng: &mut ThreadRng, number_array: &mut [u8]) {
  for num in number_array.iter_mut() {
    *num = rng.gen_range(0..9);
  }
}
