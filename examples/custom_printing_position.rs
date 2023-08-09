use screen_printer::printer::*;

const WAIT_TIME_MILLIS: u64 = 300;
const WIDTH: usize = 20;
const HEIGHT: usize = 10;

fn main() {
  let _cursor_hider = termion::cursor::HideCursor::from(std::io::stdout());
  print!("{}", termion::clear::All);

  let (terminal_width, terminal_height) = Printer::get_terminal_dimensions().unwrap();
  let printing_position = PrintingPosition::new(
    XPrintingPosition::Custom(terminal_width / 4),
    YPrintingPosition::Custom(terminal_height / 4),
  );
  let mut printer = Printer::new_with_printing_position(printing_position);

  let mut grids = vec![
    Printer::create_grid_from_single_character('|', WIDTH, HEIGHT),
    Printer::create_grid_from_single_character('/', WIDTH, HEIGHT),
    Printer::create_grid_from_single_character('-', WIDTH, HEIGHT),
    Printer::create_grid_from_single_character('\\', WIDTH, HEIGHT),
  ]
  .into_iter()
  .cycle();

  for _ in 0..100 {
    printer.dynamic_print(grids.next().unwrap()).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(WAIT_TIME_MILLIS));
  }

  print!("{}", termion::clear::All);
}
