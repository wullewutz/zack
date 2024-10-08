use egui::{Color32, Context, RichText, ViewportCommand};
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::sync::mpsc::Receiver;

pub struct App {
    pub channels: Vec<(AllocRingBuffer<f64>, String)>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
    running: bool,
    single_plot: bool,
    show_keymap: bool,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        receiver: Receiver<Vec<f64>>,
        buffer_length: Box<usize>,
        names: Vec<String>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut channels: Vec<(AllocRingBuffer<f64>, String)> = vec![];
        for name in names {
            channels.push((AllocRingBuffer::new(*buffer_length), name.clone()));
            println!("Added channel \"{}\"", name);
        }
        Self {
            channels,
            receiver,
            buffer_length,
            running: true,
            single_plot: true,
            show_keymap: false,
        }
    }

    fn receive_data(&mut self) {
        for mut chunks in self.receiver.try_iter() {
            if chunks.len() != self.channels.len() {
                // More chunks found than in previous rounds:
                // Add a new channel and fill it with as many zeros as the first channel
                while chunks.len() > self.channels.len() {
                    self.channels.push((
                        AllocRingBuffer::new(*self.buffer_length),
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
            }

            // Normal Case - as many chunks as there are channels:
            // Push new chunks to the channels
            for (i, ch) in self.channels.iter_mut().enumerate() {
                ch.0.push(chunks[i]);
            }
        }
    }

    fn keys_event_loop(&mut self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.running = !self.running;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::C)) {
            self.clean();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::W)) {
            self.single_plot = !self.single_plot;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.show_keymap = !self.show_keymap;
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

    fn clean(&mut self) {
        for ch in &mut self.channels {
            ch.0.clear();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.radio_value(&mut self.single_plot, true, "Single Plot");
                    ui.radio_value(&mut self.single_plot, false, "Stacked");
                    egui::widgets::global_theme_preference_buttons(ui);
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("Keymap").clicked() {
                        self.show_keymap = !self.show_keymap;
                    }
                });

                if self.show_keymap {
                    egui::Window::new("Keymap")
                        .default_pos([300.0, 300.0])
                        .show(ctx, |ui| {
                            ui.label("space:\tPause/Resume");
                            ui.label("c:\tClear current plots");
                            ui.label("w:\tToggle single/multi plot view");
                            ui.label("h:\tShow this help screen");
                            ui.label("q:\tQuit zack");
                        });
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if self.running {
                    ui.label(RichText::new("running").color(Color32::GREEN));
                } else {
                    ui.label(RichText::new("paused").color(Color32::RED));
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.keys_event_loop(ctx);

            if self.running {
                self.receive_data();
            }

            let lines = self.lines();

            if self.single_plot {
                Plot::new("all_plots")
                    .legend(Legend::default().position(Corner::LeftTop))
                    .show(ui, |plot_ui| {
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
                            .link_axis(link_group_id, true, false)
                            .link_cursor(link_group_id, true, false)
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
