use egui::{
    plot::{Line, Plot, PlotPoints},
    Context, Ui,
};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};
use std::sync::mpsc::Receiver;

pub struct App {
    pub channels: Vec<(AllocRingBuffer<f64>, String)>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
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
            buffer_length,
            running: true,
            windows: false,
        }
    }

    fn receive_data(&mut self) {
        for mut chunks in self.receiver.try_iter() {
            while chunks.len() > self.channels.len() {
                self.channels.push((
                    AllocRingBuffer::with_capacity(*self.buffer_length),
                    format!("Channel {}", self.channels.len()),
                ));
                println!("Added channel nr. {}", self.channels.len());
            }
            while chunks.len() < self.channels.len() {
                chunks.push(0.0);
            }
            for (i, ch) in self.channels.iter_mut().enumerate() {
                ch.0.push(chunks[i]);
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
                                Line::new(PlotPoints::from_ys_f64(&ch.0.to_vec()))
                                    .color(egui::Color32::GREEN),
                            );
                        });
                    });
                }
            } else {
                let plot = Plot::new("measurements");
                plot.show(ui, |plot_ui| {
                    for ch in &self.channels {
                        plot_ui.line(Line::new(PlotPoints::from_ys_f64(&ch.0.to_vec())));
                    }
                });
            }
        });
        ctx.request_repaint();
    }
}
