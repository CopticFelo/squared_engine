use eframe::egui;
use std::default::Default;

struct Squared;

impl Squared {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for Squared {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| ui.label("Hello World"));
    }
}

fn main() {
    let opts = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Squared",
        opts,
        Box::new(|cc| Ok(Box::new(Squared::new(cc)))),
    );
}
