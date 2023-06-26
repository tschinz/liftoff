#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::SystemTime;
use std::time::Duration as StdDuration;
use humantime::parse_duration;
use eframe::egui;

mod ui;
mod logic;

struct Liftoff {
  duration: StdDuration,
  input: String,
  start_time: SystemTime,
  end_time: SystemTime,
  millis : bool,
  human_readable : bool,
  running: bool,
  negative: bool,
}

impl Default for Liftoff {
  fn default() -> Self {
    Self {
      duration: StdDuration::new(5, 0),
      input: String::from("5s"),
      start_time: SystemTime::now(),
      end_time: SystemTime::now(),
      millis: false,
      human_readable: false,
      running: false,
      negative: false,
    }
  }
}

impl eframe::App for Liftoff {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
  }

  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ui::window_frame(ctx, frame, "Liftoff", |ui| {
      let rem_time: StdDuration;
      if self.running {
        let cur_time = SystemTime::now();
        if cur_time <= self.end_time {
          rem_time = self.end_time.duration_since(cur_time).unwrap();
          self.negative = true;
        } else {
          rem_time = cur_time.duration_since(self.end_time).unwrap();
          self.negative = false;
        }
      } else {
        rem_time = self.duration;
      }
      if self.human_readable {
        ui.vertical_centered(|ui|{
          ui.heading(logic::time_string_human(rem_time, None));
        });

      } else {
        ui.vertical_centered(|ui|{
          ui.heading(logic::time_string(rem_time, Some(self.negative), self.millis));
        });
      }
      ui.text_edit_singleline(&mut self.input).on_hover_text("Enter a duration e.g. 1h30m20s");

      ui.horizontal_centered(|ui| {
        ui.checkbox(&mut self.millis, "ms");
        ui.checkbox(&mut self.human_readable, "human readable");
      });

      // Actions on keybuttons
      if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if !self.running {
          self.duration =  parse_duration(self.input.trim()).unwrap_or(StdDuration::new(5, 0));
        }
      }
      if ui.input(|i| i.key_pressed(egui::Key::Space)) {
        if self.running {
          self.running = false;
          self.duration = rem_time;
        } else {
          self.running = true;
          if !self.negative {
            self.duration =  parse_duration(self.input.trim()).unwrap_or(StdDuration::new(5, 0));
          }
          self.start_time = SystemTime::now();
          self.end_time = self.start_time + self.duration;
        }
      }
    if self.running {
      ctx.request_repaint();
    }

    });
  }
}


fn main() {
  // Log to stdout (if you run with `RUST_LOG=debug`).
  //tracing_subscriber::fmt::init();

  let options = eframe::NativeOptions {
    decorated: false,
    transparent: true,
    icon_data: Some(ui::load_icon()),
    min_window_size: Some(egui::vec2(200.0, 100.0)),
    initial_window_size: Some(egui::vec2(200.0, 100.0)),
    ..Default::default()
  };

  eframe::run_native(
    "Liftoff",
    options,
    Box::new(|_cc| Box::new(Liftoff::default())),
  ).unwrap();

}