use eframe::egui::{self, FontId};

pub struct LipiEditor {
  pub current_file_name: String,
  pub current_file_extension: String,
  pub current_file_content: String,
}

impl Default for LipiEditor {
  fn default() -> Self {
    Self {
      current_file_name: String::from("untitled"),
      current_file_extension: String::from("txt"),
      current_file_content: String::new(),
    }
  }
}

impl eframe::App for LipiEditor {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    let Self {
      current_file_name,
      current_file_extension,
      current_file_content,
    } = self;

    egui::TopBottomPanel::bottom("information_bar").show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label(format!("File: {}.{}", current_file_name, current_file_extension));
        ui.separator();
        ui.label(format!("Last Line: {}", current_file_content.lines().last().unwrap_or("")));
      });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::vertical().show(ui, |ui| {
        let mut line_number = 0;
        line_number += current_file_content.lines().count();
        
        ui.horizontal(|ui| {
          ui.vertical(|ui| {
            for i in 1..=line_number {
                ui.label(egui::RichText::new(format!("{} ", i)).font(FontId::monospace(16.0)));
            }
          });
          ui.add(
            egui::TextEdit::multiline(current_file_content)
              .code_editor()
              .font(FontId::monospace(16.0))
              .desired_width(f32::INFINITY)
              .desired_rows(40),
          );
        });
      });
    });
  }
}