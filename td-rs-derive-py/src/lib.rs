extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::{
    DeriveInput, parse_macro_input,
};

#[proc_macro_derive(PyOp, attributes(py))]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_py_op(&input)
}

struct PyOpArgs {
    get: bool,
    set: bool,
    force_cook: bool,
    doc: Option<syn::LitStr>,
}


fn parse_attribute_args(args: syn::AttributeArgs) -> Result<PyOpArgs, syn::Error> {
    let mut get = false;
    let mut set = false;
    let mut auto_cook = false;
    let mut doc = None;

    for nested_meta in args {
        match nested_meta {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                if path.is_ident("get") {
                    get = true;
                } else if path.is_ident("set") {
                    set = true;
                } else if path.is_ident("auto_cook") {
                    auto_cook = true;
                } else {
                    return Err(syn::Error::new_spanned(path, "Unknown option"));
                }
            }
            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                if name_value.path.is_ident("doc") {
                    if let syn::Lit::Str(lit_str) = name_value.lit {
                        doc = Some(lit_str);
                    } else {
                        return Err(syn::Error::new_spanned(name_value.lit, "Expected a string literal for 'doc'"));
                    }
                } else {
                    return Err(syn::Error::new_spanned(name_value.path, "Unknown option"));
                }
            }
            _ => return Err(syn::Error::new_spanned(nested_meta, "Invalid option")),
        }
    }

    if !get && !set {
        get = true;
        set = true;
    }

    Ok(PyOpArgs { get, set, force_cook: auto_cook, doc })
}

