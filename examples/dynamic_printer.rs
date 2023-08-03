use rand::prelude::*;
use screen_printer::printer::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 50;

/// Creates a random list of numbers, prints them, then clears the grid.
fn main() {
  print!("{}", termion::clear::All);
  let _cursor_hider = termion::cursor::HideCursor::from(std::io::stdout());

  // Create a printer with the options to print any grid in the center of the screen.
  let mut printer = Printer::new_with_printing_position(PrintingPosition::new(
    XPrintingPosition::Middle,
    YPrintingPosition::Middle,
  ));

  let mut rng = rand::thread_rng();
  let mut list_of_numbers: Vec<u8> = [0; WIDTH * HEIGHT].into(); //Vec::with_capacity(WIDTH * HEIGHT);

  for _ in 0..10000 {
    // Update the grid data
    update_random_number_array(&mut rng, &mut list_of_numbers);

    // Create a grid with the data
    let grid =
      Printer::create_grid_from_full_character_list(&list_of_numbers, WIDTH, HEIGHT).unwrap();

    // Print the grid
    printer
      .dynamic_print(grid)
      .unwrap_or_else(|error| panic!("An error has occurred: '{error}'"));
  }

  print!("{}", termion::clear::All);
}

/// Retuns a list of random numbers 0-9
fn update_random_number_array(rng: &mut ThreadRng, number_array: &mut [u8]) {
  for num in number_array.iter_mut() {
    *num = rng.gen_range(0..9);
  }
}
