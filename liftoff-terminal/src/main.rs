use std::thread;
use std::time::SystemTime;
use std::time::Duration as StdDuration;

mod terminal;
mod logic;

fn main() {
  // Config
  let ht = false;
  let print_millis = false;

  // get input from user
  let countdown_duration = terminal::get_input_user();

  // get start and end time
  let start_time = SystemTime::now();
  let end_time = start_time + countdown_duration;

  // get terminal lock
  let mut handle = terminal::get_terminal_lock();


  // T- loop
  loop {
    let cur_time = SystemTime::now();

    if cur_time >= end_time {
      break;
    }
    let rem_time = end_time.duration_since(cur_time).unwrap();
    let output: String;
    if ht {
      output = logic::time_string_human(rem_time, Some(true));
    }
    else {
      output = logic::time_string(rem_time, Some(true), print_millis);
    }
    terminal::print_terminal(&mut handle, output);

    if print_millis {
      thread::sleep(StdDuration::from_millis(100));
    }
    else {
      thread::sleep(StdDuration::from_millis(200));
    }
  }
  terminal::flush_terminal(&mut handle);
  println!("\rLiftoff...!");

  // T+ loop
  loop {
    let cur_time = SystemTime::now();
    let rem_time = cur_time.duration_since(end_time).unwrap();
    let output: String;
    if ht {
      output = logic::time_string_human(rem_time, Some(false));
    }
    else {
      output = logic::time_string(rem_time, Some(false), print_millis);
    }
    terminal::print_terminal(&mut handle, output);
    if print_millis {
      thread::sleep(StdDuration::from_millis(100));
    }
    else {
      thread::sleep(StdDuration::from_secs(1));
    }
  }
}