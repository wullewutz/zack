use clap::{crate_version, value_parser, Arg, Command};
use eframe::NativeOptions;
use std::sync::mpsc::channel;

mod input;
mod ui;

enum Source {
    StdIn,
    TcpStream(String),
}

struct Opts {
    buf_length: Box<usize>,
    source: Source,
}

fn main() -> eframe::Result<()> {
    let opts = parse_opts();
    let (sender, receiver) = channel();

    match opts.source {
        Source::TcpStream(tcp_socket) => input::tcp_reader(sender, tcp_socket),
        Source::StdIn => input::stdin_reader(sender),
    }

    eframe::run_native(
        "zack",
        NativeOptions::default(),
        Box::new(|cc| Box::new(ui::App::new(cc, receiver, opts.buf_length))),
    )
}

fn parse_opts() -> Opts {
    let matches = Command::new("Zack - plot CSV (-ish) streams in realtime")
        .about("Plot CSV (-ish) streams in realtime")
        .version(crate_version!())
        .arg(
            Arg::new("buf_length")
                .help(
                    "How many points of each channels should \
                     be displayed before dropping the oldest.
                     Has to be a power of 2",
                )
                .short('b')
                .long("buffer")
                .value_parser(value_parser!(u64).range(2..=(1 << 20)))
                .default_value("65536"),
        )
        .arg(
            Arg::new("host:port")
                .help("Tcp socket from where to read csv stream")
                .required(false)
                .short('t')
                .long("tcp"),
        )
        .get_matches();

    let buf_length = Box::new(
        *matches
            .get_one::<u64>("buf_length")
            .expect("Invalid buffer length provided") as usize,
    );

    let source = if let Some(tcp_socket) = matches.get_one::<String>("host:port") {
        Source::TcpStream(tcp_socket.to_string())
    } else {
        Source::StdIn
    };
    Opts { buf_length, source }
}
