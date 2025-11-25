/// The PrintingPosition is a way to preset a place to print a grid on the screen with the [`Printer`](crate::printer::Printer).
///
/// By combining an x and y position you can print any grid in 9 positions on the grid.
///
/// Say if you wanted to print your grid to the very center of the screen, you'd set that up like so:
/// ```
/// use screen_printer::prelude::*;
///
/// let printing_position =
///   PrintingPosition::new(XPrintingPosition::Middle, YPrintingPosition::Middle);
/// let mut printer = Printer::new_with_printing_position(printing_position);
/// ```
///
/// This would make any following use of the [`dynamic_print`](crate::dynamic_printer::DynamicPrinter::dynamic_print) method print the grid to the center
/// of the screen.
///
/// For more information about adjusting the PrintingPosition, refer to the examples on [`github`](https://github.com/LinkTheDot/screen_printer/tree/master/examples) (see `preset_printing_positions.rs` and `custom_printing_position.rs`).
///
/// For more information about printing, refer to documentation on the [`Printer`](crate::printer::Printer) and [`dynamic_print`](crate::dynamic_printer::DynamicPrinter::dynamic_print) method.
#[derive(Debug, Default, Clone)]
pub struct PrintingPosition {
  pub x_printing_position: XPrintingPosition,
  pub y_printing_position: YPrintingPosition,
}

/// The list of X positions to print a grid on the screen to.
///
/// Used for [`PrintingPosition`](PrintingPosition).
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum XPrintingPosition {
  #[default]
  Left,
  Middle,
  Right,
  Custom(usize),
}

/// The list of Y positions to print a grid on the screen to.
///
/// Used for [`PrintingPosition`](PrintingPosition).
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum YPrintingPosition {
  Top,
  Middle,
  #[default]
  Bottom,
  Custom(usize),
}

impl PrintingPosition {
  /// Creates a new [`PrintingPosition`](PrintingPosition) with the given [`X`](XPrintingPosition) and [`Y`](YPrintingPosition) positions.
  pub fn new(
    x_printing_position: XPrintingPosition,
    y_printing_position: YPrintingPosition,
  ) -> Self {
    Self {
      x_printing_position,
      y_printing_position,
    }
  }

  /// Creates a new [`PrintingPosition`](PrintingPosition) with the given [`X`](XPrintingPosition) position, defaulting on the y position.
  pub fn with_x_printing_position(x_printing_position: XPrintingPosition) -> Self {
    Self {
      x_printing_position,
      ..Default::default()
    }
  }

  /// Creates a new [`PrintingPosition`](PrintingPosition) with the given [`Y`](YPrintingPosition) position, defaulting on the X position.
  pub fn with_y_printing_position(y_printing_position: YPrintingPosition) -> Self {
    Self {
      y_printing_position,
      ..Default::default()
    }
  }
}

impl From<(XPrintingPosition, YPrintingPosition)> for PrintingPosition {
  fn from(item: (XPrintingPosition, YPrintingPosition)) -> Self {
    Self {
      x_printing_position: item.0,
      y_printing_position: item.1,
    }
  }
}
