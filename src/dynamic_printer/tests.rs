#![cfg(test)]

use super::*;

#[cfg(test)]
mod get_printable_difference_logic {
  use super::*;

  /// Gets the [`BASE_GRID`](BASE_GRID), and changes the characters as the passed in list of indices.
  /// The characters will be replaced with l.
  ///
  /// The indices will apply to any newlines, so make sure to account for those.
  fn get_modified_base_grid(indices: Vec<usize>) -> String {
    indices
      .into_iter()
      .fold(BASE_GRID.to_string(), |mut base_grid, index| {
        base_grid.remove(index);
        base_grid.insert(index, 'l');

        base_grid
      })
  }

  #[test]
  fn one_pixel_difference() {
    // lbcde
    // 12345
    // vwxyz
    let printer = get_preassigned_printer();
    let different_grid = get_modified_base_grid(vec![0]);
    let origin = printer.get_origin_position().unwrap();

    let expected_different_pixels = PixelDifference {
      pixels: String::from("l"),
      index: 0,
    }
    .into_printable_difference(origin, GRID_SIZES.0);

    let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

    assert_eq!(expected_different_pixels, different_pixels);
  }

  #[cfg(test)]
  mod two_pixel_difference {
    use super::*;

    #[test]
    fn different_pixels_are_together() {
      // llcde
      // 12345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 1]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("ll"),
        index: 0,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn different_pixels_are_together_split_by_newline() {
      // abcdl
      // l2345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![4, 6]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("l\nl"),
        index: 4,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn different_pixels_are_together_right_before_newline() {
      // abcll
      // 12345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![3, 4]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("ll"),
        index: 3,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn different_pixels_are_together_right_after_newline() {
      // abcde
      // ll345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![7, 8]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("ll"),
        index: 6,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn different_pixels_are_apart() {
      // alcde
      // 1l345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![1, 8]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = [
        PixelDifference {
          pixels: String::from("l"),
          index: 1,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l"),
          index: 7,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn different_pixels_touch_last_index() {
      // abcde
      // 12345
      // vwxll
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![15, 16]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("ll"),
        index: 13,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }
  }

  #[cfg(test)]
  mod three_pixel_difference {
    use super::*;

    #[test]
    fn different_pixels_are_together() {
      // lllde
      // 1l345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 1, 2]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("lll"),
        index: 0,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn first_two_pixels_together() {
      // llcde
      // 1l345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 1, 8]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = [
        PixelDifference {
          pixels: String::from("ll"),
          index: 0,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l"),
          index: 7,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn last_two_pixels_together() {
      // alcde
      // ll345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![1, 7, 8]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = [
        PixelDifference {
          pixels: String::from("l"),
          index: 1,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("ll"),
          index: 6,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn last_two_pixels_together_split_by_newline() {
      // alcdl
      // l2345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![1, 4, 6]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = [
        PixelDifference {
          pixels: String::from("l"),
          index: 1,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l\nl"),
          index: 4,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }
  }

  #[cfg(test)]
  mod four_pixel_difference {
    use super::*;

    #[test]
    fn different_pixels_are_together() {
      // lllle
      // 12345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 1, 2, 3]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels = PixelDifference {
        pixels: String::from("llll"),
        index: 0,
      }
      .into_printable_difference(origin, GRID_SIZES.0);

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn split_pixel_pairs() {
      // llcde
      // ll345
      // vwxyz
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 1, 6, 7]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels: String = [
        PixelDifference {
          pixels: String::from("ll"),
          index: 0,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("ll"),
          index: 5,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }

    #[test]
    fn all_pixels_split() {
      // lbcdl
      // 12345
      // lwxyl
      let printer = get_preassigned_printer();
      let different_grid = get_modified_base_grid(vec![0, 4, 12, 16]);
      let origin = printer.get_origin_position().unwrap();

      let expected_different_pixels: String = [
        PixelDifference {
          pixels: String::from("l"),
          index: 0,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l"),
          index: 4,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l"),
          index: 10,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
        PixelDifference {
          pixels: String::from("l"),
          index: 14,
        }
        .into_printable_difference(origin, GRID_SIZES.0),
      ]
      .join("");

      let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

      assert_eq!(expected_different_pixels, different_pixels);
    }
  }

  #[test]
  fn entire_grid_difference() {
    // lllll
    // lllll
    // lllll
    let printer = get_preassigned_printer();
    let different_grid =
      get_modified_base_grid(vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16]);
    let origin = printer.get_origin_position().unwrap();

    let expected_different_pixels = PixelDifference {
      pixels: String::from("lllll\nlllll\nlllll"),
      index: 0,
    }
    .into_printable_difference(origin, GRID_SIZES.0);

    let different_pixels = printer.get_printable_difference(&different_grid).unwrap();

    assert_eq!(expected_different_pixels, different_pixels);
  }
}

#[cfg(test)]
mod get_origin_from_printing_potision_tests {
  use super::*;

  #[test]
  fn x_left_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_x_printing_position(XPrintingPosition::Left);

    let expected_x_position = 1;

    let (origin_x, _) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_x, expected_x_position);
  }

  #[test]
  fn x_middle_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_x_printing_position(XPrintingPosition::Middle);

    let expected_x_position = ((terminal_width as f32 / 2.0).floor()
      - (grid_dimensions.0 as f32 / 2.0).floor())
    .floor() as usize;

    let (origin_x, _) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_x, expected_x_position);
  }

  #[test]
  fn x_right_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_x_printing_position(XPrintingPosition::Right);

    let expected_x_position = (terminal_width - grid_dimensions.0) + 1;

    let (origin_x, _) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_x, expected_x_position);
  }

  #[test]
  fn y_top_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_y_printing_position(YPrintingPosition::Top);

    let expected_y_position = 1;

    let (_, origin_y) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_y, expected_y_position);
  }

  #[test]
  fn y_middle_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_y_printing_position(YPrintingPosition::Middle);

    let expected_y_position = ((terminal_height as f32 / 2.0).floor()
      - (grid_dimensions.1 as f32 / 2.0).floor())
    .floor() as usize;

    let (_, origin_y) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_y, expected_y_position);
  }

  #[test]
  fn y_bottom_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    printer
      .replace_y_printing_position(YPrintingPosition::Bottom);

    let expected_y_position = (terminal_height - grid_dimensions.1) + 1;

    let (_, origin_y) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_y, expected_y_position);
  }

