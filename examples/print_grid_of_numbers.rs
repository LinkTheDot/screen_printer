use rand::prelude::*;
use std::{fmt, thread, time::Duration};
use terminal_printing::*;

pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 20;

#[derive(Debug, PartialEq)]
pub struct ObjectData {
  pub number: u32,
}

impl fmt::Display for ObjectData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.number)
  }
}

impl ObjectData {
  fn new(number: u32) -> Self {
    Self { number }
  }
}

fn main() {
  println!("{}", "\n".repeat(GRID_HEIGHT * 2));

  loop {
    let grid = get_2d_grid_of_random_numbers();
    let printable_grid =
      Printer::create_grid_from_full_character_list(&grid, GRID_WIDTH, GRID_HEIGHT).unwrap();

    thread::sleep(Duration::from_millis(25)); // wait time in between prints
    Printer::print_over_previous_grid(printable_grid, GRID_HEIGHT);
  }
}

fn get_2d_grid_of_random_numbers() -> Vec<ObjectData> {
  let mut rng = rand::thread_rng();

  (0..(GRID_WIDTH * GRID_HEIGHT)) //
    .fold(Vec::new(), |mut data_vec, _| {
      let number = rng.gen_range(0..9);

      data_vec.push(ObjectData::new(number));

      data_vec
    })
}
