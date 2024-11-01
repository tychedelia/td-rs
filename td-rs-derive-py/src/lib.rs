extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PyOp, attributes(py))]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_py_op(&input)
}

struct PyOpArgs {
    get: bool,
    set: bool,
    auto_cook: bool,
    doc: Option<syn::LitStr>,
}

impl Default for PyOpArgs {
    fn default() -> Self {
        Self {
            get: true,
            set: true,
            auto_cook: false,
            doc: None,
        }
    }
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
                        return Err(syn::Error::new_spanned(
                            name_value.lit,
                            "Expected a string literal for 'doc'",
                        ));
                    }
                } else {
                    return Err(syn::Error::new_spanned(name_value.path, "Unknown option"));
                }
            }
            _ => return Err(syn::Error::new_spanned(nested_meta, "Invalid option")),
        }
    }

    // allow user to just pass doc / autocook
    if !get && !set {
        get = true;
        set = true;
    }
    Ok(PyOpArgs {
        get,
        set,
        auto_cook,
        doc,
    })
}

fn impl_py_op(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    struct PyGetSet {
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
                                parse_attribute_args(meta_list.nested.into_iter().collect()).expect("Failed to parse attribute args")
                            } else {
                                PyOpArgs::default()
                            }
                        } else {
                            PyOpArgs::default()
                        };
                        let field_name = field.ident.as_ref().expect("Field must have a name").clone();
                        let field_type = &field.ty;
                        let getter_name = format_ident!("get_{}", field_name);
                        let setter_name = format_ident!("set_{}", field_name);
                        let auto_cook = args.auto_cook;

                        let get_fn = if args.get {
                            Some(quote! {
                                pub unsafe extern "C" fn #getter_name(
                                    _self: *mut pyo3_ffi::PyObject,
                                    closure: *mut std::ffi::c_void
                                ) -> *mut pyo3_ffi::PyObject {
                                    use cxx::AsPlugin;
                                    let py_struct = _self as *mut cxx::PY_Struct;
                                    let info = cxx::PY_GetInfo {
                                        autoCook: #auto_cook,
                                        reserved: [0; 50]
                                    };

                                    let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                                    let me = ctx.getNodeInstance(&info, std::ptr::null_mut());
                                    if me.is_null() {
                                            return std::ptr::null_mut();
                                    }
                                    let py_chop = {
                                        let me = cxx::plugin_cast(me);
                                        let me = me.as_plugin().inner();
                                        &mut *(me as *mut #struct_name)
                                    };
                                    py::ToPyObj::to_py_obj(py_chop.#field_name)
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
                                    use cxx::AsPlugin;
                                    if !<#field_type as CheckPyObj>::check_py_obj(value) {
                                        pyo3_ffi::PyErr_SetString(
                                            pyo3_ffi::PyExc_TypeError,
                                            "could not check argument\0"
                                                .as_ptr()
                                                .cast::<std::os::raw::c_char>(),
                                        );
                                        return -1;
                                    }

                                    let value = FromPyObj::from_py_obj(value);

                                    let py_struct = _self as *mut cxx::PY_Struct;
                                    let info = cxx::PY_GetInfo {
                                        autoCook: #auto_cook,
                                        reserved: [0; 50]
                                    };
                                    let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                                    let me = ctx.getNodeInstance(&info, std::ptr::null_mut());
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
                                        let me = cxx::plugin_cast(me);
                                        let me = me.as_plugin_mut().innerMut();
                                        &mut *(me as *mut #struct_name)
                                    };

                                    py_chop.#field_name = value;
                                    let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                                    ctx.makeNodeDirty(std::ptr::null_mut());
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

                        let doc = if let Some(doc) = &args.doc {
                            quote! {
                                concat!(stringify!(#doc), '\0').as_ptr().cast::<std::os::raw::c_char>()
                            }
                        } else {
                            quote! {
                                std::ptr::null_mut()
                            }
                        };
                        let get_set_def = quote! {
                            pyo3_ffi::PyGetSetDef {
                                name: concat!(stringify!(#field_name), '\0').as_ptr().cast::<std::os::raw::c_char>(),
                                get: #get,
                                set: #set,
                                doc: #doc,
                                closure: std::ptr::null_mut(),
                            },
                        };

                        Some(PyGetSet {
                            py_get_set_def: get_set_def,
                            get_body: get_fn,
                            set_body: set_fn,
                        })
                    } else {
                        None
                    }
                }).collect();

                let defs: Vec<_> = generated_get_sets
                    .iter()
                    .map(|gf| &gf.py_get_set_def)
                    .collect();
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
pub fn py_op_methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let struct_name = &input.self_ty;
    struct PyMeth {
        py_meth: proc_macro2::TokenStream,
        fn_body: proc_macro2::TokenStream,
    }

    let generated_functions: Vec<_> = input.items.iter().filter_map(|item| {
        if let syn::ImplItem::Method(method) = item {
            let has_py_meth = method.attrs.iter().any(is_py_meth_attr);
            if has_py_meth {
                let fn_name = &method.sig.ident;
                Some(PyMeth {
                    py_meth: quote! {
                      pyo3_ffi::PyMethodDef {
                            ml_name: concat!(stringify!(#fn_name), '\0').as_ptr().cast::<std::os::raw::c_char>(),
                            ml_meth: pyo3_ffi::PyMethodDefPointer {
                                _PyCFunctionFast: #fn_name,
                            },
                            ml_flags: pyo3_ffi::METH_FASTCALL,
                            ml_doc: std::ptr::null_mut(),
                      },
                    },
                    fn_body: quote! {
                        pub unsafe extern "C" fn #fn_name(
                            _self: *mut pyo3_ffi::PyObject,
                            args: *mut *mut pyo3_ffi::PyObject,
                            nargs: pyo3_ffi::Py_ssize_t,
                        ) -> *mut pyo3_ffi::PyObject {
                            use cxx::AsPlugin;
                            let py_struct = _self as *mut cxx::PY_Struct;
                            let info = cxx::PY_GetInfo {
                                autoCook: true,
                                reserved: [0; 50]
                            };
                            let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                            let me = ctx.getNodeInstance(&info, std::ptr::null_mut());
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
                                let me = cxx::plugin_cast(me);
                                let me = me.as_plugin_mut().innerMut();
                                &mut *(me as *mut #struct_name)
                            };
                            let res = py_chop.#fn_name(args, nargs as usize);
                            let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                            ctx.makeNodeDirty(std::ptr::null_mut());
                            res
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
