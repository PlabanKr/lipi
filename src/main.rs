use eframe;
mod app;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Lipi",
        native_options,
        Box::new(|_cc| Ok(Box::<app::LipiEditor>::default())),
    );
}
