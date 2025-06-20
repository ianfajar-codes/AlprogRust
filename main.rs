mod mongo;
mod model;
mod app;

use app::MyApp;
use eframe::{NativeOptions};
use egui::ViewportBuilder;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(
        "📈 Dashboard Sensor Gas",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}
