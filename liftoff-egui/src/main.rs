#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::SystemTime;
use std::time::Duration as StdDuration;
use humantime::parse_duration;
use eframe::egui;

mod ui;
mod logic;

// App state
struct Liftoff {
  duration: StdDuration,
  input: String,
  start_time: SystemTime,
  end_time: SystemTime,
  millis : bool,
  human_readable : bool,
  running: bool,
  negative: bool,
  n_items: usize,
  list: Vec<String>,
}

// App creation method
impl Liftoff{
  fn new(cc: &eframe::CreationContext<'_>) -> Self {
    ui::setup_font(&cc.egui_ctx);
    Liftoff::default()
  }
}

// default state of the app
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
      n_items: 0,
      list: Vec::new(),
    }
  }
}
// App functions
impl eframe::App for Liftoff {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
  }


  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    // window frame
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
      // Heading
      let liftoff_text: String;
      if self.human_readable {
        liftoff_text = logic::time_string_human(rem_time, None);
        ui.vertical_centered(|ui|{
          ui.heading(liftoff_text.clone());
        });
      } else {
        liftoff_text = logic::time_string(rem_time, Some(self.negative), self.millis);
        ui.vertical_centered(|ui|{
          ui.heading(liftoff_text.clone());
        });
      }

      ui.group(|ui| {
        ui.vertical_centered(|ui| {
          // Edit field
          ui.text_edit_singleline(&mut self.input).on_hover_text("Enter a duration e.g. 1h30m20s");
        });
        // Checkboxes
        ui.horizontal(|ui|{
          ui.add_space(ui.available_width() / 2.0-70.0);
          ui.checkbox(&mut self.millis, "ms");
          ui.add_space(4.0);
          ui.checkbox(&mut self.human_readable, "human readable");
          ui.add_space(ui.available_width() / 2.0);
        });
      });

      // List view
        ui.vertical_centered(|ui| {
          eframe::egui::ScrollArea::vertical().show(
            ui,
            |ui| {
              for item in self.list.iter().rev() {
                ui.label(item);
              }
            },
          );
    });

      // Actions on keybuttons
      if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if !self.running {
          self.duration =  parse_duration(self.input.trim()).unwrap_or(StdDuration::new(5, 0));
        } else {
          self.list.push(liftoff_text.clone());
          self.n_items += 1;
        }
      }
      if ui.input(|i| i.key_pressed(egui::Key::Space)) {
        if self.running {
          self.running = false;
          self.duration = rem_time;
          self.list.push(liftoff_text.clone());
          self.n_items += 1;
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

#[cfg(not(target_arch = "wasm32"))]
fn main()  -> eframe::Result<()> {
  // Log to stdout (if you run with `RUST_LOG=debug`).
  //tracing_subscriber::fmt::init();

  let options = eframe::NativeOptions {
    decorated: false,
    transparent: true,
    icon_data: Some(ui::load_icon()),
    min_window_size: Some(egui::vec2(300.0, 200.0)),
    initial_window_size: Some(egui::vec2(300.0, 200.0)),
    ..Default::default()
  };

  eframe::run_native(
    "Liftoff",
    options,
    //Box::new(|cc| Box::new(Liftoff::default(cc))),
    Box::new(|cc| Box::new(Liftoff::new(cc))),
  )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(Liftoff::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}