// implement "dynamic printing"
// a concept where you store the rows of the previous print
// only printing over the rows that were changed

use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug)]
pub enum PrintingError {
  RowTooLong(ErrorData),
  RowTooShort(ErrorData),
  TooManyCharacters(ErrorData),
  TooLittleCharacters(ErrorData),
  RowsDontMatchLength,
}

pub struct Printer;

#[derive(Debug)]
pub struct ErrorData {
  pub expected_length: usize,
  pub got_length: usize,
}

impl ErrorData {
  #[allow(clippy::new_without_default)]
  pub fn new(expected_length: usize, got_length: usize) -> Self {
    Self {
      expected_length,
      got_length,
    }
  }
}

impl Printer {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {}
  }

  pub fn create_grid_from_single_character<T>(
    character: &T,
    width: usize,
    height: usize,
  ) -> Result<String, PrintingError>
  where
    T: Display,
  {
    let row = Self::get_row_of_character(character, width);

    Self::create_grid_from_single_row(&row, width, height)
  }

  pub fn create_grid_from_single_row<T>(
    row: &T,
    width: usize,
    height: usize,
  ) -> Result<String, PrintingError>
  where
    T: Display,
  {
    let row = format!("{}", row);
    let row_size = row.chars().count();

    match row_size.cmp(&width) {
      Ordering::Less => Err(PrintingError::RowTooShort(ErrorData::new(row_size, width))),
      Ordering::Greater => Err(PrintingError::RowTooLong(ErrorData::new(row_size, width))),
      Ordering::Equal => Ok(
        (0..height)
          .fold(String::new(), |screen, _| format!("{}\n{}", screen, row))
          .trim()
          .to_string(),
      ),
    }
  }

  pub fn create_grid_from_full_character_list<T>(
    characters: &Vec<T>,
    width: usize,
    height: usize,
  ) -> Result<String, PrintingError>
  where
    T: Display,
  {
    let grid_size = width * height;

    match characters.len().cmp(&grid_size) {
      Ordering::Less => Err(PrintingError::TooLittleCharacters(ErrorData::new(
        characters.len(),
        grid_size,
      ))),
      Ordering::Greater => Err(PrintingError::TooManyCharacters(ErrorData::new(
        characters.len(),
        width * height,
      ))),
      Ordering::Equal => Ok(create_grid_from_characters(characters, width)),
    }
  }

  pub fn create_grid_from_multiple_rows<T>(
    rows: &[T],
    width: usize,
  ) -> Result<String, PrintingError>
  where
    T: Display,
  {
    let rows: Vec<String> = rows.iter().map(|row| format!("{}", row)).collect();

    if Self::rows_have_same_lengths(&rows, width) {
      Ok(rows.join("\n"))
    } else {
      Err(PrintingError::RowsDontMatchLength)
    }
  }

  pub fn print_over_previous_grid(grid: String, height: usize) {
    print!("\x1b[{};A", height);
    print!("\r{}", grid);
  }

  fn get_row_of_character<T>(character: &T, width: usize) -> String
  where
    T: Display,
  {
    (0..width).fold(String::new(), |row, _| format!("{}{}", row, character))
  }

  fn rows_have_same_lengths(rows: &[String], expected_width: usize) -> bool {
    rows.iter().all(|row| row.chars().count() == expected_width)
  }
}

fn create_grid_from_characters<T>(characters: &[T], width: usize) -> String
where
  T: Display,
{
  characters
    .chunks(width)
    .map(|row| {
      row
        .iter()
        .map(|character| format!("{}", character))
        .collect::<String>()
        + "\n"
    })
    .collect()
}
