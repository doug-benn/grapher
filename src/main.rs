#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod monitor;

use std::{io::BufRead, sync::Arc, thread};

use eframe::egui;
use egui::{mutex::Mutex, Vec2, Vec2b};
use egui_plot::{Line, Plot, PlotBounds};
use monitor::Monitor;

struct App {
    monitor: Arc<Mutex<Monitor>>,
    first_draw: bool,
}
impl App {
    fn new(window_size: f64) -> Self {
        Self {
            monitor: Arc::new(Mutex::new(Monitor::new(window_size))),
            first_draw: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let max_point = self.monitor.lock().max_value();
            let plot = Plot::new("measurements"); //.allow_drag(Vec2b::new(true, false));
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.monitor.lock().get_values()));
                if self.first_draw {
                    self.first_draw = false;
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [100.0, 0.0]));
                    plot_ui.set_auto_bounds(Vec2b::new(false, true));
                } else {
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                        [max_point - 100.0, 0.0],
                        [max_point, 0.0],
                    ));
                    plot_ui.set_auto_bounds(Vec2b::new(false, true));
                }

                //plot_ui.plot_bounds.extend_with_x(self.monitor.lock().max_value())
                //plot_ui.plot_bounds().extend_with_x(self.monitor.lock().max_value())
                //plot_ui.plot_bounds().extend_with_x(self.monitor.lock().max_value());
                //plot_ui.translate_bounds(Vec2 { x: 2.0, y: 0.0 });
                //println!("Max Bound {:?} Max X {:?}", plot_ui.plot_bounds().width(), self.monitor.lock().max_value());
                //plot_ui.translate_bounds(Vec2 { x: 1.0, y: 0.0 });
                //plot_ui.plot_bounds().translate_x(2.0)

                //Use the max to create a windowing affect - max - window size, translate that difference
            })
        });
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = App::new(4000.0);

    let io_monitor = app.monitor.clone();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(stdin_line) => io_monitor.lock().append_str(&stdin_line.as_str()),
                Err(_) => return,
            }
        }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native("Grahper", options, Box::new(|_| Box::new(app)))
}
