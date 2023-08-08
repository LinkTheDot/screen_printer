use screen_printer::printer::*;
#[allow(unused)]
use std::{thread, time::Duration};

fn main() {
  let _cursor_hider = termion::cursor::HideCursor::from(std::io::stdout());
  println!("{}", termion::clear::All);
  let mut printer = Printer::new_with_printing_position(PrintingPosition::new(
    XPrintingPosition::Middle,
    YPrintingPosition::Middle,
  ));

  let printing_list = vec![
    "x".to_string(),
    "xx".to_string(),
    "xxx".to_string(),
    "xxxx".to_string(),
    "xxxx\nxxxx".to_string(),
    "xxxxx\nxxxxx".to_string(),
    "xxxxxx\nxxxxxx".to_string(),
    "xxxxxx\nxxxxxx\nxxxxxx".to_string(),
    "xxxxxxx\nxxxxxxx\nxxxxxxx".to_string(),
    "xxxxxxxx\nxxxxxxxx\nxxxxxxxx".to_string(),
    "xxxxxxxx\nxxxxxxxx\nxxxxxxxx\nxxxxxxxx".to_string(),
    "xxxxxxxxx\nxxxxxxxxx\nxxxxxxxxx\nxxxxxxxxx".to_string(),
    "xxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxxxx".to_string(),
    "xxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxxxx".to_string(),
  ];

  for grid in printing_list.clone() {
    printer.dynamic_print(grid).unwrap();

    thread::sleep(Duration::from_millis(400));
  }

  for grid in printing_list.into_iter().rev() {
    printer.dynamic_print(grid).unwrap();

    thread::sleep(Duration::from_millis(400));
  }
  println!("{}", termion::clear::All);
}
