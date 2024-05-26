mod monitor;

use std::{io::BufRead, sync::Arc, thread};

use eframe::egui;
use egui::{mutex::Mutex, Vec2, Vec2b};
use egui_plot::{Line, Plot, PlotBounds};
use monitor::Monitor;


struct App {
    monitor: Arc<Mutex<Monitor>>,
    first_draw: bool
}
impl App {
    fn new(window_size: f64) -> Self{
        Self{
            monitor: Arc::new(Mutex::new(Monitor::new(window_size))),
            first_draw: true
        }

    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            let plot = Plot::new("measurements").allow_drag(Vec2b::new(true, false));
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.monitor.lock().get_values()
                ));
                if self.first_draw {
                    self.first_draw = false;
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                            [0.0, 0.0],
                            [1000.0, 20.0],
                        ));}
                plot_ui.set_auto_bounds(Vec2b::new(false, true));
                plot_ui.translate_bounds(Vec2::new(1.0, 0.0))
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
        for line in stdin.lock().lines(){
            match line {
                Ok(stdin_line) => {
                    io_monitor.lock().append_str(&stdin_line.as_str())
                }
                Err(_) => return
            }
        }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Grahper", 
        options, 
        Box::new(|_| Box::new(app))
    )
}
