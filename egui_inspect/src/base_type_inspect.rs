use std::ops::Add;
use crate::InspectNumber;
use crate::InspectString;
use egui::{Color32, Ui};

macro_rules! impl_inspect_float {
    ($($t:ty),+) => {
        $(
            impl crate::InspectNumber for $t {
                fn inspect_with_slider(&mut self, label: &str, ui: &mut egui::Ui, min: f32, max: f32) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                    });
                }
                fn inspect_with_drag_value(&mut self, label: &str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.add(egui::DragValue::new(self));
                    });
                }
            }

            impl crate::EguiInspect for $t {
                fn inspect(&self, label: &str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label.to_owned() + ":");
                        ui.label(self.to_string());
                    });
                }
                fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
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
            fn inspect_with_slider(&mut self, label: &str, ui: &mut egui::Ui, min: f32, max: f32) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                });
            }
            fn inspect_with_drag_value(&mut self, label: &str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.add(egui::DragValue::new(self));
                });
            }
        }

        impl crate::EguiInspect for $t {
            fn inspect(&self, label: &str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label.to_owned() + ":");
                    ui.label(self.to_string());
                });
            }
            fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
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
    fn inspect(&self, label: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(self.to_string());
        });
    }
    fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.colored_label(Color32::from_rgb(255, 0, 0), self.to_string())
                .on_hover_text("inspect_mut is not implemented for &'static str");
        });
    }
}

impl crate::EguiInspect for String {
    fn inspect(&self, label: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(self);
        });
    }
    fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
        self.inspect_mut_singleline(label, ui);
    }
}

impl crate::InspectString for String {
    fn inspect_mut_multiline(&mut self, label: &str, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.text_edit_multiline(self);
        });
    }

    fn inspect_mut_singleline(&mut self, label: &str, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.text_edit_singleline(self);
        });
    }
}

impl crate::EguiInspect for bool {
    fn inspect(&self, label: &str, ui: &mut egui::Ui) {
        ui.add_enabled(false, egui::Checkbox::new(&mut self.clone(), label));
    }
    fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
        ui.checkbox(self, label);
    }
}

impl<T: crate::EguiInspect, const N: usize> crate::EguiInspect for [T; N] {
    fn inspect(&self, label: &str, ui: &mut Ui) {
        egui::CollapsingHeader::new(label.to_string().add(format!("[{}]", N).as_str())).show(ui, |ui| {
            for (i, item) in self.iter().enumerate() {
                item.inspect("item", ui);
            }
        });
    }

    fn inspect_mut(&mut self, label: &str, ui: &mut Ui) {
        egui::CollapsingHeader::new(label.to_string().add(format!("[{}]", N).as_str())).show(ui, |ui| {
            for (i, item) in self.iter_mut().enumerate() {
                item.inspect_mut("item", ui);
            }
        });
    }
}

impl<T: crate::EguiInspect + Default> crate::EguiInspect for Vec<T> {
    fn inspect(&self, label: &str, ui: &mut Ui) {
        egui::CollapsingHeader::new(label.to_string().add(format!("[{}]", self.len()).as_str())).show(ui, |ui| {
            for (i, item) in self.iter().enumerate() {
                item.inspect("item", ui);
            }
        });
    }

    fn inspect_mut(&mut self, label: &str, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            egui::CollapsingHeader::new(label.to_string().add(format!("[{}]", self.len()).as_str()))
                .id_source(label).show(ui, |ui| {
                for (i, item) in self.iter_mut().enumerate() {
                    item.inspect_mut("item", ui);
                }
            });

            let response = ui.button("Add");
            if response.clicked() {
                self.push(T::default());
            }

            let response = ui.button("Pop");
            if response.clicked() {
                self.pop();
            }
        });
    }
}
