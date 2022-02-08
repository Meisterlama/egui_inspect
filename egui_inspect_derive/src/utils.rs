use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::Type::{Path, Reference};
use syn::{Field, Type};

pub fn get_path_str(type_path: &Type) -> String {
    match type_path {
        Path(type_path) => type_path
            .path
            .get_ident()
            .expect("Cannot find identifier")
            .to_string(),
        Reference(type_ref) => get_path_str(&*type_ref.elem),
        _ => "".to_string(),
    }
}

pub(crate) fn get_default_function_call(field: &Field, mutable: bool) -> TokenStream {
    let name = &field.ident;
    let name_str = name.clone().unwrap().to_string();

    return if mutable {
        quote_spanned! {field.span() => {egui_inspect::EguiInspect::inspect_mut(&mut self.#name, &#name_str, ui);}}
    } else {
        quote_spanned! {field.span() => {egui_inspect::EguiInspect::inspect(&self.#name, &#name_str, ui);}}
    };
}
