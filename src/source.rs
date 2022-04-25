use egui::plot::{Value, Values};
use std::collections::VecDeque;

pub struct Channels {
    pub values: Vec<VecDeque<Value>>,
    x: f64,
}

impl Channels {
    pub fn new() -> Self {
        Self {
            values: vec![],
            x: 0.0,
        }
    }

    pub fn get_channels(&mut self) -> Vec<Values> {
        let mut v = Vec::new();
        for c in &self.values {
            v.push(Values::from_values_iter(c.iter().copied()));
        }
        v
    }

    pub fn parse_line(&mut self, s: &str) {
        // trim unwanted leading and trailing characters and split at one
        // of the possible seperators
        let sep = |c| c == ',' || c == ';' || c == ' ' || c == '\t';
        let s = s.trim_start_matches(sep);
        let s = s.trim_end_matches(sep);
        let chans = s.split(sep).collect::<Vec<&str>>();

        for (i, val) in chans.iter().enumerate() {
            let y = val.parse::<f64>().unwrap_or(0.0);

            // Add more channels if required
            while chans.len() > self.values.len() {
                self.values.push(VecDeque::default());
                println!("Added channel nr. {}", self.values.len());
            }
            self.values[i].push_back(Value { x: self.x, y });
        }
        self.x += 1.0;
    }
}
