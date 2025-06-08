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
                quote! {
                    impl<'a, 'py> pyo3::impl_::extract_argument::ExtractPyClassRef<'a, 'py> for #struct_name {
                        fn extract_ref(
                            obj: &'a pyo3::Bound<'py, pyo3::PyAny>,
                            holder: &'a mut Option<pyo3::PyRef<'py, Self>>,
                        ) -> pyo3::PyResult<&'a Self> {
                            unsafe {
                                let me = obj.as_ptr();
                                let py_struct = me as *mut cxx::PY_Struct;
                                let info = cxx::PY_GetInfo {
                                    autoCook: true,
                                    reserved: [0; 50],
                                };
                                // SAFETY:
                                // Pinning the context is safe because the context is not moved or dropped as it is
                                // derived from our C++ operator instance which is not moved or dropped during the
                                // lifetime of the Python object.
                                let mut ctx = Pin::new_unchecked(&mut *cxx::getPyContext(py_struct));
                                // Look up our operator instance.
                                let me = ctx.getNodeInstance(&info, std::ptr::null_mut());
                                if me.is_null() {
                                    return Err(pyo3::exceptions::PyTypeError::new_err("operator is null"));
                                }
                                let py_op = {
                                    let me = cxx::plugin_cast(me);
                                    let me = me.as_plugin().inner();
                                    &*(me as *const #struct_name)
                                };

                                Ok(py_op)
                            }
                        }
                    }

                    impl<'a, 'py> pyo3::impl_::extract_argument::ExtractPyClassRefMut<'a, 'py> for #struct_name {
                        fn extract_mut(
                            obj: &'a pyo3::Bound<'py, pyo3::PyAny>,
                            holder: &'a mut Option<pyo3::PyRefMut<'py, Self>>,
                        ) -> PyResult<&'a mut Self> {
                            unsafe {
                                let me = obj.as_ptr();
                                let py_struct = me as *mut cxx::PY_Struct;
                                let info = cxx::PY_GetInfo {
                                    autoCook: true,
                                    reserved: [0; 50],
                                };
                                // SAFETY:
                                // Pinning the context is safe because the context is not moved or dropped as it is
                                // derived from our C++ operator instance which is not moved or dropped during the
                                // lifetime of the Python object.
                                let py_ctx = cxx::getPyContext(py_struct);
                                // Mark the node as dirty so that it will be cooked on the next frame.
                                Pin::new_unchecked(&mut *py_ctx).makeNodeDirty(std::ptr::null_mut());
                                // Look up our operator instance.
                                let me = Pin::new_unchecked(&mut *py_ctx).getNodeInstance(&info, std::ptr::null_mut());
                                if me.is_null() {
                                    return Err(pyo3::exceptions::PyTypeError::new_err("operator is null"));
                                }
                                // SAFETY:
                                // We have a valid operator instance pointer
                                let py_op = {
                                    let me = cxx::plugin_cast(me);
                                    let me = me.as_plugin_mut().innerMut();
                                    &mut *(me as *mut #struct_name)
                                };
                                Ok(py_op)
                            }
                        }
                    }

                    impl PyGetSets for #struct_name {
                        fn get_get_sets() -> &'static [pyo3::ffi::PyGetSetDef] {
                            let clazz = pyo3::impl_::pyclass::PyClassImplCollector::<#struct_name>::new();
                            let methods = <pyo3::impl_::pyclass::PyClassImplCollector::<#struct_name> as pyo3::impl_::pyclass::PyMethods::<#struct_name>>::py_methods(clazz);
                            let mut getset_builders = std::collections::HashMap::<&std::ffi::CStr, pyo3::pyclass::create_type_object::GetSetDefBuilder>::new();
                            for method in methods.methods {
                                let method_def = match method {
                                    pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Runtime(m) => &m(),
                                    pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Static(m) => m,
                                };

                                match method_def {
                                    pyo3::PyMethodDefType::Getter(getter) => {
                                        getset_builders
                                            .entry(getter.name)
                                            .or_default()
                                            .add_getter(getter)
                                    }
                                    pyo3::PyMethodDefType::Setter(setter) => {
                                        getset_builders
                                            .entry(setter.name)
                                            .or_default()
                                            .add_setter(setter)
                                    }
                                    _ => {}
                                }
                            }

                            let items = #struct_name::items_iter();
                            for item in items {
                                for method in item.methods {
                                    let method_def = match method {
                                        pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Runtime(m) => &m(),
                                        pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Static(m) => m,
                                    };

                                    match method_def {
                                        pyo3::PyMethodDefType::Getter(getter) => {
                                            getset_builders
                                                .entry(getter.name)
                                                .or_default()
                                                .add_getter(getter)
                                        }
                                        pyo3::PyMethodDefType::Setter(setter) => {
                                            getset_builders
                                                .entry(setter.name)
                                                .or_default()
                                                .add_setter(setter)
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            let mut getset_destructors = Vec::with_capacity(getset_builders.len());

                            let property_defs: Vec<pyo3::ffi::PyGetSetDef> = getset_builders
                                .iter()
                                .map(|(name, builder)| {
                                    let (def, destructor) = builder.as_get_set_def(name);
                                    getset_destructors.push(destructor);
                                    def
                                })
                                .collect();


                            // We just have to leak these to keep them alive
                            // TODO: right now we are leaking the memory, we should free it when the plugin is unloaded
                            // but unless you're loading and unloading the plugin a lot, it's not a big deal
                            getset_destructors.leak();
                            property_defs.leak()
                        }
                    }

                    impl PyMethods for #struct_name {
                        fn get_methods() -> &'static [pyo3::ffi::PyMethodDef] {
                            let clazz = pyo3::impl_::pyclass::PyClassImplCollector::<#struct_name>::new();
                            let methods = <pyo3::impl_::pyclass::PyClassImplCollector::<#struct_name> as pyo3::impl_::pyclass::PyMethods::<#struct_name>>::py_methods(clazz);
                            let mut method_defs = Vec::new();
                            for method in methods.methods {
                                let method_def = match method {
                                    pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Runtime(m) => &m(),
                                    pyo3::impl_::pyclass::MaybeRuntimePyMethodDef::Static(m) => m,
                                };

                                match method_def {
                                    pyo3::PyMethodDefType::Method(m) => {
                                        method_defs.push(m.as_method_def());
                                    }
                                    _ => {}
                                }
                            }

                            method_defs.leak()
                        }
                    }


                    impl PyOp for #struct_name {}
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
                      pyo3::ffi::PyMethodDef {
                            ml_name: concat!(stringify!(#fn_name), '\0').as_ptr().cast::<std::os::raw::c_char>(),
                            ml_meth: pyo3::ffi::PyMethodDefPointer {
                                _PyCFunctionFast: #fn_name,
                            },
                            ml_flags: pyo3::ffi::METH_FASTCALL,
                            ml_doc: std::ptr::null_mut(),
                      },
                    },
                    fn_body: quote! {
                        pub unsafe extern "C" fn #fn_name(
                            _self: *mut pyo3::ffi::PyObject,
                            args: *mut *mut pyo3::ffi::PyObject,
                            nargs: pyo3::ffi::Py_ssize_t,
                        ) -> *mut pyo3::ffi::PyObject {
                            use cxx::AsPlugin;
                            let py_struct = _self as *mut cxx::PY_Struct;
                            let info = cxx::PY_GetInfo {
                                autoCook: true,
                                reserved: [0; 50]
                            };
                            let mut ctx = std::pin::Pin::new_unchecked(&mut*cxx::getPyContext(py_struct));;
                            let me = ctx.getNodeInstance(&info, std::ptr::null_mut());
                            if me.is_null() {
                                pyo3::ffi::PyErr_SetString(
                                    pyo3::ffi::PyExc_TypeError,
                                    "operator is null\0"
                                        .as_ptr()
                                        .cast::<std::os::raw::c_char>(),
                                );
                                return std::ptr::null_mut();
                            }
                            let py_op = {
                                let me = cxx::plugin_cast(me);
                                let me = me.as_plugin_mut().innerMut();
                                &mut *(me as *mut #struct_name)
                            };
                            let res = py_op.#fn_name(args, nargs as usize);
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
            fn get_methods() -> &'static [pyo3::ffi::PyMethodDef] {
                &METHODS
            }
        }

        pub const METHODS: [pyo3::ffi::PyMethodDef; #size] = [
            #( #methods )*
            pyo3::ffi::PyMethodDef::zeroed()
        ];

        #( #fns )*
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn py_meth(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input // just return the input unchanged
}
