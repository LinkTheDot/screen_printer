pub use crate::dynamic_printer::*;
use std::cmp::Ordering;
use std::fmt;
use std::{io, io::Write};

/// These are the possible ways the program can fail.
///
/// Each error will contain 'ErrorData' which holds the
/// expected and outcome results in the event of the error.
#[derive(Debug)]
pub enum PrintingError {
  /// In the context of creating a grid from a full list of characters
  TooManyCharacters(LengthErrorData),
  /// In the context of creating a grid from a full list of characters
  TooLittleCharacters(LengthErrorData),

  NonRectangularGrid,
  CursorError(String),
  GridLargerThanTerminal,
  GridDimensionsNotDefined,
  CursorPositionNotDefined,
  CouldntGetTerminalDimensions,
  GridIsLargerThanTerminal,
  MissingPrintingOptions,
}

impl fmt::Display for PrintingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{self:?}")
  }
}

/// The namespace for all methods used to create and print a grid
///
/// Also used to store data for the dynamic printing method
#[derive(Default, Debug)]
pub struct Printer {
  pub(crate) previous_grid: String,

  pub(crate) origin_position: Option<(usize, usize)>,

  pub(crate) grid_height: Option<usize>,
  pub(crate) grid_width: Option<usize>,

  pub printing_options: Option<PrintingOptions>,
}

/// Error data for when incorrect sizes are detected in a method
#[derive(Debug)]
pub struct LengthErrorData {
  pub expected_length: usize,
  pub got_length: usize,
}

impl LengthErrorData {
  /// Creates a new LengthErrorData for the expected length and actual length
  pub fn new(expected_length: usize, got_length: usize) -> Self {
    Self {
      expected_length,
      got_length,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct PrintingOptions {
  pub x_printing_option: XPrintingOption,
  pub y_printing_option: YPrintingOption,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum XPrintingOption {
  #[default]
  Left,
  Middle,
  Right,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum YPrintingOption {
  Top,
  Middle,
  #[default]
  Bottom,
}

impl PrintingOptions {
  pub fn new(x_printing_option: XPrintingOption, y_printing_option: YPrintingOption) -> Self {
    Self {
      x_printing_option,
      y_printing_option,
    }
  }
}

impl Printer {
  /// Creates a new printer for the [`dynamic_print()`](Printer::dynamic_print) method.
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self::new_printer(None)
  }

  /// Creates a new printer for the [`dynamic_print()`](Printer::dynamic_print) method with the given printing options.
  ///
  /// PrintingOptions tell the printer where to print any grids passed into it.
  /// Refer to [`PrintingOptions`](PrintingOptions) for more information;
  pub fn with_printing_options(printing_options: PrintingOptions) -> Self {
    Self::new_printer(Some(printing_options))
  }

  fn new_printer(printing_options: Option<PrintingOptions>) -> Self {
    Self {
      printing_options,
      ..Default::default()
    }
  }

  /// Creates a grid of the given size with the given character.
  ///
  /// # Example
  /// ```
  /// use screen_printer::printer::*;
  ///
  /// let character = 'a';
  /// let expected_grid = "aaa\naaa\naaa";
  ///
  /// let grid = Printer::create_grid_from_single_character(character, 3, 3);
  ///
  /// assert_eq!(expected_grid, grid);
  /// ```
  pub fn create_grid_from_single_character(character: char, width: usize, height: usize) -> String {
    // This was the fastest way I found to create a large 2-dimensional string of 1 character.
    let pixel_row = character.to_string().repeat(width) + "\n";
    let mut frame = pixel_row.repeat(height);
    frame.pop(); // remove new line

    frame
  }

  /// Creates a grid of the given size with the given list of characters
  ///
  /// An error is returned when the grid size doesn't match the amount of characters given
  ///
  /// # Example
  /// ```
  /// use screen_printer::printer::*;
  ///
  /// let characters = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"];
  /// let expected_grid = "abc\ndef\nghi";
  ///
  /// let grid = Printer::create_grid_from_full_character_list(&characters, 3, 3).unwrap();
  ///
  /// assert_eq!(expected_grid, grid);
  /// ```
  pub fn create_grid_from_full_character_list<T>(
    characters: &Vec<T>,
    width: usize,
    height: usize,
  ) -> Result<String, PrintingError>
  where
    T: fmt::Display,
  {
    let grid_size = width * height;

    match characters.len().cmp(&grid_size) {
      Ordering::Less => Err(PrintingError::TooLittleCharacters(LengthErrorData::new(
        characters.len(),
        grid_size,
      ))),
      Ordering::Greater => Err(PrintingError::TooManyCharacters(LengthErrorData::new(
        characters.len(),
        width * height,
      ))),
      Ordering::Equal => Ok(create_grid_from_characters(characters, width)),
    }
  }

  /// Moves the cursor up by the given height and prints the given grid.
  ///
  /// This is for printing over the previously printed grid.
  /// It's recommended to add some whitespace before your first print so the grid
  /// doesn't print into anything that was printed before this method was called.
  ///
  /// # Example
  /// ```
  /// use screen_printer::printer::*;
  ///
  /// let height = 10;
  /// let width = 10;
  /// let grid = Printer::create_grid_from_single_character('a', width, height);
  ///
  /// print!("{}", "\n".repeat(height + 5)); // add some space for the grid
  /// Printer::print_over_previous_grid(grid, height);
  /// ```
  pub fn print_over_previous_grid(grid: String, height: usize) {
    print!("\x1b[{};A", height - 1);
    print!("\r{grid}");
    let _ = io::stdout().flush();
  }

  pub fn get_grid_dimensions(&self) -> Result<(usize, usize), PrintingError> {
    let (Some(width), Some(height)) = (self.grid_width, self.grid_height) else {
      return Err(PrintingError::GridDimensionsNotDefined);
    };

    Ok((width, height))
  }

  pub fn assign_options(&mut self, printing_options: PrintingOptions) {
    self.printing_options = Some(printing_options)
  }

  pub(crate) fn get_origin_position(&self) -> Result<(usize, usize), PrintingError> {
    self
      .origin_position
      .ok_or(PrintingError::CursorPositionNotDefined)
  }
}

/// Creates a grid of the given width out of the given 1D array of characters.
fn create_grid_from_characters<T: fmt::Display>(characters: &[T], width: usize) -> String {
  characters
    .chunks(width)
    .map(|row| {
      row.iter().fold(String::new(), |mut row, character| {
        row.push_str(format!("{character}").as_str());

        row
      })
    })
    .collect::<Vec<String>>()
    .join("\n")
}
