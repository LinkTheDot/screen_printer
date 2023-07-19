#![allow(unused)]
// Remove all the unwraps/expects that will cause panics
// Fix newlines to work with printing away from the edge of the screen.
//   Newlines move the cursor to the *start* of the next line.
// Fill the old grid with whitespace if the new one changes dimensions.

use crate::printer::*;
use lazy_static::lazy_static;
use std::{io, io::Write};
use termion::cursor::DetectCursorPos;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

mod tests;

lazy_static! {
  static ref TERMINAL_DIMENSIONS: (usize, usize) = {
    let (width, height) = termion::terminal_size().unwrap();

    (width as usize, height as usize)
  };
}

/// A PixelDifference will contain a single instance of
/// a contigious string of different pixels within the grid.
///
/// The index in which the first different pixel occupies
/// is also stored.
#[derive(Debug, PartialEq)]
struct PixelDifference {
  pixels: String,
  index: usize,
}

impl PixelDifference {
  /// Creates a new pixel difference with the given pixel and index
  fn new(pixel: String, index: usize) -> Self {
    Self {
      pixels: pixel,
      index,
    }
  }

  /// Checks if the current index is bordering the grid, and adds a newline
  /// to the PixelDifference if it is.
  fn check_newlines(&mut self, current_index: usize, grid_width: usize, grid_size: usize) {
    if current_index % grid_width == 0 && current_index != grid_size - 1 {
      self.pixels.push('\n');
    }
  }
}

pub trait DynamicPrinter {
  /// An error is returned when the cursor can't be read or the passed in grid does not match the expected size.
  fn dynamic_print(&mut self, new_grid: String) -> Result<(), PrintingError>;

  /// Resets all data for the printer.
  fn reset(&mut self);
  /// Resets all data for the printer and assigns the given printing options.
  fn reset_with_options(&mut self, printing_options: PrintingOptions);
  /// Resets all data for the printer except for the current options.
  fn reset_and_retain_options(&mut self);

  /// Replaces every character in the grid with whitespace
  fn clear_grid(&mut self) -> Result<(), PrintingError>;
}

impl DynamicPrinter for Printer {
  fn dynamic_print(&mut self, new_grid: String) -> Result<(), PrintingError> {
    let (terminal_width, terminal_height) = *TERMINAL_DIMENSIONS;
    let (new_grid_width, new_grid_height) = valid_rectangle_check(&new_grid)?;

    if new_grid_width > terminal_width || new_grid_height > terminal_height {
      return Err(PrintingError::GridIsLargerThanTerminal);
    }

    if !self.previous_grid.is_empty() {
      let new_origin = self.get_new_origin((new_grid_width, new_grid_height))?;

      if self.grid_width.unwrap() != new_grid_width || self.grid_height.unwrap() != new_grid_height
      {
        self.clear_grid()?;
        self
          .overwrite_currently_printed_grid(&new_grid, Some((new_grid_width, new_grid_height)))?;
      }

      self.origin_position = Some(new_origin);

      let different_pixels =
        Self::get_pixel_difference(&self.previous_grid, &new_grid, new_grid_width);
      let printable_difference = self.get_printable_difference(different_pixels);

      print!("{}", printable_difference);
    } else {
      self.overwrite_currently_printed_grid(&new_grid, Some((new_grid_width, new_grid_height)))?;
    }

    let _ = io::stdout().flush();
    self.previous_grid = new_grid;
    self.grid_width = Some(new_grid_width);
    self.grid_height = Some(new_grid_height);

    Ok(())
  }

  fn reset(&mut self) {
    *self = Printer::default()
  }

  fn reset_with_options(&mut self, printing_options: PrintingOptions) {
    self.reset();
    self.assign_options(printing_options);
  }

  fn reset_and_retain_options(&mut self) {
    let options = self.printing_options.take();

    self.reset();
    self.printing_options = options;
  }

  fn clear_grid(&mut self) -> Result<(), PrintingError> {
    let Some(grid_width) = self.grid_width else { return Err(PrintingError::GridDimensionsNotDefined) };
    let Some(grid_height) = self.grid_height else { return Err(PrintingError::GridDimensionsNotDefined) };

    let empty_grid = Self::create_grid_from_single_character(' ', grid_width, grid_height);

    self.move_to_origin()?;
    // Here would be a method to replace '\n' with escape codes.
    print!("{}", empty_grid);

    Ok(())
  }
}

trait DynamicPrinterMethods {
  /// Returns true if both the previous and new grid have the same
  /// length per row and amount of rows
  fn grid_size_matches_previous_grid(&self, grid: &str) -> bool;

  /// Gets a list of the pixel indices that were different from the previous grid
  fn get_pixel_difference(
    previous_grid: &str,
    grid: &str,
    grid_width: usize,
  ) -> Vec<PixelDifference>;

  /// Moves the cursor to the assigned origin.
  fn move_to_origin(&self) -> Result<(), PrintingError>;

