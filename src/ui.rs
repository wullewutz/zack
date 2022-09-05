use egui::{
    plot::{Line, Plot, Value, Values},
    Context, Ui,
};
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;

pub struct App {
    pub channels: Vec<(VecDeque<Value>, String)>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
    x: f64,
    running: bool,
    windows: bool,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        receiver: Receiver<Vec<f64>>,
        buffer_length: Box<usize>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            channels: vec![],
            receiver,
            x: 0.0,
            buffer_length,
            running: true,
            windows: false,
        }
    }

    fn receive_data(&mut self) {
        for mut chunks in self.receiver.try_iter() {
            self.x += 1.0;
            while chunks.len() > self.channels.len() {
                self.channels.push((
                    VecDeque::default(),
                    format!("Channel {}", self.channels.len()),
                ));
                println!("Added channel nr. {}", self.channels.len());
            }
            while chunks.len() < self.channels.len() {
                chunks.push(0.0);
            }
            for (i, ch) in self.channels.iter_mut().enumerate() {
                if ch.0.len() > *self.buffer_length {
                    ch.0.pop_front();
                }

                ch.0.push_back(Value {
                    x: self.x,
                    y: chunks[i],
                });
            }
        }
    }

    fn keys_event_loop(&mut self, ui: &mut Ui) {
        if ui
            .input_mut()
            .consume_key(egui::Modifiers::NONE, egui::Key::Space)
        {
            self.running = !self.running;
        }
        if ui
            .input_mut()
            .consume_key(egui::Modifiers::NONE, egui::Key::W)
        {
            self.windows = !self.windows;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.keys_event_loop(ui);

            if self.running {
                self.receive_data();
            }

            if self.windows {
                for ch in &self.channels {
                    egui::Window::new(ch.1.to_owned()).show(ctx, |ui| {
                        let plot = Plot::new("measurements");
                        plot.show(ui, |plot_ui| {
                            plot_ui.line(
                                Line::new(Values::from_values_iter(ch.0.iter().copied()))
                                    .color(egui::Color32::GREEN),
                            );
                        });
                    });
                }
            } else {
                let plot = Plot::new("measurements");
                plot.show(ui, |plot_ui| {
                    for ch in &self.channels {
                        plot_ui.line(Line::new(Values::from_values_iter(ch.0.iter().copied())));
                    }
                });
            }
        });
        ctx.request_repaint();
    }
}
