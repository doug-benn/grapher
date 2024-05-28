#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod monitor;

use std::{net::UdpSocket, sync::Arc, thread};

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
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.monitor.lock().get_values()));
                if self.first_draw {
                    // Sets the plot size on first draw
                    self.first_draw = false;
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [100.0, 0.0]));
                    plot_ui.set_auto_bounds(Vec2b::new(false, true));
                } else if max_point - plot_ui.transform().bounds().max()[0] > 0.0 {
                    //Translates the plot to create a windowing effect: max plot point - max plot size, translate the difference
                    let plot_diff = max_point - plot_ui.transform().bounds().max()[0];
                    plot_ui.translate_bounds(Vec2 {
                        x: plot_diff as f32,
                        y: 0.0,
                    });
                    plot_ui.set_auto_bounds(Vec2b::new(false, true));
                };

                //println!("{:?}", max_point - plot_ui.transform().bounds().max()[0]);
            })
        });
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = App::new(4000.0);

    let io_monitor = app.monitor.clone();

    thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:6969").expect("Failed to bind");
        loop {
            let mut buf = [0; 1024];
            match socket.recv_from(&mut buf) {
                Ok((nbytes, remote_addr)) => {
                    //let message = std::str::from_utf8(buf.as_slice()).expect("utf-8 convert failed");
                    let message = std::str::from_utf8(&buf[..nbytes]).unwrap();
                    println!("{} from {}", message, remote_addr);
                    io_monitor.lock().append_str(&message)
                }
                Err(e) => {
                    println!("Something bad happened: {}", e);
                }
            }
        }

        // let stdin = std::io::stdin();
        // for line in stdin.lock().lines() {
        //     match line {
        //         Ok(stdin_line) => io_monitor.lock().append_str(&stdin_line.as_str()),
        //         Err(_) => return,
        //     }
        // }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native("Grahper", options, Box::new(|_| Box::new(app)))
}