fn impl_py_op(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    struct PyGetSet {
        name: syn::Ident,
        py_get_set_def: proc_macro2::TokenStream,
        get_body: Option<proc_macro2::TokenStream>,
        set_body: Option<proc_macro2::TokenStream>,
    }

    let gen = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                let generated_get_sets: Vec<_> = fields_named.named.iter().filter_map(|field| {
                    // Check for our attribute
                    if let Some(attr) = field.attrs.iter().find(|attr| attr.path.is_ident("py")) {
                        let args = if let Ok(meta) = attr.parse_meta() {
                            if let syn::Meta::List(meta_list) = meta {
                                parse_attribute_args(meta_list.nested.into_iter().collect()).unwrap()
                            } else {
                                panic!("Failed to parse attribute arguments")
                            }
                        } else {
                            panic!("Failed to parse attribute arguments")
                        };
                        let field_name = field.ident.as_ref().expect("Field must have a name").clone();
                        let field_type = &field.ty;
                        let getter_name = format_ident!("get_{}", field_name);
                        let setter_name = format_ident!("set_{}", field_name);
                        let auto_cook = args.force_cook;

                        // Match on the type of the field
                        let (return_converter, arg_checker, arg_reader) = match field_type {
                            syn::Type::Path(type_path) if type_path.path.is_ident("i32") => {
                                (
                                    quote! {
                                        pyo3_ffi::PyLong_FromLong
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_Check
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_AsLong
                                    }
                                )
                            }
                            syn::Type::Path(type_path) if type_path.path.is_ident("f32") => {
                                (
                                    quote! {
                                        pyo3_ffi::PyFloat_FromDouble
                                    },
                                    quote! {
                                        pyo3_ffi::PyFloat_Check
                                    },
                                    quote! {
                                        pyo3_ffi::PyFloat_AsDouble
                                    }
                                )
                            }
                            syn::Type::Path(type_path) if type_path.path.is_ident("i64") => {
                                (
                                    quote! {
                                        pyo3_ffi::PyLong_FromLongLong
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_Check
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_AsLongLong
                                    }
                                )
                            }
                            syn::Type::Path(type_path) if type_path.path.is_ident("u64") => {
                                (
                                    quote! {
                                        pyo3_ffi::PyLong_FromLongLong
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_Check
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_AsLongLong
                                    }
                                )
                            }
                            syn::Type::Path(type_path) if type_path.path.is_ident("bool") => {
                                (
                                    quote! {
                                        pyo3_ffi::PyBool_FromLong
                                    },
                                    quote! {
                                        pyo3_ffi::PyBool_Check
                                    },
                                    quote! {
                                        pyo3_ffi::PyLong_AsLong
                                    }
                                )
                            }
                            _ => {
                                panic!("Unsupported type")
                            }
                        };

                        let get_fn = if args.get {
                            Some(quote! {
                                pub unsafe extern "C" fn #getter_name(
                                    _self: *mut pyo3_ffi::PyObject,
                                    closure: *mut std::ffi::c_void
                                ) -> *mut pyo3_ffi::PyObject {
                                    let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
                                    let info = td_rs_chop::cxx::PY_GetInfo {
                                        autoCook: #auto_cook,
                                        reserved: [0; 50]
                                    };
                                    let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
                                    let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
                                    std::mem::forget(ctx);
                                    if me.is_null() {
                                            return std::ptr::null_mut();
                                    }
                                    let py_chop = {
                                        let me = &mut *(me as *mut td_rs_chop::cxx::RustChopPluginImplCpp);
                                        let me = me.As_RustChopPlugin().inner();
                                        &mut *(me as *mut #struct_name)
                                    };
                                    #return_converter(py_chop.#field_name)
                                }
                            })
                        } else {
                            None
                        };

                        let set_fn = if args.set {
                            Some(quote! {
                                pub unsafe extern "C" fn #setter_name(
                                    _self: *mut pyo3_ffi::PyObject,
                                    value: *mut pyo3_ffi::PyObject,
                                    closure: *mut std::ffi::c_void
                                ) -> i32 {
                                    if #arg_checker(value) == 0 {
                                        pyo3_ffi::PyErr_SetString(
                                            pyo3_ffi::PyExc_TypeError,
                                            "could not check argument\0"
                                                .as_ptr()
                                                .cast::<std::os::raw::c_char>(),
                                        );
                                    }

                                    let value = #arg_reader(value);
                                    if !pyo3_ffi::PyErr_Occurred().is_null() {
                                        pyo3_ffi::PyErr_SetString(
                                            pyo3_ffi::PyExc_TypeError,
                                            "could not read argument\0"
                                                .as_ptr()
                                                .cast::<std::os::raw::c_char>(),
                                        );
                                        return -1;
                                    }

                                    let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
                                    let info = td_rs_chop::cxx::PY_GetInfo {
                                        autoCook: #auto_cook,
                                        reserved: [0; 50]
                                    };
                                    let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
                                    let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
                                    if me.is_null() {
                                        pyo3_ffi::PyErr_SetString(
                                            pyo3_ffi::PyExc_TypeError,
                                            "operator is null\0"
                                                .as_ptr()
                                                .cast::<std::os::raw::c_char>(),
                                        );
                                        return -1;
                                    }
                                    let py_chop = {
                                        let me = &mut *(me as *mut td_rs_chop::cxx::RustChopPluginImplCpp);
                                        let me = me.As_RustChopPlugin().inner();
                                        &mut *(me as *mut #struct_name)
                                    };

                                    py_chop.#field_name = value;
                                    ctx.pin_mut().makeNodeDirty(std::ptr::null_mut());
                                    std::mem::forget(ctx);
                                    return 0;
                                }
                            })
                        } else {
                            None
                        };


                        let get = if args.get {
                            quote! {
                                Some(#getter_name)
                            }
                        } else {
                            quote! {
                                None
                            }
                        };
                        let set = if args.set {
                            quote! {
                                Some(#setter_name)
                            }
                        } else {
                            quote! {
                                None
                            }
                        };

                        let get_set_def = quote! {
                            pyo3_ffi::PyGetSetDef {
                                name: concat!(stringify!(#field_name), '\0').as_ptr().cast::<std::os::raw::c_char>(),
                                get: #get,
                                set: #set,
                                doc: "returns the sum of two integers as a string\0"
                                .as_ptr()
                                .cast::<std::os::raw::c_char>(),
                                closure: std::ptr::null_mut(),
                            },
                        };

                        Some(PyGetSet {
                            name: field_name,
                            py_get_set_def: get_set_def,
                            get_body: get_fn,
                            set_body: set_fn,
                        })
                    } else {
                        None
                    }
                }).collect();

                let defs: Vec<_> = generated_get_sets.iter().map(|gf| &gf.py_get_set_def).collect();
                let getters: Vec<_> = generated_get_sets.iter().map(|gf| &gf.get_body).collect();
                let setters: Vec<_> = generated_get_sets.iter().map(|gf| &gf.set_body).collect();
                let size = generated_get_sets.len() + 1;


                quote! {
                    pub const GETSETS: [pyo3_ffi::PyGetSetDef; #size] = [
                        #( #defs )*
                        pyo3_ffi::PyGetSetDef {
                            name: std::ptr::null_mut(),
                            get: None,
                            set: None,
                            doc: std::ptr::null_mut(),
                            closure: std::ptr::null_mut(),
                        }
                    ];

                    impl PyGetSets for #struct_name {
                        fn get_get_sets() -> &'static [pyo3_ffi::PyGetSetDef] {
                            &GETSETS
                        }
                    }

                    #( #getters )*
                    #( #setters )*
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    gen.into()
}

fn is_py_meth_attr(attr: &syn::Attribute) -> bool {
    attr.path.is_ident("py_meth")
}

#[proc_macro_attribute]
pub fn py_op_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let struct_name = &input.self_ty;
    struct PyMeth {
        name: syn::Ident,
        py_meth: proc_macro2::TokenStream,
        fn_body: proc_macro2::TokenStream,
    }

    let generated_functions: Vec<_> = input.items.iter().filter_map(|item| {
        if let syn::ImplItem::Method(method) = item {
            let has_py_meth = method.attrs.iter().any(|attr| is_py_meth_attr(&attr));
            if has_py_meth {
                let fn_name = &method.sig.ident;
                Some(PyMeth {
                    name: fn_name.clone(),
                    py_meth: quote! {
                      pyo3_ffi::PyMethodDef {
                            ml_name: concat!(stringify!(#fn_name), '\0').as_ptr().cast::<std::os::raw::c_char>(),
                            ml_meth: pyo3_ffi::PyMethodDefPointer {
                                _PyCFunctionFast: #fn_name,
                            },
                            ml_flags: pyo3_ffi::METH_FASTCALL,
                            ml_doc: "returns the sum of two integers as a string\0"
                            .as_ptr()
                            .cast::<std::os::raw::c_char>(),
                      },
                    },
                    fn_body: quote! {
                        pub unsafe extern "C" fn #fn_name(
                            _self: *mut pyo3_ffi::PyObject,
                            args: *mut *mut pyo3_ffi::PyObject,
                            nargs: pyo3_ffi::Py_ssize_t,
                        ) -> *mut pyo3_ffi::PyObject {
                            let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
                            let info = td_rs_chop::cxx::PY_GetInfo {
                                autoCook: true,
                                reserved: [0; 50]
                            };
                            let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
                            let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
                            if me.is_null() {
                                pyo3_ffi::PyErr_SetString(
                                    pyo3_ffi::PyExc_TypeError,
                                    "operator is null\0"
                                        .as_ptr()
                                        .cast::<std::os::raw::c_char>(),
                                );
                                return std::ptr::null_mut();
                            }
                            let py_chop = {
                                &mut *(me as *mut #struct_name)
                            };
                            std::mem::forget(ctx);
                            py_chop.#fn_name(&mut **args, nargs as usize)
                        }
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    let methods: Vec<_> = generated_functions.iter().map(|gf| &gf.py_meth).collect();
    let fns: Vec<_> = generated_functions.iter().map(|gf| &gf.fn_body).collect();
    let size = generated_functions.len() + 1;

    let gen = quote! {
        #input

        impl PyMethods for #struct_name {
            fn get_methods() -> &'static [pyo3_ffi::PyMethodDef] {
                &METHODS
            }
        }

        pub const METHODS: [pyo3_ffi::PyMethodDef; #size] = [
            #( #methods )*
            pyo3_ffi::PyMethodDef::zeroed()
        ];

        #( #fns )*
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn py_meth(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input // just return the input unchanged
}