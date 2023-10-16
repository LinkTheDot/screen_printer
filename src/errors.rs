use thiserror::Error;

/// These are the possible ways the program can fail.
///
/// Each error will contain 'ErrorData' which holds the
/// expected and outcome results in the event of the error.
#[derive(Error, Debug, Clone)]
pub enum PrintingError {
  #[error("Failed to create a grid as there were too many characters. Expected {}, got {}", .0.expected_character_count, .0.actual_character_count)]
  TooManyCharacters(LengthErrorData),
  #[error("Failed to create a grid as there weren't enough characters. Expected {}, got {}", .0.expected_character_count, .0.actual_character_count)]
  TooLittleCharacters(LengthErrorData),

  #[error("Failed to obtain the dimensions of the terminal. Reason: {}", .0)]
  FailedToGetTerminalDimensions(String),
  #[error("A grid larger than the terminal itself was passed in.")]
  GridLargerThanTerminal,

  #[error("A non rectangular grid was passed in.")]
  NonRectangularGrid,
  #[error("Failed to obtain the stored dimensions of the grid.")]
  GridDimensionsNotDefined,
  #[error("Failed to obtain the stored terminal dimensions.")]
  TerminalDimensionsNotDefined,
  #[error("Failed to obtain the stored origin position.")]
  OriginNotDefined,
}

impl PartialEq for PrintingError {
  fn eq(&self, other: &Self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(other)
  }
}

impl Eq for PrintingError {}

/// When creating a grid from [`crate::printer::Printer::create_grid_from_full_character_list`](crate::printer::Printer::create_grid_from_full_character_list),
/// the sizes given and the actual amount of characters didn't match.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LengthErrorData {
  pub expected_character_count: usize,
  pub actual_character_count: usize,
}

impl LengthErrorData {
  /// Creates a new LengthErrorData for the expected length and actual length
  pub(crate) fn new(expected_character_count: usize, actual_character_count: usize) -> Self {
    Self {
      expected_character_count,
      actual_character_count,
    }
  }
}
