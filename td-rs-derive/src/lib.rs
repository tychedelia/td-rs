extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{AttributeArgs, Data, DeriveInput, Fields, Lit, Meta, MetaNameValue, NestedMeta, parse_macro_input};

#[proc_macro_derive(Params)]
pub fn parameter_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Implement the macro
    impl_parameter(&input)
}

fn impl_parameter(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut register_code = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(named_fields) = &data_struct.fields {
            for field in named_fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let mut label = None;
                let mut page = None;
                let mut min = None;
                let mut max = None;
                let mut default = None;

                for attr in &field.attrs {
                    if attr.path.is_ident("parameter") {
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested_meta in meta_list.nested.iter() {
                                if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = nested_meta {
                                    if path.is_ident("label") {
                                        if let Lit::Str(lit_str) = lit {
                                            label = Some(lit_str.value());
                                        }
                                    } else if path.is_ident("page") {
                                        if let Lit::Str(lit_str) = lit {
                                            page = Some(lit_str.value());
                                        }
                                    } else if path.is_ident("min") {
                                        if let Lit::Float(lit_float) = lit {
                                            min = Some(lit_float.base10_parse().unwrap());
                                        }
                                    } else if path.is_ident("max") {
                                        if let Lit::Float(lit_float) = lit {
                                            max = Some(lit_float.base10_parse().unwrap());
                                        }
                                    } else if path.is_ident("default") {
                                        if let Lit::Float(lit_float) = lit {
                                            default = Some(lit_float.base10_parse().unwrap());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if is_numeric_type(field_type) {
                    let field_name_upper = capitalize_first(&field_name.to_string()); // Capitalize the field name
                    let default_label = format!("{}", field_name);
                    let label = label.unwrap_or_else(|| default_label);
                    let default_page = "Custom".to_string();
                    let page = page.unwrap_or_else(|| default_page);
                    let min_values = array_to_tokens(&[min.unwrap_or(0.0); 4]);
                    let max_values = array_to_tokens(&[max.unwrap_or(0.0); 4]);
                    let default_values = array_to_tokens(&[default.unwrap_or(0.0); 4]);

                    let code = quote! {
                        {
                            use td_rs_chop::cxx::ffi::NumericParameter;
                            let mut np = NumericParameter {
                                name: #field_name_upper.to_string(),
                                label: #label.to_string(),
                                page: #page.to_string(),
                                min_values: #min_values,
                                max_values: #max_values,
                                default_values: #default_values,
                                ..Default::default()
                            };
                            parameter_manager.append_float(np);
                        }
                    };
                    register_code.push(code);
                }
            }
        }
    }

    let register_code = quote! { #(#register_code)* };

    let gen = quote! {
        impl #impl_generics ChopParams for #struct_name #ty_generics #where_clause {
            fn register(&mut self, parameter_manager: &mut ParameterManager) {
                #register_code
            }

            fn update(&mut self, input: &OperatorInput) {
                // Update the parameter value
                // ...
            }
        }
    };
    gen.into()
}


fn is_numeric_type(field_type: &syn::Type) -> bool {
    // Add more numeric types as necessary
    let numeric_types = [
        "f32",
        "f64",
    ];

    for numeric_type in numeric_types.iter() {
        if let syn::Type::Path(type_path) = field_type {
            if type_path.path.is_ident(numeric_type) {
                return true;
            }
        }
    }
    false
}

fn array_to_tokens(array: &[f64; 4]) -> TokenStream2 {
    let elems = array.iter().map(|elem| quote! { #elem });
    quote! { [#(#elems),*] }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}