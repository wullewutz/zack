mod source;

use source::Channels;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::{epi, NativeOptions};
use egui::{
    plot::{Line, Plot},
    Context,
};

struct App {
    chans: Arc<Mutex<Channels>>,
}

impl App {
    fn new() -> Self {
        Self {
            chans: Arc::new(Mutex::new(Channels::new())),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &Context, _: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                for values in self.chans.lock().unwrap().get_channels() {
                    plot_ui.line(Line::new(values));
                }
            });
        });
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "zack"
    }
}

fn main() {
    let app = App::new();

    let chans = app.chans.clone();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(s) => chans.lock().unwrap().parse_line(&s),
                _ => return,
            }
        }
    });

    let native_options = NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
