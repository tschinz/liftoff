use std::thread;
use std::time::SystemTime;
use std::time::Duration as StdDuration;
use std::io::{self, Write, StdoutLock};
use chrono_humanize::{HumanTime, Accuracy, Tense};
use humantime::parse_duration;
use chrono::Duration as ChronoDuration;

fn main() {
  // Config
  let ht = false;
  let print_millis = false;

  // get input from user
  let countdown_duration = get_input_user();

  // get start and end time
  let start_time = SystemTime::now();
  let end_time = start_time + countdown_duration;

  // get terminal lock
  let stdout = io::stdout();
  let mut handle = stdout.lock();

  // T- loop
  loop {
    let cur_time = SystemTime::now();

    if cur_time >= end_time {
      break;
    }
    let rem_time = end_time.duration_since(cur_time).unwrap();
    if ht {
      print_human_time(rem_time, &mut handle, Some(true));
    }
    else {
      print_time(rem_time, &mut handle, Some(true), print_millis);
    }

    if print_millis {
      thread::sleep(StdDuration::from_millis(100));
    }
    else {
      thread::sleep(StdDuration::from_secs(1));
    }
  }
  handle.flush().unwrap();
  println!("\rLiftoff...!");

  // T+ loop
  loop {
    let cur_time = SystemTime::now();
    let rem_time = cur_time.duration_since(end_time).unwrap();

    if ht {
      print_human_time(rem_time, &mut handle, Some(false));
    }
    else {
      print_time(rem_time, &mut handle, Some(false), print_millis);
    }
    if print_millis {
      thread::sleep(StdDuration::from_millis(100));
    }
    else {
      thread::sleep(StdDuration::from_secs(1));
    }
  }
}

fn get_input_user() -> StdDuration {
  println!("Please enter a duration (e.g., '1h30m20s'):");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let duration =  parse_duration(&input.trim()).unwrap_or(StdDuration::new(5, 0));
  return duration
}

fn print_human_time(time: StdDuration, handle: &mut StdoutLock<'_>, neg: Option<bool>) {
  let time_chrono = ChronoDuration::from_std(time).unwrap();
  let humantime = HumanTime::from(time_chrono);
  match neg {
    Some(neg) =>
      if neg {
        write!(handle, "\rT-{}", humantime.to_text_en(Accuracy::Precise, Tense::Future)).unwrap();
      } else {
        write!(handle, "\rT+{}", humantime.to_text_en(Accuracy::Precise, Tense::Future)).unwrap();
      },
    None => write!(handle, "\r{}", humantime.to_text_en(Accuracy::Precise, Tense::Future)).unwrap(),
  }
  handle.flush().unwrap();
}

fn split_duration(time: StdDuration) -> (u8, u128, u128, u128, u128, u128) {
  let mut millis = time.as_millis();
  let mut secs: u128 = 0;
  let mut mins: u128 = 0;
  let mut hours: u128 = 0;
  let mut days: u128 = 0;
  let mut size: u8 = 1;

  if millis > 1000 {
    secs = millis / 1000;
    size += 1;
  }
  if millis > 1000*60 {
    mins =  secs / 60;
    size += 1;
  }
  if millis > 1000*60*60 {
    hours =  mins / 60;
    size += 1;
  }
  if millis > 1000*60*60*24 {
    days =  hours / 60;
    size += 1;
  }
  hours = hours % 24;
  mins = mins % 60;
  secs = secs % 60;
  millis = millis % 1000;

  return (size, days, hours, mins, secs, millis)
}

fn print_time(time: StdDuration, handle: &mut StdoutLock<'_>, neg: Option<bool>, print_millis: bool){
  let (size, millis, secs, mins, hours, days): (u8, u128, u128, u128, u128, u128);
  (size, days, hours, mins, secs, millis) = split_duration(time);

  let mut output = String::new();

  match neg {
    Some(neg) =>
      if neg {
        output += "\rT-";
      } else {
        output += "\rT+";
      },
    None => output += "\r",
  }

  if print_millis {
    if size >= 5 {
      output = format!("{}{}d{:02}h{:02}m{:02}s{:03}ms", output, days, hours, mins, secs, millis);
    } else if size >= 4 {
      output = format!("{}{}h{:02}m{:02}s{:03}ms", output, hours, mins, secs, millis);
    } else if size >= 3 {
      output = format!("{}{}m{:02}s{:03}ms", output, mins, secs, millis);
    } else if size >= 2 {
      output = format!("{}{}s{:03}ms", output, secs, millis);
    } else if size >= 1 {
      output = format!("{}{}ms", output, millis);
    }
  } else {
if size >= 5 {
      output = format!("{}{}d{:02}h{:02}m{:02}s", output, days, hours, mins, secs);
    } else if size >= 4 {
      output = format!("{}{}h{:02}m{:02}s", output, hours, mins, secs);
    } else if size >= 3 {
      output = format!("{}{}m{:02}s", output, mins, secs);
    } else if size >= 2 {
      output = format!("{}{}s", output, secs);
    }
  }

  write!(handle, "{}", output).unwrap();
  handle.flush().unwrap();
}
