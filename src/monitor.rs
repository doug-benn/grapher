use std::collections::VecDeque;

use egui_plot::{PlotPoint, PlotPoints};

pub struct Monitor {
  pub values: VecDeque<PlotPoint>,
  pub window_size: f64
}

impl Monitor {
  pub fn new(window_size: f64) -> Self {
    Self {
      values: VecDeque::default(),
      window_size,
    }
  }

  pub fn max_value(&mut self) {
    let max = self.values.iter().max();
  }

  pub fn append_value(&mut self, plot_point: PlotPoint){
    self.values.push_back(plot_point);
  }

  pub fn get_values(&self) -> PlotPoints {
    PlotPoints::Owned(Vec::from_iter(self.values.iter().copied())
    )
 }

  pub fn append_str(&mut self, input_string: &str) {
    let parts = input_string.split(' ').collect::<Vec<&str>>();

    if parts.len() != 2 {
      return
    }

    let x = match parts.first().unwrap().parse::<f64>() {
      Ok(value) => value,
      Err(_) => return,
    };
    let y = match parts.last().unwrap().parse::<f64>() {
      Ok(value) => value,
      Err(_) => return,
    };

    self.append_value(PlotPoint { x: x, y: y })
  }
}



