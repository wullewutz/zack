use std::io::BufRead;
use std::sync::mpsc::channel;
use std::thread;

use eframe::NativeOptions;

mod ui;

use clap::{crate_version, Arg, Command};

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
        Box::new(|cc| Box::new(ui::App::new(cc, receiver, buf_length))),
    );
}
