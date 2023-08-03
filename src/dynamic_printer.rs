use crate::printer::*;
use std::{io, io::Write};
use termion::cursor::DetectCursorPos;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

mod tests;

pub trait DynamicPrinter {
  /// The dynamic_print method is the main part of the screen_printer crate.
  /// This method will print any grid to the terminal based on the [`PrintingPosition`](crate::printing_position::PrintingPosition).
  ///
  /// When printing a new grid to the screen, it'll compare every character from the previous one, and only print the characters that have changed.
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
  /// For more information about using the printer, refer to the example on [`github`](https://github.com/LinkTheDot/screen_printer/blob/master/examples/dynamic_printer.rs)
  ///
  /// # Errors
  ///
  /// - The given grid wasn't rectangular in shape.
  /// - The given grid is larger than the current dimensions of the terminal.
  ///
  /// ### When no printing options are defined
  ///
  /// - Failed to get stdout in raw mode.
  /// - Timed out when trying to get a hold on stdin when reading for the cursor's position.
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
    let new_grid_dimensions = Self::valid_rectangle_check(&new_grid)?;

    if new_grid_dimensions.0 > terminal_dimensions.0
      || new_grid_dimensions.1 > terminal_dimensions.1
    {
      return Err(PrintingError::GridLargerThanTerminal);
    }

    if !self.previous_grid.is_empty() && !self.printing_position_changed_since_last_print {
      let new_origin = self.get_new_origin(new_grid_dimensions, terminal_dimensions)?;
      let (old_grid_width, old_grid_height) = self.get_grid_dimensions()?;

      if old_grid_width != new_grid_dimensions.0 || old_grid_height != new_grid_dimensions.1 {
        self.replace_currently_printed_grid(
          &new_grid,
          Some(new_grid_dimensions),
          terminal_dimensions,
        )?;
      }

      self.origin_position = Some(new_origin);

      let printable_difference = self.get_printable_difference(&new_grid)?;

      print!("{}", printable_difference);
    } else {
      self.replace_currently_printed_grid(
        &new_grid,
        Some(new_grid_dimensions),
        terminal_dimensions,
      )?;
    }

    let _ = io::stdout().flush();
    self.previous_grid = new_grid;
    self.grid_width = Some(new_grid_dimensions.0);
    self.grid_height = Some(new_grid_dimensions.1);
    self.printing_position_changed_since_last_print = false;

