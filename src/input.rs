use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;

pub fn stdin_reader(sender: Sender<Vec<f64>>) {
    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => sender
                    .send(parse_line(line))
                    .expect("Error: Could not send new data"),
                _ => return,
            }
        }
    });
}

pub fn tcp_reader(sender: Sender<Vec<f64>>, tcp_socket: String) {
    let stream = TcpStream::connect(tcp_socket).expect("could not connect to tcp stream");
    let reader = BufReader::new(stream);
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(line) => sender
                    .send(parse_line(line))
                    .expect("Error: Could not send new data"),
                _ => return,
            }
        }
    });
}

fn parse_line(line: String) -> Vec<f64> {
    // trim unwanted leading and trailing characters and split at one
    // of the possible seperators
    let sep = |c| c == ',' || c == ';' || c == ' ' || c == '\t';
    let line = line.trim_start_matches(sep);
    let line = line.trim_end_matches(sep);
    line.split(sep)
        .map(|y| y.parse::<f64>().unwrap_or(0.0))
        .collect()
}
