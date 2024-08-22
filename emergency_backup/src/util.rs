use eframe::egui;
use eframe::egui::CentralPanel;
use std::time::Instant;
use win_beep;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

pub struct MyApp {
    text: String,
    start_time: Instant,
}

impl MyApp {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            start_time: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let elapsed = self.start_time.elapsed().as_secs() as i32;
        let remaining_time = 10_i32.saturating_sub(elapsed);
        let display_text = format!("{} ({})", self.text, remaining_time);

        CentralPanel::default().show(ctx, |ui| {
            ui.label(display_text+"\n");
            if ui.button("  OK  ").clicked() {
                std::process::exit(0);
            }
        });

        // Check if the timer has expired
        if remaining_time == 0 {
            std::process::exit(0);
        }

        ctx.request_repaint();
    }
}

pub fn popup(title: &str, text: &str, _sound: &str) {
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(text)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

pub fn hide_console() {
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_null() {
            ShowWindow(console_window, SW_HIDE);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <title> <text> <sound>", args[0]);
        eprintln!("Sound options: 'error' , 'info', 'none' ");
        std::process::exit(1);
    }

    let title = &args[1];
    let text = &args[2];
    match args[3].as_str() {
        "error" => win_beep::beep_with_hz_and_millis(200, 500),
        "info" => win_beep::beep_with_hz_and_millis(400, 300),
        _ => {
            eprintln!("Invalid sound option.");
        }
    };

    let app = MyApp::new(text);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([280 as f32, 100 as f32]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        title, 
        options, 
        Box::new(|_cc| Ok(Box::new(app))));
}

