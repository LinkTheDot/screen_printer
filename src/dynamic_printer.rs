use crate::printer::*;
use std::{io, io::Write};

mod tests;

pub trait DynamicPrinter {
  /// This method will print any grid to the terminal based on the [`PrintingPosition`](crate::printing_position::PrintingPosition).
  ///
  /// When printing a new grid to the screen, it'll compare every character from the previous one, and only print the characters that have changed.
  ///
  /// # Errors
  ///
  /// - The given grid wasn't rectangular in shape.
  /// - The string for the grid is empty.
  /// - The given grid is larger than the current dimensions of the terminal.
  ///
  /// # Example
  /// ```rust,no_run
  /// use screen_printer::printer::*;
  ///
  /// const WIDTH: usize = 3;
  /// const HEIGHT: usize = 3;
  ///
  /// fn main() {
  ///   print!("\u{1b}[2J"); // Clear all text on the terminal
  ///   // The default printing position is the bottom left of the terminal
  ///   let mut printer = Printer::new();
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
  ///   // This will only end up printing the difference between the two grids.
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
  /// For more information about using the printer, refer to the example on [`github`](https://github.com/LinkTheDot/screen_printer/blob/master/examples/dynamic_printer.rs)
  fn dynamic_print(&mut self, new_grid: String) -> Result<(), PrintingError>;

  /// Replaces every character in the grid with whitespace.
  ///
  /// # Errors
  ///
  /// - Grid dimensions weren't defined.
  /// - Origin wasn't defined.
  fn clear_grid(&mut self) -> Result<(), PrintingError>;
}

impl DynamicPrinter for Printer {
  fn dynamic_print(&mut self, new_grid: String) -> Result<(), PrintingError> {
    let terminal_dimensions = Printer::get_terminal_dimensions()?;
    let new_grid_dimensions = Self::get_rectangular_dimensions(&new_grid)?;

    if new_grid_dimensions.0 > terminal_dimensions.0
      || new_grid_dimensions.1 > terminal_dimensions.1
    {
      return Err(PrintingError::GridLargerThanTerminal);
    }

    // Check if the dimensions of the grid have changed
    if let Ok((old_grid_width, old_grid_height)) = self.get_grid_dimensions() {
      if old_grid_width != new_grid_dimensions.0 || old_grid_height != new_grid_dimensions.1 {
        self.printing_position_changed_since_last_print = true;
      }
    }

    // Check if the dimensions of the terminal have changed
    if let Ok((old_terminal_width, old_terminal_height)) =
      self.get_terminal_dimensions_from_previous_print()
    {
      if old_terminal_width != terminal_dimensions.0 || old_terminal_height != terminal_dimensions.1
      {
        self.printing_position_changed_since_last_print = true;
      }
    }

    if !self.previous_grid.is_empty() && !self.printing_position_changed_since_last_print {
      let new_origin = self.get_new_origin(new_grid_dimensions, terminal_dimensions);
      self.update_origin(new_origin);

      let printable_difference = self.get_printable_difference(&new_grid)?;

      print!("{}", printable_difference);
    } else if self.printing_position_changed_since_last_print {
      self.replace_currently_printed_grid(
        &new_grid,
        Some(new_grid_dimensions),
        terminal_dimensions,
      )?;
    } else {
      let new_origin = self.get_new_origin(new_grid_dimensions, terminal_dimensions);
      self.update_origin(new_origin);

      print_grid_freestanding(&new_grid, new_origin)?;
    }

    let _ = io::stdout().flush();
    self.previous_grid = new_grid;
    self.update_dimensions(new_grid_dimensions);
    self.update_terminal_dimensions_from_previous_print(terminal_dimensions);
    self.printing_position_changed_since_last_print = false;

    Ok(())
  }

  fn clear_grid(&mut self) -> Result<(), PrintingError> {
    let (grid_width, grid_height) = self.get_grid_dimensions()?;

    Self::clear_space_on_terminal((grid_width, grid_height), self.get_origin_position()?)?;

    self.previous_grid = Self::create_grid_from_single_character(' ', grid_width, grid_height);

    Ok(())
  }
}

