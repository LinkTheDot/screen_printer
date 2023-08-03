// Create documentation for:
//  - Printer struct.
//  - PrintingPosition struct.
//  - dynamic_print method and;
//  - probably a few other things I forgot to document.

pub use crate::dynamic_printer::*;
pub use crate::errors::*;
pub use crate::printing_position::*;
use std::cmp::Ordering;
use std::fmt;
use std::{io, io::Write};

impl fmt::Display for PrintingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{self:?}")
  }
}

/// # Screen Printer
///
/// Screen Printer is a rust crate that will allow you to build and print arrays of
/// data into a grid format.
///
/// The purpose of this crate is to make it easier to print rectangular blocks of text to the terminal.
/// Including features like:
///
/// - `DynamicPrint`, which only prints any characters that changed from any previously printed grid\*.
/// - `PrintingPosition`, which allows you to print your string to different places on the terminal, such as the center.
///
/// \* If the grid changes in size or position it is reprinted in its entirety.
///
/// ## Examples
///
/// #### Using the dynamic print method to print a grid
///
/// The core part of this crate is the [`dynamic_print`](crate::dynamic_printer::DynamicPrinter::dynamic_print) method.
/// This will take a rectangular grid of characters, and print only the parts of the grid that have changed since the last print.
///
/// ```rust,no_run
/// use screen_printer::printer::*;
///
/// const WIDTH: usize = 3;
/// const HEIGHT: usize = 3;
///
/// fn main() {
///   print!("\u{1b}[2J"); // Clear all text on the terminal
///   // The default printing position is the bottom left of the terminal
///   let mut printer = Printer::new_with_printing_position(PrintingPosition::default());
///
///   // Create the first grid to be printed.
///   let grid_1 = "abc\n123\nxyz".to_string();
///   // print the first grid.
///   printer.dynamic_print(grid_1).unwrap();
///
///   // Wait before printing the second grid.
///   std::thread::sleep(std::time::Duration::from_millis(500));
///
///   // Create the second grid to be printed.
///   let grid_2 = "abc\n789\nxyz".to_string();
///   // Print the second grid.
///   // This will only end up printing the difference between the two grids/
///   printer.dynamic_print(grid_2).unwrap();
/// }
/// ```
///
/// This will result in
///
/// ```bash,no_run
/// abc
/// 123
/// xyz
/// ```
///
/// Into
///
/// ```bash,no_run
/// abc
/// 789 < only line that was actually printed
/// xyz
/// ```
///
/// #### Printing Position
///
/// Another feature shown in the above example, the [`PrintingPosition`](crate::printing_position::PrintingPosition).
///
/// This will print the grid in any of the 9 defined positions on the terminal.
/// These are split by the X and Y axes:
///
/// - Left/Top,
/// - Middle, and;
/// - Right/Bottom.
#[derive(Default, Debug)]
pub struct Printer {
  pub(crate) previous_grid: String,

  pub(crate) origin_position: Option<(usize, usize)>,

  pub(crate) grid_height: Option<usize>,
  pub(crate) grid_width: Option<usize>,

  printing_position: Option<PrintingPosition>,
  pub(crate) printing_position_changed_since_last_print: bool,
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
