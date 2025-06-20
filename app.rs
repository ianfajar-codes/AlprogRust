use eframe::{egui, App, Frame};
use egui::{Color32, RichText};
use egui_plot::{Line, Plot, PlotPoints};
use mongodb::Client;
use tokio::runtime::Runtime;
use chrono::{DateTime as ChronoDateTime, TimeZone, Utc};
use bson::DateTime as BsonDateTime;

use crate::model::SensorData;
use crate::mongo::{fetch_data, get_client};

#[derive(PartialEq)]
enum ViewMode {
    Dashboard,
    Realtime,
    History,
}

pub struct MyApp {
    view: ViewMode,
    rt: Runtime,
    client: Client,
    data: Vec<SensorData>,
    last_update_time: f64,
}

impl MyApp {
    pub fn new() -> Self {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        let client = rt.block_on(get_client()).expect("Failed to connect MongoDB");

        Self {
            view: ViewMode::Dashboard,
            rt,
            client,
            data: vec![],
            last_update_time: 0.0,
        }
    }

    fn update_data(&mut self) {
        match self.rt.block_on(fetch_data(&self.client)) {
            Ok(result) => {
                println!("✅ Dapat {} data", result.len());
                self.data = result;
            }
            Err(e) => {
                eprintln!("❌ ERROR Fetch Mongo: {:?}", e);
            }
        }
    }

    fn draw_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("📡 SensorGas UI").heading().color(Color32::WHITE));
            ui.add_space(20.0);
        });

        ui.separator();

        ui.vertical(|ui| {
            if ui.selectable_label(self.view == ViewMode::Dashboard, "📊 Dashboard").clicked() {
                self.view = ViewMode::Dashboard;
            }
            if ui.selectable_label(self.view == ViewMode::Realtime, "📟 Realtime").clicked() {
                self.view = ViewMode::Realtime;
            }
            if ui.selectable_label(self.view == ViewMode::History, "📜 History").clicked() {
                self.view = ViewMode::History;
            }
            if ui.button("❌ Keluar").clicked() {
                std::process::exit(0);
            }
        });
    }

    fn convert_bson_datetime(dt: &BsonDateTime) -> ChronoDateTime<Utc> {
    Utc.timestamp_millis(dt.timestamp_millis())
}

    fn draw_dashboard(&self, ui: &mut egui::Ui) {
        ui.heading("📈 Grafik Sensor (10 Data Terbaru)");
        let recent_points: PlotPoints = self
            .data
            .iter()
            .rev()
            .take(10)
            .rev()
            .map(|d| [Self::convert_bson_datetime(&d.timestamp).timestamp() as f64, d.value])
            .collect();

        Plot::new("plot")
            .view_aspect(2.0)
            .allow_scroll(true)
            .allow_zoom(true)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(recent_points));
            });
    }

    fn draw_realtime(&self, ui: &mut egui::Ui) {
        ui.heading("📟 Nilai Sensor Terkini");
        if let Some(last) = self.data.last() {
            let ts = Self::convert_bson_datetime(&last.timestamp);
            ui.label(format!("Waktu: {}", ts));
            ui.add_space(10.0);
            ui.label(RichText::new(format!("{:.2} ppm", last.value)).size(64.0).strong());

            let status = if last.value >= 100.0 {
                ("🚨 Udara Kotor", Color32::RED)
            } else {
                ("✅ Udara Bersih", Color32::GREEN)
            };

            ui.label(RichText::new(status.0).color(status.1).size(24.0).strong());
        } else {
            ui.label("Belum ada data sensor.");
        }
    }

    fn draw_history(&self, ui: &mut egui::Ui) {
        ui.heading("📜 Riwayat Data Sensor");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for d in &self.data {
                let ts = Self::convert_bson_datetime(&d.timestamp);
                ui.horizontal(|ui| {
                    ui.label(format!("{}", ts));
                    ui.label(format!("{:.2} ppm", d.value));
                });
            }
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let time = ctx.input(|i| i.time);
        if time - self.last_update_time > 2.0 {
            self.update_data();
            self.last_update_time = time;
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Selamat Datang di Aplikasi Monitoring Sensor Gas!");
            });
        });

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(180.0)
            .frame(egui::Frame::none().fill(Color32::from_rgb(30, 180, 140)))
            .show(ctx, |ui| {
                self.draw_sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view {
                ViewMode::Dashboard => self.draw_dashboard(ui),
                ViewMode::Realtime => self.draw_realtime(ui),
                ViewMode::History => self.draw_history(ui),
            }
        });
    }
}
