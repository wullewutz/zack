use std::collections::VecDeque;
use std::io::BufRead;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use eframe::NativeOptions;
use egui::{
    plot::{Line, Plot, Value, Values},
    Context,
};

use clap::{crate_version, Arg, Command};

struct ZackApp {
    pub channels: Vec<VecDeque<Value>>,
    receiver: Receiver<Vec<f64>>,
    buffer_length: Box<usize>,
    x: f64,
}

impl ZackApp {
    fn new(
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
}

impl eframe::App for ZackApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                for chunks in self.receiver.try_iter() {
                    self.x += 1.0;
                    while chunks.len() > self.channels.len() {
                        self.channels.push(VecDeque::default());
                        println!("Added channel nr. {}", self.channels.len());
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
                for ch in &self.channels {
                    plot_ui.line(Line::new(Values::from_values_iter(ch.iter().copied())));
                }
            });
        });
        ctx.request_repaint();
    }
}

fn main() {
    let matches = Command::new("Zack - plot CSV (-ish) streams in realtime")
        .about("Plot CSV (-ish) streams in realtime")
        .version(crate_version!())
        .arg(
            Arg::new("buf_length")
                .help(
                    "How many points of each channels should \
                     be displayed before dropping the oldest",
                )
                .short('b')
                .long("buffer")
                .default_value("10000"),
        )
        .get_matches();

    let buf_length = Box::new(
        matches
            .value_of("buf_length")
            .expect("Invalid buffer length provided")
            .parse::<usize>()
            .expect("Buffer length needs to be a positiv integer"),
    );

    let (sender, receiver) = channel();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(s) => {
                    // trim unwanted leading and trailing characters and split at one
                    // of the possible seperators
                    let sep = |c| c == ',' || c == ';' || c == ' ' || c == '\t';
                    let s = s.trim_start_matches(sep);
                    let s = s.trim_end_matches(sep);
                    let str_chunks = s.split(sep).collect::<Vec<&str>>();
                    let chunks = str_chunks
                        .iter()
                        .map(|y| y.parse::<f64>().unwrap_or(0.0))
                        .collect();
                    sender.send(chunks).unwrap();
                }
                _ => return,
            }
        }
    });

    eframe::run_native(
        "zack",
        NativeOptions::default(),
        Box::new(|cc| Box::new(ZackApp::new(cc, receiver, buf_length))),
    );
}
