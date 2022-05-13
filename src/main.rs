use clap::{crate_version, Arg, Command};
use eframe::NativeOptions;
use std::sync::mpsc::channel;

mod input;
mod ui;

struct Opts {
    buf_length: Box<usize>,
}

fn main() {
    let opts = parse_opts();
    let (sender, receiver) = channel();
    input::stdin_reader(sender);

    eframe::run_native(
        "zack",
        NativeOptions::default(),
        Box::new(|cc| Box::new(ui::App::new(cc, receiver, opts.buf_length))),
    );
}

fn parse_opts() -> Opts {
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
    Opts { buf_length }
}