    Ok(())
  }

  fn clear_grid(&mut self) -> Result<(), PrintingError> {
    let (grid_height, grid_width) = self.get_grid_dimensions()?;
    let empty_grid = Self::create_grid_from_single_character(' ', grid_width, grid_height);

    let printer_origin = self.get_origin_position()?;
    print_grid_freestanding(&empty_grid, printer_origin)?;

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
  ///
  /// # Errors
  /// - Failed to get stdout in raw mode.
  /// - Timed out when trying to get a hold on stdin when reading for the cursor's position.
  fn get_new_origin(
    &mut self,
    new_grid_dimensions: (usize, usize),
    terminal_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;

  /// Returns the (x, y) of the printer's new origin
  ///
  /// # Errors
  ///
  /// - Failed to get stdout in raw mode.
  /// - Timed out when trying to get a hold on stdin when reading for the cursor's position.
  fn get_origin_from_cursor(
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;
  /// Returns the (x, y) of the printer's new origin
  ///
  /// # Errors
  ///
  /// - When no printing position exists.
  fn get_origin_from_printing_position(
    &self,
    new_grid_dimensions: (usize, usize),
    terminal_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;

  /// Returns the current position of the cursor
  ///
  /// # Errors
  ///
  /// - Failed to get stdout in raw mode.
  /// - Timed out when trying to get a hold on stdin when reading for the cursor's position.
  fn get_current_cursor_position() -> Result<(usize, usize), PrintingError>;

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
  ///
  /// ### When no printing options are defined
  ///
  /// - Failed to get stdout in raw mode.
  /// - Timed out when trying to get a hold on stdin when reading for the cursor's position.
  fn replace_currently_printed_grid(
    &mut self,
    new_grid: &str,
    new_grid_dimensions: Option<(usize, usize)>,
    terminal_dimensions: (usize, usize),
  ) -> Result<(), PrintingError>;
}

impl DynamicPrinterMethods for Printer {
  fn get_printable_difference(&self, grid: &str) -> Result<String, PrintingError> {
    let old_grid = self.previous_grid.replace('\n', "");
    let new_grid = grid.replace('\n', "");
    let grid_size = new_grid.chars().count();

    let (origin_x, origin_y) = self.get_origin_position()?;
    let (grid_width, _) = self.get_grid_dimensions()?;

    let mut last_appended_pixel_index = 100000;
    let mut latest_pixel_index = 100000;
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
    let Some((x, y)) = self.origin_position else {
      return Err(PrintingError::CursorPositionNotDefined);
    };

    print!("\x1B[{};{}H", y, x);

    Ok(())
  }

  fn get_new_origin(
    &mut self,
    new_grid_dimensions: (usize, usize),
    terminal_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let origin: (usize, usize) = if self.get_current_printing_position().is_none() {
      Self::get_origin_from_cursor(new_grid_dimensions)?
    } else {
      self.get_origin_from_printing_position(new_grid_dimensions, terminal_dimensions)?
    };

    Ok(origin)
  }

  fn get_origin_from_cursor(
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let (mut x, mut y) = Self::get_current_cursor_position()?;
    let (grid_width, grid_height) = new_grid_dimensions;

    x = (x as isize - grid_width as isize).max(1) as usize;
    y = (y as isize - grid_height as isize).max(1) as usize;

    Ok((x, y))
  }

  fn get_origin_from_printing_position(
    &self,
    (grid_width, grid_height): (usize, usize),
    (terminal_width, terminal_height): (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let Some(printing_position) = self.get_current_printing_position() else {
      return Err(PrintingError::MissingPrintingPosition);
    };

    let x: usize = match printing_position.x_printing_position {
      XPrintingPosition::Left => 1,
      XPrintingPosition::Middle => {
        ((terminal_width as f32 / 2.0).floor() - (grid_width as f32 / 2.0).floor()).floor() as usize
      }

      XPrintingPosition::Right => (terminal_width - grid_width) + 1,
    };

    let y: usize = match printing_position.y_printing_position {
      YPrintingPosition::Top => 1,
      YPrintingPosition::Middle => ((terminal_height as f32 / 2.0).floor()
        - (grid_height as f32 / 2.0).floor())
      .floor() as usize,
      YPrintingPosition::Bottom => (terminal_height - grid_height) + 1,
    };

    Ok((x, y))
  }

  fn get_current_cursor_position() -> Result<(usize, usize), PrintingError> {
    let cursor_position = match io::stdout().into_raw_mode() {
      Ok(raw_stdout) => MouseTerminal::from(raw_stdout).cursor_pos(),
      Err(error) => return Err(PrintingError::CursorError(error.to_string())),
    };

    match cursor_position {
      Ok(position) => Ok((position.0 as usize, position.1 as usize)),
      Err(error) => Err(PrintingError::CursorError(error.to_string())),
    }
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
      Self::valid_rectangle_check(new_grid)?
    };

    if self.get_origin_position().is_err() {
      self.origin_position =
        Some(self.get_new_origin((new_grid_width, new_grid_height), terminal_dimensions)?);
    }

    self.grid_width = Some(new_grid_width);
    self.grid_height = Some(new_grid_width);

    self.clear_grid()?;

    self.origin_position =
      Some(self.get_new_origin((new_grid_width, new_grid_height), terminal_dimensions)?);

    let printer_origin = self.get_origin_position()?;
    print_grid_freestanding(new_grid, printer_origin)?;

    Ok(())
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
  Printer::valid_rectangle_check(grid)?;
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
