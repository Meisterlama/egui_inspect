#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_inspect::EguiInspect;

use eframe::egui;

#[derive(EguiInspect)]
struct MyApp {
    #[inspect(no_edit)]
    string: String,
    #[inspect(multiline)]
    code: String,
    #[inspect(min = 12.0, max = 53.0)]
    unsigned32: u32,
    #[inspect(hide)]
    _skipped: bool,
    #[inspect(no_edit)]
    strings: Vec<String>,
    #[inspect(no_edit)]
    raw_string: &'static str,
    #[inspect(slider, min = -43.0, max = 125.0)]
    float64: f64,
    #[inspect(name = "A proper field name")]
    ugly_internal_field_name: u16,
    #[inspect(name = "A tuple struct")]
    ugly_internal_field_name_2: Salut,
    #[inspect(name = "A struct with three floats")]
    vector_struct: Vector,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            string: "I am a single line string".to_owned(),
            code: "Hello\nI\nam\na\nmultiline\nstring".to_owned(),
            _skipped: true,
            unsigned32: 42,
            strings: vec!{"Bonjour".to_string(),
                          "Voici une liste de string".to_string(),
                          "Avec plusieurs strings".to_string()},
            raw_string: "YetAnotherString",
            float64: 6.0,
            ugly_internal_field_name: 16,
            ugly_internal_field_name_2: Salut(50, 123.45),
            vector_struct: Vector { x: 10.0, y: 20.0, z: 30.0 },
        }
    }
}

#[derive(EguiInspect)]
struct Salut(i32, f32);

#[derive(EguiInspect)]
struct Vector { 
    #[inspect(name = "X axis")]
    x: f32, 
    #[inspect(name = "Y axis")]
    y: f32, 
    #[inspect(name = "Z axis")]
    z: f32 
}


fn custom_bool_inspect(boolean: &mut bool, label: &'static str, ui: &mut egui::Ui) {
    ui.label("C'EST LA GIGA FONCTION CUSTOM WÉ");
    boolean.inspect(label, ui);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.inspect_mut("Test App", ui);
            // self.inspect("Test App", ui);

            let salut = Salut(1, 2.0);
            salut.inspect("label for tuple struct", ui);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(MyApp::default())));
}
