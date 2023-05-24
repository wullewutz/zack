use egui::{
    plot::{Corner, Legend, Line, Plot, PlotPoints},
    Context,
};
use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::sync::mpsc::Receiver;

pub struct App {
    pub channels: Vec<(AllocRingBuffer<f64>, String)>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
    running: bool,
    single_plot: bool,
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
            single_plot: true,
        }
    }

    fn receive_data(&mut self) {
        for mut chunks in self.receiver.try_iter() {
            // More chunks found than in previous rounds:
            // Add a new channel and fill it with as many zeros as the first channel
            while chunks.len() > self.channels.len() {
                self.channels.push((
                    AllocRingBuffer::with_capacity(*self.buffer_length),
                    format!("Channel {}", self.channels.len()),
                ));
                if !self.channels.is_empty() {
                    for _i in 0..self.channels.first().unwrap().0.len() {
                        self.channels.last_mut().unwrap().0.push(0.0);
                    }
                }
                println!("Added channel nr. {}", self.channels.len());
            }

            // Less chunks found than in previous rounds:
            // Missing chunks will be assumed to be zeros
            while chunks.len() < self.channels.len() {
                chunks.push(0.0);
            }

            // Finally, push new chunks to the channels
            for (i, ch) in self.channels.iter_mut().enumerate() {
                ch.0.push(chunks[i]);
            }
        }
    }

    fn keys_event_loop(&mut self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.running = !self.running;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::W)) {
            self.single_plot = !self.single_plot;
        }
    }

    fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        for ch in &self.channels {
            lines.push(
                Line::new(PlotPoints::from_ys_f64(&ch.0.to_vec()))
                    .color(egui::Color32::TRANSPARENT)
                    .name(ch.1.to_owned()),
            );
        }
        lines
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.keys_event_loop(ctx);

            if self.running {
                self.receive_data();
            }

            let lines = self.lines();

            if self.single_plot {
                let plot =
                    Plot::new("all_plot").legend(Legend::default().position(Corner::LeftTop));
                plot.show(ui, |plot_ui| {
                    for l in lines {
                        plot_ui.line(l);
                    }
                });
            } else {
                let plot_height = ui.available_height() / self.channels.len() as f32;
                egui::ScrollArea::both().show(ui, |ui| {
                    let link_group_id = ui.id().with("linked_group");
                    for (i, l) in lines.into_iter().enumerate() {
                        Plot::new(format!("plot_{i}"))
                            .legend(Legend::default().position(Corner::LeftTop))
                            .height(plot_height)
                            .min_size(egui::Vec2::new(300.0, 200.0))
                            .allow_scroll(false)
                            .link_axis(link_group_id, true, true)
                            .link_cursor(link_group_id, true, true)
                            .show(ui, |plot_ui| {
                                plot_ui.line(l.color(egui::Color32::GREEN));
                            });
                    }
                });
            }
        });
        ctx.request_repaint();
    }
}
