use std::collections::VecDeque;

use egui_plot::{PlotPoint, PlotPoints};

pub struct Monitor {
    pub values: VecDeque<PlotPoint>,
    pub window_size: f64,
}

impl Monitor {
    pub fn new(window_size: f64) -> Self {
        Self {
            values: VecDeque::default(),
            window_size,
        }
    }

    pub fn max_value(&mut self) -> f64 {
        match self.values.len() {
            0 => 0.0,
            n => self.values[n - 1].clone().x,
        }
        //self.values.iter().last().unwrap().x
    }

    pub fn append_value(&mut self, plot_point: PlotPoint) {
        self.values.push_back(plot_point);
    }

    pub fn get_values(&self) -> PlotPoints {
        PlotPoints::Owned(Vec::from_iter(self.values.iter().copied()))
    }

    pub fn append_str(&mut self, input_string: &str) {
        println!("Input string: {}", input_string);
        let parts = input_string.split(' ').collect::<Vec<&str>>();
        println!("Input String: {:?}", parts);

        if parts.len() != 2 {
            println!("Nope");
            return;
        }

        let x = match parts.first().unwrap().parse::<f64>() {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        let y = match parts.last().unwrap().parse::<f64>() {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        self.append_value(PlotPoint { x: x, y: y })
    }
}

// pub struct Testing {
//     pub test_monitor: PlotPoints,
// }

// impl Testing {
//     pub fn new() -> Self {
//         Self {
//             test_monitor: PlotPoints::default(),
//         }
//     }
// }
