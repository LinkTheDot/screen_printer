// Change printing options to PrintingPositions

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
  /// When creating a grid, the defined size of the grid was larger than the given amount of characters.
  TooManyCharacters(LengthErrorData),
  /// When creating a grid, the defined size of the grid was smaller than the given amount of characters.
  TooLittleCharacters(LengthErrorData),

  /// A grid was passed in and wasn't in a 2d shape.
  ///
  /// Grids are stored as a 1d string, but treated as a 2d shape.
  /// Each column is a character, and each row is a new line.
  NonRectangularGrid,

  /// When no [`printing positions`](PrintingPosition) are defined, the printer will attempt to read the
  /// position of the cursor to print the grid.
  /// This error is returned when getting the position of the cursor failed.
  ///
  /// The error message is contained.
  CursorError(String),

  /// When attempting to get the dimensions of the terminal, an error occurred.
  ///
  /// The error message is contained
  FailedToGetTerminalDimensions(String),
  /// This error is returned when a grid passed in to [`dynamic_print`](Printer::dynamic_print) is
  /// larger than the dimensions of the terminal.
  GridLargerThanTerminal,

  /// When attempting to get the dimensions of the grid through [`get_grid_dimensions`](Printer::get_grid_dimensions),
  /// there was no stored dimensions for the grid.
  GridDimensionsNotDefined,
  /// When attempting to get the origin position of the printer through [`get_origin_position`](Printer::get_origin_position), there was no stored position for the grid.
  CursorPositionNotDefined,

  /// There was no [`PrintingPosition`](PrintingPosition) when attempting to get origin from printing position.
  MissingPrintingPosition,
}

impl fmt::Display for PrintingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{self:?}")
  }
}

// Define how to use the printer
#[derive(Default, Debug)]
pub struct Printer {
  pub(crate) previous_grid: String,

  pub(crate) origin_position: Option<(usize, usize)>,

  pub(crate) grid_height: Option<usize>,
  pub(crate) grid_width: Option<usize>,

  printing_position: Option<PrintingPosition>,
  pub(crate) printing_position_changed_since_last_print: bool,
}