  #[test]
  fn y_custom_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    let printing_position = (terminal_height / 10).max(1);
    printer
      .replace_y_printing_position(YPrintingPosition::Custom(printing_position));

    let expected_y_position = printing_position
      - ((printing_position + grid_dimensions.1) as isize - terminal_height as isize).max(0)
        as usize;

    let (_, origin_y) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_y, expected_y_position);
  }

  #[test]
  fn x_custom_position() {
    let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
    let mut printer = get_preassigned_printer();
    let grid_dimensions = GRID_SIZES;
    let printing_position = (terminal_width / 10).max(1);
    printer
      .replace_x_printing_position(XPrintingPosition::Custom(printing_position));

    let expected_x_position = printing_position
      - ((printing_position + grid_dimensions.1) as isize - terminal_width as isize).max(0)
        as usize;

    let (origin_x, _) = printer.get_new_origin(grid_dimensions, (terminal_width, terminal_height));

    assert_eq!(origin_x, expected_x_position);
  }
}

#[test]
fn anyhow_compatibility() {
  fn return_anyhow_error() -> anyhow::Result<()> {
    Err(PrintingError::OriginNotDefined)?;

    Ok(())
  }

  let _ = return_anyhow_error();
}

// Base grid will be
// abcde
// 12345
// vwxyz
static BASE_GRID: &str = "abcde\n12345\nvwxyz";
static GRID_SIZES: (usize, usize) = (5, 3);

fn get_preassigned_printer() -> Printer {
  let terminal_dimensions = Printer::get_terminal_dimensions().unwrap();
  let (grid_width, grid_height) = Printer::get_rectangular_dimensions(BASE_GRID).unwrap();

  let mut printer = Printer::new_with_printing_position(
    PrintingPosition::with_y_printing_position(YPrintingPosition::Top),
  );

  printer.previous_grid = BASE_GRID.to_string();
  printer.update_dimensions((grid_width, grid_height));
  printer.update_origin(printer.get_new_origin((grid_width, grid_height), terminal_dimensions));

  printer
}

// Was used before, now here just to make rewriting tests easier.
#[derive(Debug, PartialEq)]
struct PixelDifference {
  pixels: String,
  index: usize,
}

impl PixelDifference {
  fn into_printable_difference(self, origin: (usize, usize), grid_width: usize) -> String {
    let (mut x, mut y) = self.index.index_as_coordinates(&grid_width);
    x = (x + origin.1).max(1);
    y = (y + origin.1).max(1);

    let mut printable_difference = String::new();

    for pixels in self.pixels.split('\n') {
      println!("x: {x}, y: {y}");
      //"\x1B[{y};{x}H{pixels}"
      printable_difference.push_str(&format!("\x1B[{y};{x}H{}", pixels));

      y += 1;
      x = (x as isize - (grid_width as isize - 1)).max(1) as usize;
    }

    printable_difference
  }
}
