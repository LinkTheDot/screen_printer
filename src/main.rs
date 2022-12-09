#![allow(dead_code)]

use rand::prelude::*;
use std::{io, thread, time::Duration};
use terminal_printing::printer::*;
use termion::cursor::DetectCursorPos;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

fn main() {
  random_number_printing();
}

fn manual_testing() {
  let width = 5;
  let height = 3;

  print!("{}", "\n".repeat(height * 10));

  let mut printer = Printer::new(width, height);

  //abcde
  //12345
  //vwxyz
  let grid1 = String::from("abcde\n12345\nvwxyz");
  // a-c--
  // -23--
  // -wx--
  let grid2 = String::from("a-c--\n-----\n-wx--");

  let _ = printer.dynamic_print(grid1);
  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(grid2);

  // let _ = io::stdout().flush();
  thread::sleep(Duration::from_millis(1000));
}

fn cursor_position() -> (u16, u16) {
  let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

  let cursor_position = stdout.cursor_pos();

  match cursor_position {
    Ok(position) => position,
    Err(error) => panic!("{error}"),
  }
}

fn printing_test() {
  let height = 3;
  let width = 3;

  let mut printer = Printer::new(width, height);
  let a_grid = Printer::create_grid_from_single_character(&"a", width, height);
  let b_grid = Printer::create_grid_from_single_character(&"b", width, height);
  let c_grid = Printer::create_grid_from_single_character(&"c", width, height);

  let rows = vec!["aaa", "bbb", "ccc"];
  let abc_grid = Printer::create_grid_from_multiple_rows(&rows).unwrap();

  print!("{}", "\n".repeat(10));
  // println!("{:?}", abc_grid);
  // println!("{:?}", a_grid);
  // println!("{:?}", b_grid);
  // println!("{:?}", c_grid);

  thread::sleep(Duration::from_millis(500));
  let _ = printer.dynamic_print(a_grid);

  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(abc_grid.clone());

  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(b_grid);

  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(abc_grid.clone());

  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(c_grid);

  thread::sleep(Duration::from_millis(1000));
  let _ = printer.dynamic_print(abc_grid);
}

fn random_number_printing() {
  let width = 175;
  let height = 40;

  println!("{}", "\n".repeat(height + 1));

  let mut printer = Printer::new(width, height);

  for _ in 0..100000 {
    let number_array = get_random_number_array(width * height);
    let grid = Printer::create_grid_from_full_character_list(&number_array, width, height).unwrap();

    let _ = printer.dynamic_print(grid);

    thread::sleep(Duration::from_millis(24));
  }
}

fn get_random_number_array(total_size: usize) -> Vec<u16> {
  let mut rng = rand::thread_rng();

  (0..total_size).fold(Vec::new(), |mut number_array, _| {
    number_array.push(rng.gen_range(0..9));

    number_array
  })
}