  /// Returns the current position of the cursor
  fn get_current_cursor_position() -> Result<(usize, usize), PrintingError>;

  fn get_printable_difference(&self, pixel_differences: Vec<PixelDifference>) -> String;

  fn get_new_origin(
    &mut self,
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;

  /// Returns the (x, y) of the printer's new origin
  fn get_origin_from_cursor(
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;
  /// Returns the (x, y) of the printer's new origin
  fn get_origin_from_options(
    &self,
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError>;

  fn overwrite_currently_printed_grid(
    &mut self,
    new_grid: &str,
    new_grid_dimensions: Option<(usize, usize)>,
  ) -> Result<(), PrintingError>;

  // fn adjust_grids_for_comparison(
  //   &self,
  //   new_grid: &str,
  //   new_origin: (usize, usize),
  // ) -> Result<(String, String), PrintingError>;
  //
  // /// Returns the (Left, Right) space to be added to a grid for it to match the difference;
  // fn get_whitespace_split_for_grid_width(difference: f32, origin: (usize, usize))
  //   -> (usize, usize);
  // /// Returns the (Top, Bottom) space to be added to a grid for it to match the difference;
  // fn get_whitespace_split_for_grid_height(
  //   difference: f32,
  //   origin: (usize, usize),
  // ) -> (usize, usize);
}

impl DynamicPrinterMethods for Printer {
  fn grid_size_matches_previous_grid(&self, grid: &str) -> bool {
    let new_grid_height = grid.rsplit('\n').count();
    let old_grid_height = self.previous_grid.rsplit('\n').count();

    let mut new_grid_split = grid.rsplit('\n');

    let Some(old_grid_row) = self.previous_grid.rsplit('\n').next() else { return false; };
    let old_grid_row_width = old_grid_row.chars().count();

    new_grid_height == old_grid_height
      && new_grid_split.all(|new_row| new_row.chars().count() == old_grid_row_width)
  }

  fn get_pixel_difference(
    previous_grid: &str,
    grid: &str,
    grid_width: usize,
  ) -> Vec<PixelDifference> {
    let old_grid = previous_grid.replace('\n', "");
    let new_grid = grid.replace('\n', "");

    let grid_size = new_grid.chars().count();
    let grid_iter = old_grid.chars().zip(new_grid.chars());

    let mut last_edited_pixel_index = 0;

    grid_iter.enumerate().fold(
      Vec::new(),
      |mut different_pixels, (pixel_index, (old_pixel, new_pixel))| {
        if new_pixel == old_pixel {
          return different_pixels;
        }

        if let Some(latest_pixel) = different_pixels.get_mut_top() {
          if last_edited_pixel_index == pixel_index - 1 || latest_pixel.index == pixel_index - 1 {
            latest_pixel.check_newlines(pixel_index, grid_width, grid_size);

            latest_pixel.pixels.push_str(&new_pixel.to_string());

            last_edited_pixel_index = pixel_index;

            return different_pixels;
          }
        }

        let pixel_difference = PixelDifference::new(new_pixel.to_string(), pixel_index);

        different_pixels.push(pixel_difference);

        different_pixels
      },
    )
  }

  fn move_to_origin(&self) -> Result<(), PrintingError> {
    let Some((x, y)) = (self.origin_position) else {
      return Err(PrintingError::CursorPositionNotDefined);
    };

    print!("\x1B[{};{}H", y, x);

    Ok(())
  }

  fn get_current_cursor_position() -> Result<(usize, usize), PrintingError> {
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let cursor_position = stdout.cursor_pos();

    match cursor_position {
      Ok(position) => Ok((position.0 as usize, position.1 as usize)),
      Err(error) => Err(PrintingError::CursorError(error.to_string())),
    }
  }

  fn get_printable_difference(&self, pixel_differences: Vec<PixelDifference>) -> String {
    let (origin_x, origin_y) = self.get_origin_position().unwrap();

    pixel_differences
      .iter()
      .fold(String::new(), |mut printable_diff, pixel_difference| {
        let (mut x, mut y) = pixel_difference
          .index
          .index_as_coordinates(&self.grid_width.expect("Grid width is not set."));

        x += origin_x;
        y += origin_y;

        let cursor_movement = format!("\x1B[{y};{x}H");
        let movement_with_pixels = format!("{}{}", cursor_movement, pixel_difference.pixels);

        printable_diff.push_str(&movement_with_pixels);

        printable_diff
      })
  }

  fn get_new_origin(
    &mut self,
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let origin: (usize, usize) = if self.printing_options.is_none() {
      Self::get_origin_from_cursor(new_grid_dimensions)?
    } else {
      self.get_origin_from_options(new_grid_dimensions)?
    };

    Ok(origin)
  }

  fn get_origin_from_cursor(
    new_grid_dimensions: (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let (mut x, mut y) = Self::get_current_cursor_position()?;
    let (grid_width, grid_height) = new_grid_dimensions;

    // If y - grid_height < 1, set y to 1.
    y = (y as isize - grid_height as isize).max(1) as usize;
    // If x - grid_width < 1, set x to 1.
    x = (x as isize - grid_width as isize).max(1) as usize;

    Ok((x, y))
  }

  fn get_origin_from_options(
    &self,
    (grid_width, grid_height): (usize, usize),
  ) -> Result<(usize, usize), PrintingError> {
    let Some(printing_options) = self.printing_options.as_ref() else {
      return Err(PrintingError::MissingPrintingOptions);
    };
    let (terminal_width, terminal_height) = *TERMINAL_DIMENSIONS;

    let x: usize = match printing_options.x_printing_option {
      XPrintingOption::Left => 1,
      XPrintingOption::Middle => {
        ((terminal_width as f32 / 2.0).floor() - (grid_width as f32 / 2.0).floor()).floor() as usize
      }

      XPrintingOption::Right => (terminal_width - grid_width) + 1,
    };

    let y: usize = match printing_options.y_printing_option {
      YPrintingOption::Top => 1,
      YPrintingOption::Middle => ((terminal_height as f32 / 2.0).floor()
        - (grid_height as f32 / 2.0).floor())
      .floor() as usize,
      YPrintingOption::Bottom => (terminal_height - grid_height) + 1,
    };

    Ok((x, y))
  }

  // fn adjust_grids_for_comparison(
  //   &self,
  //   new_grid: &str,
  //   new_origin: (usize, usize),
  // ) -> Result<(String, String), PrintingError> {
  //   let (old_grid_width, old_grid_height) = self.get_grid_dimensions()?;
  //   let (new_grid_width, new_grid_height) = valid_rectangle_check(new_grid)?;
  //
  //   let width_difference = (old_grid_width as f32 - new_grid_width as f32).abs();
  //   let height_difference = (old_grid_height as f32 - new_grid_height as f32).abs();
  //
  //   let (back_whitespace, front_whitespace) =
  //     Self::get_whitespace_split_for_grid_width(width_difference, new_origin);
  //   let (top_whitespace, bottom_whitespace) =
  //     Self::get_whitespace_split_for_grid_height(height_difference, new_origin);
  //
  //   todo!()
  // }
  //
  // // Isn't working as intended, check the temp file under projects.
  // fn get_whitespace_split_for_grid_width(
  //   difference: f32,
  //   (origin_x, _): (usize, usize),
  // ) -> (usize, usize) {
  //   let terminal_width = TERMINAL_DIMENSIONS.0;
  //   let (mut left, mut right) = (
  //     (difference / 2.0).floor() as usize,
  //     (difference / 2.0).ceil() as usize,
  //   );
  //
  //   if origin_x - left < 1 {
  //     // add the overflow amount to the right
  //     let overflow_amount = (origin_x as isize - (left + 1) as isize).unsigned_abs();
  //
  //     left -= overflow_amount;
  //     right += overflow_amount;
  //   } else if origin_x + right > terminal_width {
  //     // add the overflow amount to the left
  //     let overflow_amount = ((origin_x + right) - terminal_width).max(0);
  //
  //     right -= overflow_amount;
  //     left += overflow_amount;
  //   } else {
  //     panic!("Grid is larger than terminal."); // This shouldn't be possible as long as it's checked before
  //   }
  //
  //   (left, right)
  // }
  //
  // fn get_whitespace_split_for_grid_height(
  //   difference: f32,
  //   origin: (usize, usize),
  // ) -> (usize, usize) {
  //   let terminal_height = TERMINAL_DIMENSIONS.1;
  //
  //   todo!()
  // }

  fn overwrite_currently_printed_grid(
    &mut self,
    new_grid: &str,
    new_grid_dimensions: Option<(usize, usize)>,
  ) -> Result<(), PrintingError> {
    let (new_grid_width, new_grid_height) = if let Some(new_grid_dimensions) = new_grid_dimensions {
      new_grid_dimensions
    } else {
      valid_rectangle_check(new_grid)?
    };

    self.grid_width = Some(new_grid_width);
    self.grid_height = Some(new_grid_height);

    self.origin_position = Some(self.get_new_origin((new_grid_width, new_grid_height))?);

    self.move_to_origin()?;
    // Here would be a method to replace any '\n' with an escape code.
    print!("{}", new_grid);

    Ok(())
  }
}

trait VecMethods<T> {
  /// Gets a mutable reference to the top most item in a vector
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
  /// Converts an index into coordinates
  fn index_as_coordinates(&self, grid_width: &Self) -> (usize, usize);
}

impl UsizeMethods for usize {
  fn index_as_coordinates(&self, grid_width: &Self) -> (usize, usize) {
    (self % grid_width, self / grid_width)
  }
}

fn valid_rectangle_check(rectangle_shape: &str) -> Result<(usize, usize), PrintingError> {
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
