use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::Type::{Path, Reference};
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, FieldsNamed,
    GenericParam, Generics, Type,
};

use darling::FromField;

#[derive(Debug, FromField)]
#[darling(attributes(inspect), default)]
struct AttributeArgs {
    ident: Option<Ident>,
    slider: bool,
    min: f32,
    max: f32,
    multiline: bool,
}

impl Default for AttributeArgs {
    fn default() -> Self {
        Self {
            ident: None,
            slider: true,
            min: 0.0,
            max: 100.0,
            multiline: false,
        }
    }
}

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn derive_egui_inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inspect = inspect_struct(&input.data, &name, false);

    let inspect_mut = inspect_struct(&input.data, &name, true);

    let expanded = quote! {
        impl #impl_generics egui_inspect::EguiInspect for #name #ty_generics #where_clause {
            fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
                #inspect
            }
            fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
                #inspect_mut
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(egui_inspect::EguiInspect));
        }
    }
    generics
}

fn inspect_struct(data: &Data, struct_name: &Ident, mutable: bool) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => handle_named_fields(fields, mutable),
            _ => {
                quote! {}
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn path_is_internally_handled(path_str: &String) -> bool {
    return path_str == "f32"
        || path_str == "f64"
        || path_str == "u8"
        || path_str == "i8"
        || path_str == "u16"
        || path_str == "i16"
        || path_str == "u32"
        || path_str == "i32"
        || path_str == "u64"
        || path_str == "i64"
        || path_str == "usize"
        || path_str == "isize"
        || path_str == "bool"
        || path_str == "String"
        || path_str == "str";
}

fn get_path_str(type_path: &Type) -> String {
    match type_path {
        Path(type_path) => type_path.path.get_ident().unwrap().to_string(),
        Reference(type_ref) => get_path_str(&*type_ref.elem),
        _ => "".to_string(),
    }
}

fn handle_named_fields(fields: &FieldsNamed, mutable: bool) -> TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        let name_str = name.clone().unwrap().to_string();

        let attr = AttributeArgs::from_field(f).unwrap();
        let slider = attr.slider;
        let min = attr.min;
        let max = attr.max;
        let multiline = attr.multiline;

        let path_str = get_path_str(&f.ty);

        let func = if mutable { quote!(inspect_mut) } else { quote!(inspect) };
        let ref_type = if mutable { quote!(&mut) } else { quote!(&) };
        let default_function_call = quote_spanned! {f.span() => {egui_inspect::EguiInspect::#func(#ref_type self.#name, &#name_str, ui);}};

        return if path_is_internally_handled(&path_str) {
            match path_str.as_str() {
                "f64" | "f32" | "u8" | "i8" |
                "u16" | "i16" | "u32" | "i32" |
                "u64" | "i64" => {
                    if mutable {
                        if slider {
                            return quote_spanned! {f.span() => {
                                egui_inspect::InspectNumber::inspect_with_slider(#ref_type self.#name, &#name_str, ui, #min, #max);
                                    }
                                };
                        } else {
                            return quote_spanned! {f.span() => {
                                    egui_inspect::InspectNumber::inspect_with_drag_value(#ref_type self.#name, &#name_str, ui);
                                    }
                                };
                        }
                    } else {
                        return default_function_call;
                    }
                },
                "String" => {
                    if mutable {
                        if multiline {
                            return quote_spanned! {f.span() => {
                                egui_inspect::InspectString::inspect_mut_multiline(#ref_type self.#name, &#name_str, ui);
                                    }
                                }
                        }
                        else {
                            return quote_spanned! {f.span() => {
                                egui_inspect::InspectString::inspect_mut_singleline(#ref_type self.#name, &#name_str, ui);
                                    }
                                }
                        }
                    } else {
                        return default_function_call
                    }
                }
                _ => default_function_call,
            }
        } else {
            default_function_call
        }
    });
    quote! {
        ui.strong(label);
        #(#recurse)*
    }
}
