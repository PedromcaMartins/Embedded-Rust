use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use tokio::{sync::mpsc, time::Instant};

type Telemetry = [f64; 2];

pub struct MyApp {
    rx: mpsc::Receiver<Telemetry>,
    data: Vec<Telemetry>
}

impl MyApp {
    pub fn new(rx: mpsc::Receiver<Telemetry>) -> Self {
        Self { rx, data: Vec::new() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Receive telemetry updates
        while let Ok(point) = self.rx.try_recv() {
            self.data.push(point);
            if self.data.len() > 100 {
                self.data.remove(0); // Keep last 500 points for performance
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ground Station");

            Plot::new("Telemetry Plot")
                .show(ui, |plot_ui| {
                    let line = Line::new(PlotPoints::from_iter(self.data.iter().copied()))
                        .color(egui::Color32::LIGHT_BLUE)
                        .name("Telemetry");
                    plot_ui.line(line);
                });
        });

        ctx.request_repaint(); // Keep updating UI
    }
}

pub async fn simulated_telem(tx: mpsc::Sender<Telemetry>) {
    let start_time = Instant::now();
    let mut value;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
        let time = start_time.elapsed().as_secs_f64();
        value = (time * 2.0).sin(); // Simulated telemetry data (sine wave)
        tx.send([time, value]).await.ok();
    }
}