pub use egui_inspect_derive::*;

pub trait EguiInspect {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui);
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui);
}

pub trait InspectNumber {
    fn inspect_with_slider(&mut self, label: &'static str, ui: &mut egui::Ui, min: f32, max: f32);
    fn inspect_with_drag_value(&mut self, label: &'static str, ui: &mut egui::Ui);
}

pub trait InspectString {
    fn inspect_mut_multiline(&mut self, label: &'static str, ui: &mut egui::Ui);
    fn inspect_mut_singleline(&mut self, label: &'static str, ui: &mut egui::Ui);
}

pub mod base_type_inspect;
