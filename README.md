# Screen Printer

A Rust crate for displaying rectangular blocks of text to a terminal with dynamic updates and flexible positioning.

## Overview

Screen Printer is a Rust crate that will allow you to build and print arrays of
data into a grid format.

The purpose of this crate is to make it easier to print rectangular blocks of text to the terminal.

## Features

- **Dynamic Printing**: Only prints characters that changed from any previously printed grid\*, making updates efficient.
- **Flexible Positioning**: Print grids to any of 9 preset positions on the terminal (combinations of Left/Middle/Right and Top/Middle/Bottom) or custom positions.
- **Grid Operations**: Create grids from single characters or arrays of data.
- **Terminal Awareness**: Automatically handles terminal dimensions and prevents overflow.

\* If the grid changes in size or position it is reprinted in its entirety.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
screen_printer = "0.2.7"
```

## Examples

#### Using the dynamic print method to print a grid

The core part of this crate is the `dynamic_print` method for the `Printer`.
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
  // This will only end up printing the difference between the two grids.
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

Another feature shown in the above example is the `PrintingPosition`.

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

## More Examples

This crate includes several examples demonstrating different features:

- **`dynamic_printer.rs`**: Shows how to use dynamic printing with changing grid sizes
- **`preset_printing_positions.rs`**: Demonstrates the 9 preset printing positions
- **`custom_printing_position.rs`**: Shows how to use custom X and Y positions
- **`creating_grids.rs`**: Examples of creating grids from characters
- **`change_grid_dimensions.rs`**: How to handle changing grid dimensions

Run an example with:
```bash
cargo run --example dynamic_printer
```

## API Documentation

The main types you'll work with are:

- **`Printer`**: The main struct for managing grid printing
  - `new()`: Creates a printer with default positioning (Bottom-Left)
  - `new_with_printing_position()`: Creates a printer with custom positioning
  - `dynamic_print()`: Prints a grid, only updating changed characters
  - `create_grid_from_single_character()`: Helper to create uniform grids
  - `create_grid_from_full_character_list()`: Creates grids from character arrays

- **`PrintingPosition`**: Controls where grids are printed on the terminal
  - Combines `XPrintingPosition` (Left/Middle/Right/Custom) with `YPrintingPosition` (Top/Middle/Bottom/Custom)

- **`PrintingError`**: Error type for printing operations
  - Includes detailed error messages for common issues like non-rectangular grids or oversized grids

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
