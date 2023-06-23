use std::time::Duration as StdDuration;
use chrono_humanize::{HumanTime, Accuracy, Tense};
use chrono::Duration as ChronoDuration;

pub fn time_string_human(time: StdDuration, neg: Option<bool>) -> String {
  let time_chrono = ChronoDuration::from_std(time).unwrap();
  let humantime = HumanTime::from(time_chrono);
  let humantime = humantime.to_text_en(Accuracy::Precise, Tense::Future);

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
  format!("{}{}                                 ", output, humantime)
}

pub fn time_string(time: StdDuration, neg: Option<bool>, print_millis: bool) -> String {
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
      output = format!("{}{}d{:02}h{:02}m{:02}s{:03}ms        ", output, days, hours, mins, secs, millis);
    } else if size >= 4 {
      output = format!("{}{}h{:02}m{:02}s{:03}ms              ", output, hours, mins, secs, millis);
    } else if size >= 3 {
      output = format!("{}{}m{:02}s{:03}ms                    ", output, mins, secs, millis);
    } else if size >= 2 {
      output = format!("{}{}s{:03}ms                          ", output, secs, millis);
    } else if size >= 1 {
      output = format!("{}{}ms                                ", output, millis);
    }
  } else {
    if size >= 5 {
      output = format!("{}{}d{:02}h{:02}m{:02}s               ", output, days, hours, mins, secs);
    } else if size >= 4 {
      output = format!("{}{}h{:02}m{:02}s                     ", output, hours, mins, secs);
    } else if size >= 3 {
      output = format!("{}{}m{:02}s                           ", output, mins, secs);
    } else if size >= 2 {
      output = format!("{}{}s                                 ", output, secs);
    }
  }
  output
}

pub fn split_duration(time: StdDuration) -> (u8, u128, u128, u128, u128, u128) {
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