trait DynamicPrinterMethods {
  /// Gets a list of escape codes for cursor movement followed by
  /// the difference in pixels between the old and new grids.
  ///
  /// # Errors
  ///
  /// - When origin hasn't been set before calling this method.
  /// - When the old grid's dimensions haven't been set before calling this method.
  fn get_printable_difference(&self, grid: &str) -> Result<String, PrintingError>;

  /// Moves the cursor to the assigned origin.
  ///
  /// # Errors
  ///
  /// - When origin isn't set.
  fn move_to_origin(&self) -> Result<(), PrintingError>;

  /// Returns a new origin based on a few parameters:
  /// The dimensions of the new grid,
  /// The dimensions of the terminal and;
  /// The current printing settings, or where the terminal cursor is if there are none.
  fn get_new_origin(
    &self,
    new_grid_dimensions: (usize, usize),
    terminal_dimensions: (usize, usize),
  ) -> (usize, usize);

  /// Prints whitespace over the previous grid, then prints the new one wherever it needs to go.
  ///
  /// Takes optional dimensions for the new grid for if they've already been calculated.
  /// Does not check if those dimensions are valid or not.
  ///
  /// # Errors
  ///
  /// - The new grid wasn't rectangular in shape.
  /// - Grid dimensions weren't set.
  /// - Origin wasn't set.
  fn replace_currently_printed_grid(
    &mut self,
    new_grid: &str,
    new_grid_dimensions: Option<(usize, usize)>,
    terminal_dimensions: (usize, usize),
  ) -> Result<(), PrintingError>;

  fn clear_space_on_terminal(
    clearing_dimensions: (usize, usize),
    top_left_position: (usize, usize),
  ) -> Result<(), PrintingError>;
}

impl DynamicPrinterMethods for Printer {
  fn get_printable_difference(&self, grid: &str) -> Result<String, PrintingError> {
    let old_grid = self.previous_grid.replace('\n', "");
    let new_grid = grid.replace('\n', "");
    let grid_size = new_grid.chars().count();

    let (origin_x, origin_y) = self.get_origin_position()?;
    let (grid_width, _) = self.get_grid_dimensions()?;

    let mut last_appended_pixel_index = 1000000;
    let mut latest_pixel_index = 1000000;
    let mut printable_difference = String::new();

    old_grid.chars().zip(new_grid.chars()).enumerate().for_each(
      |(pixel_index, (old_pixel, new_pixel))| {
        if new_pixel == old_pixel {
          return;
        }

        if pixel_index != 0
          && (last_appended_pixel_index == pixel_index - 1 || latest_pixel_index == pixel_index - 1)
          && (pixel_index % grid_width != 0 || pixel_index == grid_size - 1)
        {
          printable_difference.push(new_pixel);

          last_appended_pixel_index = pixel_index;
        } else {
          let mut index_as_coords = pixel_index.index_as_coordinates(&grid_width);
          index_as_coords.0 += origin_x;
          index_as_coords.1 += origin_y;

          latest_pixel_index = pixel_index;

          printable_difference.push_str(&format!(
            "\x1B[{};{}H{}",
            index_as_coords.1, index_as_coords.0, new_pixel
          ));
        }
      },
    );

    Ok(printable_difference)
  }

  fn move_to_origin(&self) -> Result<(), PrintingError> {
    let (x, y) = self.get_origin_position()?;

    print!("\x1B[{};{}H", y, x);

    Ok(())
  }

  fn get_new_origin(
    &self,
    (grid_width, grid_height): (usize, usize),
    (terminal_width, terminal_height): (usize, usize),
  ) -> (usize, usize) {
    let printing_position = self.get_current_printing_position();

    let x: usize = match printing_position.x_printing_position {
      XPrintingPosition::Left => 1,
      XPrintingPosition::Middle => calculate_grid_center_placement(grid_width, terminal_width),
      XPrintingPosition::Right => {
        calculate_grid_positive_border_placement(grid_width, terminal_width)
      }
      XPrintingPosition::Custom(cursor_x_position) => {
        calculate_custom_grid_position(grid_width, terminal_width, cursor_x_position)
      }
    };

    let y: usize = match printing_position.y_printing_position {
      YPrintingPosition::Top => 1,
      YPrintingPosition::Middle => calculate_grid_center_placement(grid_height, terminal_height),
      YPrintingPosition::Bottom => {
        calculate_grid_positive_border_placement(grid_height, terminal_height)
      }
      YPrintingPosition::Custom(cursor_y_position) => {
        calculate_custom_grid_position(grid_height, terminal_height, cursor_y_position)
      }
    };

    (x, y)
  }

