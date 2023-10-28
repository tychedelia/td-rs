extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote};

use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, MetaNameValue,
    NestedMeta, Variant,
};


#[proc_macro_derive(Param)]
pub fn derive_param(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_ident = input.ident;
    let enum_data = if let Data::Enum(data) = input.data {
        data
    } else {
        panic!("`Param` can only be derived for enums");
    };

    let variant_names = enum_data
        .variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect::<Vec<String>>();

    let variant_labels = variant_names
        .iter()
        .map(|name| camel_case_to_words(name))
        .collect::<Vec<String>>();

    let variants = enum_data.variants.into_iter().collect::<Vec<Variant>>();
    let try_from_i32_match_arms = try_from_i32_match_arms(&variants);

    let output = quote! {
        impl MenuParam for #enum_ident {
            fn names() -> Vec<String> {
                vec![
                    #(String::from(#variant_names)),*
                ]
            }

            fn labels() -> Vec<String> {
                vec![
                    #(String::from(#variant_labels)),*
                ]
            }
        }

        impl std::convert::TryFrom<i32> for #enum_ident {
            type Error = String;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    #try_from_i32_match_arms
                    _ => Err(format!("Invalid value for {}: {}", stringify!(#enum_ident), value)),
                }
            }
        }

        impl Param for #enum_ident {
            fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
                let param: StringParameter = options.into();
                let names = #enum_ident::names();
                let labels = #enum_ident::labels();
                parameter_manager.append_menu(param, &names, &labels);
            }

            fn update(&mut self, name: &str, inputs: &ParamInputs) {
                let idx = inputs.get_int(name, 0);
                let value = #enum_ident::try_from(idx).unwrap();
                *self = value;
            }
        }
    };

    output.into()
}

fn try_from_i32_match_arms(variants: &[Variant]) -> proc_macro2::TokenStream {
    let arms = variants.iter().enumerate().map(|(idx, variant)| {
        let ident = &variant.ident;
        let index = idx as i32;
        quote! {
            #index => Ok(Self::#ident),
        }
    });

    quote! {
        #( #arms )*
    }
}

fn camel_case_to_words(s: &str) -> String {
    let mut words = String::new();

    for (i, c) in s.char_indices() {
        if i != 0 && c.is_uppercase() {
            words.push(' ');
        }
        words.push(c);
    }

    words
}

#[proc_macro_derive(Params, attributes(param))]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_params(&input)
}

fn impl_params(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut register_code = Vec::new();
    let mut update_code = Vec::new(); // Add this line to store update code

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(named_fields) = &data_struct.fields {
            for field in named_fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let _field_type = &field.ty;

                let mut label = None;
                let mut page = None;
                let mut min = None;
                let mut max = None;

                for attr in &field.attrs {
                    if attr.path.is_ident("param") {
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested_meta in meta_list.nested.iter() {
                                if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                    path,
                                    lit,
                                    ..
                                })) = nested_meta
                                {
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
                                    }
                                }
                            }
                        }
                    }
                }

                let field_name_upper = format_name(&field_name.to_string());
                let default_label = format!("{}", field_name);
                let label = label.unwrap_or_else(|| default_label);
                let default_page = "Custom".to_string();
                let page = page.unwrap_or_else(|| default_page);
                let min = min.unwrap_or(0.0);
                let max = max.unwrap_or(1.0);

                let code = quote! {
                    {
                        let options = ParamOptions {
                            name: #field_name_upper.to_string(),
                            label: #label.to_string(),
                            page: #page.to_string(),
                            min: #min,
                            max: #max,
                        };
                        Param::register(&self.#field_name, options, parameter_manager);
                    }
                };
                register_code.push(code);

                let update_field_code = quote! {
                    // TODO: Field name should be null terminated
                    Param::update(&mut self.#field_name, &(#field_name_upper.to_string()), inputs);
                };

                update_code.push(update_field_code);
            }
        }
    }

    let register_code = quote! { #(#register_code)* };

    let gen = quote! {
        impl #impl_generics OperatorParams for #struct_name #ty_generics #where_clause {
            fn register(&mut self, parameter_manager: &mut ParameterManager) {
                #register_code
            }

            fn update(&mut self, inputs: &ParamInputs) {
                #(#update_code)*
            }
        }
    };
    gen.into()
}

fn array_to_tokens(array: &[f64; 4]) -> TokenStream2 {
    let elems = array.iter().map(|elem| quote! { #elem });
    quote! { [#(#elems),*] }
}

fn format_name(name: &str) -> String {
    let name = remove_underscores(name);
    capitalize_first(&name)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

fn remove_underscores(s: &str) -> String {
    s.replace("_", "")
}
