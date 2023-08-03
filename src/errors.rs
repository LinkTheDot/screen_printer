/// These are the possible ways the program can fail.
///
/// Each error will contain 'ErrorData' which holds the
/// expected and outcome results in the event of the error.
#[derive(Debug, Eq, PartialEq)]
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

  /// When no [`printing positions`](crate::printing_position::PrintingPosition) are defined, the printer will attempt to read the
  /// position of the cursor to print the grid.
  /// This error is returned when getting the position of the cursor failed.
  ///
  /// The error message is contained.
  CursorError(String),

  /// When attempting to get the dimensions of the terminal, an error occurred.
  ///
  /// The error message is contained
  FailedToGetTerminalDimensions(String),
  /// This error is returned when a grid passed in to [`dynamic_print`](crate::dynamic_printer::DynamicPrinter::dynamic_print) is
  /// larger than the dimensions of the terminal.
  GridLargerThanTerminal,

  /// When attempting to get the dimensions of the grid, there were no stored dimensions for the grid.
  GridDimensionsNotDefined,
  /// When attempting to get the origin position of the printer, there was no stored position for the grid.
  CursorPositionNotDefined,

  /// There was no [`PrintingPosition`](crate::printing_position::PrintingPosition) when attempting to get origin from printing position.err
  MissingPrintingPosition,
}

/// When creating a grid from [`crate::printer::Printer::create_grid_from_full_character_list`](crate::printer::Printer::create_grid_from_full_character_list),
/// the sizes given and the actual amount of characters didn't match.
#[derive(Debug, Eq, PartialEq)]
pub struct LengthErrorData {
  pub expected_character_count: usize,
  pub actual_character_count: usize,
}

impl LengthErrorData {
  /// Creates a new LengthErrorData for the expected length and actual length
  pub fn new(expected_character_count: usize, actual_character_count: usize) -> Self {
    Self {
      expected_character_count,
      actual_character_count,
    }
  }
}
