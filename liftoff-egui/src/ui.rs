
use eframe::egui;

pub fn setup_font(ctx: &egui::Context) {
  let mut style = (*ctx.style()).clone();
  style.text_styles = [
    (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
    //(egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
    //(egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
    //(egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
    //(egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Body, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Monospace, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Small, egui::FontId::new(9.0, egui::FontFamily::Proportional)),
  ]
  .into();
  ctx.set_style(style);
  }

pub(crate) fn load_icon() -> eframe::IconData {
  let (icon_rgba, icon_width, icon_height) = {
    let icon = include_bytes!("../../img/liftoff.png");
    let image = image::load_from_memory(icon)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };

  eframe::IconData {
    rgba: icon_rgba,
    width: icon_width,
    height: icon_height,
  }
}

pub fn window_frame(
  ctx: &egui::Context,
  frame: &mut eframe::Frame,
  title: &str,
  add_contents: impl FnOnce(&mut egui::Ui),
) {
  use egui::*;

  let panel_frame = egui::Frame {
    fill: ctx.style().visuals.window_fill(),
    rounding: 10.0.into(),
    stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
    outer_margin: 0.5.into(), // so the stroke is within the bounds
    ..Default::default()
  };

  CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    let app_rect = ui.max_rect();

    let title_bar_height = 32.0;
    let title_bar_rect = {
      let mut rect = app_rect;
      rect.max.y = rect.min.y + title_bar_height;
      rect
    };
    title_bar_ui(ui, frame, title_bar_rect, title);

    // Add the contents:
    let content_rect = {
      let mut rect = app_rect;
      rect.min.y = title_bar_rect.max.y;
      rect
    }
    .shrink(4.0);
    let mut content_ui = ui.child_ui(content_rect, *ui.layout());
    add_contents(&mut content_ui);
  });
}

pub fn title_bar_ui(
  ui: &mut egui::Ui,
  frame: &mut eframe::Frame,
  title_bar_rect: eframe::epaint::Rect,
  title: &str,
) {
  use egui::*;

  let painter = ui.painter();

  let title_bar_response = ui.interact(title_bar_rect, Id::new("Liftoff"), Sense::click());

  // Paint the title:
  painter.text(
    title_bar_rect.center(),
    Align2::CENTER_CENTER,
    title,
    FontId::proportional(20.0),
    ui.style().visuals.text_color(),
  );

  // Paint the line under the title:
  painter.line_segment(
    [
      title_bar_rect.left_bottom() + vec2(1.0, 0.0),
      title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
    ],
    ui.visuals().widgets.noninteractive.bg_stroke,
  );

  // Interact with the title bar (drag to move window):
  if title_bar_response.double_clicked() {
      frame.set_maximized(!frame.info().window_info.maximized);
  } else if title_bar_response.is_pointer_button_down_on() {
      frame.drag_window();
  }

  ui.allocate_ui_at_rect(title_bar_rect, |ui| {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
      ui.spacing_mut().item_spacing.x = 0.0;
      ui.visuals_mut().button_frame = false;
      ui.add_space(8.0);
      use egui::special_emojis::{GITHUB};

      let link = TBLink {
        display: true,
        label: String::from(format!("{}", GITHUB)),
        link: String::from("https://github.com/tschinz/liftoff"),
      };
      window_buttons(ui, frame, true, false, true, true, link);
    });
  });
}
pub struct TBLink {
  display: bool,
  label: String,
  link: String,
}
/// Show some close/maximize/minimize buttons for the native window.
pub fn window_buttons(ui: &mut egui::Ui, frame: &mut eframe::Frame, close: bool, maximize: bool, minimize:bool, theme:bool, link: TBLink) {
  use egui::{Button, RichText};

  let button_height = 12.0;

  if close {
    let close_response = ui
      .add(Button::new(RichText::new("❌").size(button_height)));
      //.on_hover_text("Close the window");
    if close_response.clicked() {
      frame.close();
    }
  }

  if maximize {
    if frame.info().window_info.maximized {
      let maximized_response = ui
        .add(Button::new(RichText::new("🗗").size(button_height)));
        //.on_hover_text("Restore window");
      if maximized_response.clicked() {
        frame.set_maximized(false);
      }
    } else {
      let maximized_response = ui
        .add(Button::new(RichText::new("🗗").size(button_height)));
        //.on_hover_text("Maximize window");
      if maximized_response.clicked() {
        frame.set_maximized(true);
      }
    }
  }

  if minimize {
    let minimized_response = ui
      .add(Button::new(RichText::new("_").size(button_height)));
      //.on_hover_text("Minimize the window");
    if minimized_response.clicked() {
      frame.set_minimized(true);
    }
  }

  if theme {
    egui::widgets::global_dark_light_mode_switch(ui);
  }
  if link.display {
    ui.hyperlink_to(
      link.label,
      link.link,
    ).on_hover_text("");
  }

}