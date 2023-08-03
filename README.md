# Screen Printer

Screen Printer is a Rust crate that will allow you to build and print arrays of
data into a grid format.

The purpose of this crate is to make it easier to print rectangular blocks of text to the terminal.
Including features like:

- `DynamicPrint`, which only prints any characters that changed from any previously printed grid\*.
- `PrintingPosition`, which allows you to print your string to different places on the terminal, such as the center.

\* If the grid changes in size or position it is reprinted in its entirety.

## Examples

#### Using the dynamic print method to print a grid

The core part of this crate is the [`dynamic_print`](crate::dynamic_printer::DynamicPrinter::dynamic_print) method for the [`Printer`](crate::printer::Printer).
This will take a rectangular grid of characters and print only the parts of the grid that have changed since the last print.

```rust,no_run
use screen_printer::printer::*;

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

fn main() {
  print!("\u{1b}[2J"); // Clear all text on the terminal
  // The default printing position is the bottom left of the terminal
  let mut printer = Printer::new_with_printing_position(PrintingPosition::default());

  // Create the first grid to be printed.
  let grid_1 = "abc\n123\nxyz".to_string();
  // print the first grid.
  printer.dynamic_print(grid_1).unwrap();

  // Wait before printing the second grid.
  std::thread::sleep(std::time::Duration::from_millis(500));

  // Create the second grid to be printed.
  let grid_2 = "abc\n789\nxyz".to_string();
  // Print the second grid.
  // This will only end up printing the difference between the two grids/
  printer.dynamic_print(grid_2).unwrap();
}
```

This will result in:

```bash,no_run
abc
123
xyz
```

Into

```bash,no_run
abc
789 < only line that was actually printed
xyz
```

#### Printing Position

Another feature shown in the above example, the [`PrintingPosition`](crate::printer::PrintingPosition).

This will print the grid in any of the 9 defined positions on the terminal.
These are split by the X and Y axes:

- Left/Top,
- Middle, and
- Right/Bottom.

# What is a "rectangular grid"?

A grid is referring to a "grid" of characters, AKA a string with rows and columns.
Each row of the "grid" would be sets of characters separated by newlines.
Each column would be an individual character between the newlines.

A 3x2 "grid" would be something like: `"xxx\nxxx"`
Each `x` on either side of the `\n` is like a column and the `\n` separates each row.

For a grid to "not be rectangular" would mean that a row has a differing amount of characters from every other,
like so: `"xx\nxxx"`
