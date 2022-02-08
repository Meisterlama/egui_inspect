use crate::InspectNumber;
use crate::InspectString;
use egui::{Color32, Ui};

macro_rules! impl_inspect_float {
    ($($t:ty),+) => {
        $(
            impl crate::InspectNumber for $t {
                fn inspect_with_slider(&mut self, label: &'static str, ui: &mut egui::Ui, min: f32, max: f32) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                    });
                }
                fn inspect_with_drag_value(&mut self, label: &'static str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.add(egui::DragValue::new(self));
                    });
                }
            }

            impl crate::EguiInspect for $t {
                fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.label(self.to_string());
                    });
                }
                fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
                    self.inspect_with_slider(label, ui, 0.0f32, 100.0f32);
                }
            }
        )*
    }
}

macro_rules! impl_inspect_int {
    ($($t:ty),+) => {
        $(
        impl crate::InspectNumber for $t {
            fn inspect_with_slider(&mut self, label: &'static str, ui: &mut egui::Ui, min: f32, max: f32) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                });
            }
            fn inspect_with_drag_value(&mut self, label: &'static str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.add(egui::DragValue::new(self));
                });
            }
        }

        impl crate::EguiInspect for $t {
            fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.label(self.to_string());
                });
            }
            fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
                self.inspect_with_slider(label, ui, 0.0, 100.0);
            }
        }
        )*
    }
}

impl_inspect_float!(f32, f64);

impl_inspect_int!(i8, u8);
impl_inspect_int!(i16, u16);
impl_inspect_int!(i32, u32);
impl_inspect_int!(i64, u64);
impl_inspect_int!(isize, usize);

impl crate::EguiInspect for &'static str {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(self.to_string());
        });
    }
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.colored_label(Color32::from_rgb(255, 0, 0), self.to_string())
                .on_hover_text("inspect_mut is not implemented for &'static str");
        });
    }
}

impl crate::EguiInspect for String {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(self);
        });
    }
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
        self.inspect_mut_singleline(label, ui);
    }
}

impl crate::InspectString for String {
    fn inspect_mut_multiline(&mut self, label: &'static str, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.text_edit_multiline(self);
        });
    }

    fn inspect_mut_singleline(&mut self, label: &'static str, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.text_edit_singleline(self);
        });
    }
}

impl crate::EguiInspect for bool {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
        ui.add_enabled(false, egui::Checkbox::new(&mut self.clone(), label));
    }
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
        ui.checkbox(self, label);
    }
}
