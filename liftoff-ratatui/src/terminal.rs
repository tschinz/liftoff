use std::io::{self, Write, StdoutLock};
use std::time::Duration as StdDuration;
use humantime::parse_duration;

pub fn get_input_user() -> StdDuration {
  println!("Please enter a duration (e.g., '1h30m20s'):");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let duration =  parse_duration(&input.trim()).unwrap_or(StdDuration::new(5, 0));
  return duration
}

pub fn print_terminal(handle: &mut StdoutLock<'_>, output: String) {
  write!(handle, "{}", output).unwrap();
  handle.flush().unwrap();
}

pub fn get_terminal_lock() -> StdoutLock<'static> {
  let stdout = io::stdout();
  let stdout_lock = stdout.lock();
  stdout_lock
}

pub fn flush_terminal(handle: &mut StdoutLock<'_>) {
  handle.flush().unwrap();
}