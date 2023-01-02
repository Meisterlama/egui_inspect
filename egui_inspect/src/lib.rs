//! # egui_inspect
//! This crate expose macros and traits to generate boilerplate code
//! for structs inspection and edition.
//!
//! Basic usage would be
//! ```
//! # use egui_inspect::*;
//! #[derive(EguiInspect)]
//! struct MyApp {
//!     #[inspect(no_edit)]
//!     string: String,
//!     #[inspect(multiline)]
//!     code: String,
//!     #[inspect(min = 12.0, max = 53.0)]
//!     unsigned32: u32,
//!     #[inspect(hide)]
//!     skipped: bool,
//!     #[inspect(custom_func_mut = "custom_bool_inspect")]
//!     boolean: bool,
//!     #[inspect(no_edit)]
//!     raw_string: &'static str,
//!     #[inspect(slider, min = -43.0, max = 125.0)]
//!     float64: f32,
//! }
//!
//! fn custom_bool_inspect(boolean: &mut bool, label: &'static str, ui: &mut egui::Ui) {
//!    ui.label("C'EST LA GIGA FONCTION CUSTOM WÉ");
//!    boolean.inspect(label, ui);
//! }
//!
//! fn main() {
//!     let app = MyApp::default();
//!     app.inspect("My App", &ui); // here `ui` would be some `&mut egui::Ui`
//! }
//! ```
//!
//! You can add attributes to structures field.
//! Currently supported attributes are defined in the struct AttributeArgs of egui_inspect_derive
//!
//! Here is a list of supported attributes.
//! It might not be up to date, it's better to check directly AttributeArgs declaration
//!
//! - `hide` *(bool)*: If true, doesn't generate code for the given field
//! - `no_edit` *(bool)*: If true, never call mut function for the given field (May be overridden by other params)
//! - `slider` *(bool)*: If true, use a slider when inspecting numbers (`mut` only)
//! - `min` *(f32)*: Min value for inspecting numbers (`mut` only)
//! - `max` *(f32)*: Max value for inspecting numbers (`mut` only)
//! - `multiline` *(bool)*: If true, display the text on multiple lines (`mut` only)
//! - `custom_func` *(string)*: Use custom function for non-mut inspect (Evaluate the string as a function path)
//! - `custom_func_mut` *(string)*: Use custom function for mut inspect (Evaluate the string as a function path)
//!

/// See also [EguiInspect]
pub use egui_inspect_derive::*;

/// Base trait to automatically inspect structs
pub trait EguiInspect {
    fn inspect(&self, label: &str, ui: &mut egui::Ui);
    fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui);
}

pub trait InspectNumber {
    fn inspect_with_slider(&mut self, label: &str, ui: &mut egui::Ui, min: f32, max: f32);
    fn inspect_with_drag_value(&mut self, label: &str, ui: &mut egui::Ui);
}

pub trait InspectString {
    fn inspect_mut_multiline(&mut self, label: &str, ui: &mut egui::Ui);
    fn inspect_mut_singleline(&mut self, label: &str, ui: &mut egui::Ui);
}

pub mod base_type_inspect;
