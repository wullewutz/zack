use egui::{
    plot::{Line, Plot, Value, Values},
    Context,
};
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;

pub struct App {
    pub channels: Vec<VecDeque<Value>>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
    x: f64,
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
        }
    }

    fn receive_data(&mut self) {
        for mut chunks in self.receiver.try_iter() {
            self.x += 1.0;
            while chunks.len() > self.channels.len() {
                self.channels.push(VecDeque::default());
                println!("Added channel nr. {}", self.channels.len());
            }
            while chunks.len() < self.channels.len() {
                chunks.push(0.0);
            }
            for (i, ch) in self.channels.iter_mut().enumerate() {
                if ch.len() > *self.buffer_length {
                    ch.pop_front();
                }

                ch.push_back(Value {
                    x: self.x,
                    y: chunks[i],
                });
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.receive_data();
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                for ch in &self.channels {
                    plot_ui.line(Line::new(Values::from_values_iter(ch.iter().copied())));
                }
            });
        });
        ctx.request_repaint();
    }
}
