use rand::prelude::*;
use screen_printer::printer::*;

/// Creates a random list of numbers and prints them.
fn main() {
  print!("{}", termion::clear::All);
  let _cursor_hider = termion::cursor::HideCursor::from(std::io::stdout());

  // Create a printer with the options to print any grid in the center of the screen.
  let mut printer = Printer::new_with_printing_position(PrintingPosition::new(
    XPrintingPosition::Middle,
    YPrintingPosition::Middle,
  ));

  let mut rng = rand::thread_rng();

  for iteration in 1..=6 {
    let (width, height, mut list_of_numbers): (usize, usize, Vec<u8>) = if iteration % 2 == 0 {
      // Print a large grid of numbers.
      (200, 50, vec![0; 200 * 50])
    } else {
      // Print a smaller grid of numbers.
      (100, 25, vec![0; 100 * 25])
    };

    for _ in 0..5000 {
      // Update the grid data
      update_random_number_array(&mut rng, &mut list_of_numbers);

      // Create a grid with the data
      let grid =
        Printer::create_grid_from_full_character_list(&list_of_numbers, width, height).unwrap();

      // Print the grid
      printer
        .dynamic_print(grid)
        .unwrap_or_else(|error| panic!("An error has occurred: '{error}'"));
    }
  }

  print!("{}", termion::clear::All);
}

/// Retuns a list of random numbers 0-9
fn update_random_number_array(rng: &mut ThreadRng, number_array: &mut [u8]) {
  for num in number_array.iter_mut() {
    *num = rng.gen_range(0..9);
  }
}