/// Error data for when attempting to compare strings of differing lengths.
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
pub struct PrintingPosition {
  pub x_printing_position: XPrintingPosition,
  pub y_printing_position: YPrintingPosition,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum XPrintingPosition {
  #[default]
  Left,
  Middle,
  Right,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum YPrintingPosition {
  Top,
  Middle,
  #[default]
  Bottom,
}

impl PrintingPosition {
  pub fn new(
    x_printing_position: XPrintingPosition,
    y_printing_position: YPrintingPosition,
  ) -> Self {
    Self {
      x_printing_position,
      y_printing_position,
    }
  }

  pub fn with_x_printing_position(x_printing_position: XPrintingPosition) -> Self {
    Self {
      x_printing_position,
      ..Default::default()
    }
  }

  pub fn with_y_printing_position(y_printing_position: YPrintingPosition) -> Self {
    Self {
      y_printing_position,
      ..Default::default()
    }
  }
}

impl Printer {
  /// Creates a new printer for the [`dynamic_print()`](Printer::dynamic_print) method.
  pub fn new() -> Self {
    Self::new_printer(None)
  }

  /// Creates a new printer for the [`dynamic_print()`](Printer::dynamic_print) method with the given printing position.
  ///
  /// PrintingPositons tell the printer where to print any grids passed into it.
  /// Refer to [`PrintingPosition`](PrintingPosition) for more information;
  pub fn new_with_printing_position(printing_position: PrintingPosition) -> Self {
    Self::new_printer(Some(printing_position))
  }

  /// Creates a new printer with the given optional Position.
  fn new_printer(printing_position: Option<PrintingPosition>) -> Self {
    Self {
      printing_position,
      ..Default::default()
    }
  }

  pub fn replace_printing_position(&mut self, printing_position: PrintingPosition) {
    self.printing_position = Some(printing_position);
    self.printing_position_changed_since_last_print = true;
  }

  /// # Errors
  ///
  /// - There is no defined printing position
  pub fn replace_x_printing_position(
    &mut self,
    new_x_printing_position: XPrintingPosition,
  ) -> Result<(), PrintingError> {
    let Some(printing_position) = &mut self.printing_position else {
      return Err(PrintingError::MissingPrintingPosition);
    };

    printing_position.x_printing_position = new_x_printing_position;
    self.printing_position_changed_since_last_print = true;

    Ok(())
  }

  /// # Errors
  ///
  /// - There is no defined printing position
  pub fn replace_y_printing_position(
    &mut self,
    new_y_printing_position: YPrintingPosition,
  ) -> Result<(), PrintingError> {
    let Some(printing_position) = &mut self.printing_position else {
      return Err(PrintingError::MissingPrintingPosition);
    };

    printing_position.y_printing_position = new_y_printing_position;
    self.printing_position_changed_since_last_print = true;

    Ok(())
  }

  pub fn get_current_printing_position(&self) -> Option<&PrintingPosition> {
    self.printing_position.as_ref()
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
  ///
  /// # Errors
  ///
  /// - When the amount of characters passed in doesn't fit the expected grid dimensions.
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
        grid_size,
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

  /// Returns the currently stored grid's dimensions.
  ///
  /// If no dimensions have been defined, or there's no stored grid, an error is returned.
  ///
  /// # Errors
  ///
  /// - When no dimensions have been defined.
  pub(crate) fn get_grid_dimensions(&self) -> Result<(usize, usize), PrintingError> {
    let (Some(width), Some(height)) = (self.grid_width, self.grid_height) else {
      return Err(PrintingError::GridDimensionsNotDefined);
    };

    Ok((width, height))
  }

  // /// Assign the passed in [`PrintingPosition`](PrintingPosition) for the printer.
  // pub fn assign_printing_position(&mut self, printing_position: PrintingPosition) {
  //   self.printing_position = Some(printing_position)
  // }

  /// Returns the currently stored origin positions.
  ///
  /// If no position has been defined, an error is returned.
  ///
  /// # Errors
  ///
  /// - When no origin has been defined.
  pub(crate) fn get_origin_position(&self) -> Result<(usize, usize), PrintingError> {
    self
      .origin_position
      .ok_or(PrintingError::CursorPositionNotDefined)
  }

  pub(crate) fn valid_rectangle_check(
    rectangle_shape: &str,
  ) -> Result<(usize, usize), PrintingError> {
    if rectangle_shape.is_empty() {
      return Ok((0, 0));
    }

    let rows: Vec<&str> = rectangle_shape.split('\n').collect();
    let model_width = rows[0].chars().count();

    let rows_have_same_lengths = rows.iter().all(|row| row.chars().count() == model_width);

    if rows_have_same_lengths {
      Ok((model_width, rows.len()))
    } else {
      Err(PrintingError::NonRectangularGrid)
    }
  }

  pub(crate) fn get_terminal_dimensions() -> Result<(usize, usize), PrintingError> {
    match termion::terminal_size() {
      Ok(terminal_dimensions) => Ok((
        terminal_dimensions.0 as usize,
        terminal_dimensions.1 as usize,
      )),
      Err(io_error) => Err(PrintingError::FailedToGetTerminalDimensions(
        io_error.to_string(),
      )),
    }
  }

  /// Resets all data for the printer.
  pub fn reset(&mut self) {
    *self = Printer::default()
  }

  /// Resets all data for the printer except for the current position.
  pub fn reset_and_retain_printing_position(&mut self) {
    *self = Printer {
      printing_position: self.printing_position.take(),
      ..Default::default()
    };
  }

  /// Resets all data for the printer and assigns the given printing position.
  pub fn reset_with_position(&mut self, printing_position: PrintingPosition) {
    *self = Printer {
      printing_position: Some(printing_position),
      ..Default::default()
    }
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
