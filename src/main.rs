#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod views;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 820.0])
            .with_min_inner_size([850.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "IAM Lab",
        options,
        Box::new(|cc| Ok(Box::new(app::Lab::new(cc)))),
    )
}