  fn replace_currently_printed_grid(
    &mut self,
    new_grid: &str,
    new_grid_dimensions: Option<(usize, usize)>,
    terminal_dimensions: (usize, usize),
  ) -> Result<(), PrintingError> {
    let (new_grid_width, new_grid_height) = if let Some(new_grid_dimensions) = new_grid_dimensions {
      new_grid_dimensions
    } else {
      Self::get_rectangular_dimensions(new_grid)?
    };

    // Can return an error if the PrintingPosition was changed before a first print.
    let _ = self.clear_grid();

    let new_origin = self.get_new_origin((new_grid_width, new_grid_height), terminal_dimensions);

    self.update_dimensions((new_grid_width, new_grid_height));
    self.update_origin(new_origin);

    print_grid_freestanding(new_grid, new_origin)?;

    Ok(())
  }

  fn clear_space_on_terminal(
    clearing_dimensions: (usize, usize),
    top_left_position: (usize, usize),
  ) -> Result<(), PrintingError> {
    let empty_grid =
      Self::create_grid_from_single_character(' ', clearing_dimensions.0, clearing_dimensions.1);

    print_grid_freestanding(&empty_grid, top_left_position)
  }
}

/// Splits the grid into rows and moves the cursor down to print each row at the given position, starting from the top left.
/// Does not check if the printed grid will overflow off the right or bottom of the terminal.
///
/// # Errors
///
/// - The passed in grid isn't rectangular.
fn print_grid_freestanding(
  grid: &str,
  printing_position: (usize, usize),
) -> Result<(), PrintingError> {
  Printer::get_rectangular_dimensions(grid)?;
  let mut grid_with_cursor_movements = String::new();
  let cursor_movement = format!("\x1B[1B\x1B[{}G", printing_position.0);

  for grid_row in grid.split('\n') {
    grid_with_cursor_movements.push_str(grid_row);
    grid_with_cursor_movements.push_str(&cursor_movement);
  }

  print!("\x1B[{};{}H", printing_position.1, printing_position.0);
  print!("{}", grid_with_cursor_movements);

  Ok(())
}

/// Determines the position of where to place a grid in the center of the screen based on the length
/// of the grid and terinal.
fn calculate_grid_center_placement(grid_length: usize, terminal_length: usize) -> usize {
  ((terminal_length as f32 / 2.0).floor() - (grid_length as f32 / 2.0).floor()) as usize
}

/// Determines the position of where to place a grid on the positive border of the screen(bottom and right)
/// on the length of the grid and terminal.
fn calculate_grid_positive_border_placement(grid_length: usize, terminal_length: usize) -> usize {
  ((terminal_length as isize - grid_length as isize).max(0) + 1) as usize
}

fn calculate_custom_grid_position(
  grid_length: usize,
  terminal_length: usize,
  grid_placement: usize,
) -> usize {
  // Accounts for when the placement is set to 0 due to user error.
  let grid_placement = grid_placement.max(1);

  grid_placement
    - ((grid_placement + grid_length) as isize - terminal_length as isize).max(0) as usize
}

trait VecMethods<T> {
  /// Gets a mutable reference to the top most item in a vector
  ///
  /// # Example
  /// ```ignore
  /// let mut data = vec![0, 2, 4, 5];
  ///
  /// let mut top_item = data.get_mut_top();
  /// *top_item += 1;
  ///
  /// assert_eq!(data.get(3), Some(6));
  /// ```
  fn get_mut_top(&mut self) -> Option<&mut T>;
}

impl<T> VecMethods<T> for std::vec::Vec<T> {
  fn get_mut_top(&mut self) -> Option<&mut T> {
    let item_count = self.len();

    if !self.is_empty() {
      self.get_mut(item_count - 1)
    } else {
      None
    }
  }
}

trait UsizeMethods {
  /// Converts an index into coordinates for the given grid's width.
  fn index_as_coordinates(&self, grid_width: &Self) -> (usize, usize);
}

impl UsizeMethods for usize {
  fn index_as_coordinates(&self, grid_width: &Self) -> (usize, usize) {
    (self % grid_width, self / grid_width)
  }
}
