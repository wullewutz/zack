use std::io::BufRead;
use std::sync::mpsc::Sender;
use std::thread;

pub fn stdin_reader(sender: Sender<Vec<f64>>) {
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
                    let chunks: Vec<f64> = s
                        .split(sep)
                        .map(|y| y.parse::<f64>().unwrap_or(0.0))
                        .collect();
                    sender.send(chunks).expect("Error: Could not send new data");
                }
                _ => return,
            }
        }
    });
}
