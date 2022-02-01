#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_inspect::EguiInspect;

use eframe::{egui, epi};

#[derive(EguiInspect)]
struct MyApp {
    #[inspect(multiline=false)]
    string: String,
    #[inspect(multiline=true)]
    code: String,
    #[inspect(min = 12.0, max = 53.0)]
    unsigned32: u32,
    boolean: bool,
    raw_string: &'static str,
    #[inspect(slider = false)]
    float32: f32,
    #[inspect(slider = true, min = -43.0, max = 125.0)]
    float64: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            string: "I am a single line string".to_owned(),
            code: "Hello\nI\nam\na\nmultiline\nstring".to_owned(),
            unsigned32: 42,
            boolean: false,
            raw_string: "YetAnotherString",
            float32: 12.0,
            float64: 6.0
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.inspect_mut("Test App", ui);
            // self.inspect("Test App", ui);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "My egui App"
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}
