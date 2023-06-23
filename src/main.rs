use std::thread;
use std::time::SystemTime;
use std::time::Duration as StdDuration;
use std::io::{self, Write};
use chrono_humanize::HumanTime;
use humantime::parse_duration;
use chrono::Duration as ChronoDuration;

fn main() {
    println!("Please enter a duration (e.g., '1h30m'):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let countdown_duration: StdDuration;

    match parse_duration(&input.trim()) {
        Ok(duration_int) => {
            println!("Input duration: {:?}", duration_int);
            countdown_duration = duration_int;
            // Perform further operations with the parsed duration value if needed
        },
        Err(_) => {
            println!("Invalid duration input using 5 seconds instead");
            countdown_duration = StdDuration::new(5, 0);
        }
    }
    let start_time = SystemTime::now();
    let end_time = start_time + countdown_duration;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        let current_time = SystemTime::now();

        if current_time >= end_time {
            break;
        }

        let remaining_time = end_time.duration_since(current_time).unwrap();
        let humantime = HumanTime::from(ChronoDuration::from_std(remaining_time).unwrap());
        //let remaining_secs = remaining_time.as_secs();
        //let minutes = remaining_secs / 60;
        //let seconds = remaining_secs % 60;

        //write!(handle, "\rCountdown: {:02}:{:02}", minutes, seconds).unwrap();
        write!(handle, "\rCountdown: {}", humantime).unwrap();
        handle.flush().unwrap();

        thread::sleep(StdDuration::from_secs(1));
    }
    println!("\nCountdown finished!");
    loop {
      let current_time = SystemTime::now();
      let remaining_time = current_time.duration_since(end_time).unwrap();
      //let remaining_time = end_time.duration_since(current_time).unwrap();
        //let ht = chrono_humanize::HumanTime::from(remaining_time);
        let remaining_secs = remaining_time.as_secs();
        let minutes = remaining_secs / 60;
        let seconds = remaining_secs % 60;

        write!(handle, "\rCountdown: -{:02}:{:02}", minutes, seconds).unwrap();
        handle.flush().unwrap();

        thread::sleep(StdDuration::from_secs(1));
    }
}
