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

fn impl_py_op(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let gen = quote! {

    };
    gen.into()
}

// fn py_op(attr: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = &input.ident;
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//
//     let gen = quote! {
//         #input
//         pub const GETSETS: [pyo3_ffi::PyGetSetDef; 2] = [
//             pyo3_ffi::PyGetSetDef {
//                 name: "bar\0".as_ptr().cast::<std::os::raw::c_char>(),
//                 get: Some(sum_as_string),
//                 set: None,
//                 doc: "returns the sum of two integers as a string\0"
//                 .as_ptr()
//                 .cast::<c_char>(),
//                 closure: std::ptr::null_mut(),
//             },
//             pyo3_ffi::PyGetSetDef {
//                 name: std::ptr::null_mut(),
//                 get: None,
//                 set: None,
//                 doc: std::ptr::null_mut(),
//                 closure: std::ptr::null_mut(),
//             }
//         ];
//
//         pub unsafe extern "C" fn sum_as_string(
//             _self: *mut PyObject,
//             closure: *mut std::ffi::c_void
//         ) -> *mut PyObject {
//             let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
//             let info = td_rs_chop::cxx::PY_GetInfo {
//                 autoCook: false,
//                 reserved: [0; 50]
//             };
//             let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
//             let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
//             let py_chop = {
//                 &mut *(me as *mut #struct_name)
//             };
//             PyLong_FromLong(py_chop.bar)
//         }
//     };
//     gen.into()
// }

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
                      PyMethodDef {
                            ml_name: concat!(stringify!(#fn_name), '\0').as_ptr().cast::<c_char>(),
                            ml_meth: PyMethodDefPointer {
                                _PyCFunctionFast: #fn_name,
                            },
                            ml_flags: METH_FASTCALL,
                            ml_doc: "returns the sum of two integers as a string\0"
                            .as_ptr()
                            .cast::<c_char>(),
                      },
                    },
                    fn_body: quote! {
                       pub unsafe extern "C" fn #fn_name(
                            _self: *mut PyObject,
                            args: *mut *mut PyObject,
                            nargs: Py_ssize_t,
                        ) -> *mut PyObject {
                            let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
                            let info = td_rs_chop::cxx::PY_GetInfo {
                                autoCook: false,
                                reserved: [0; 50]
                            };
                            let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
                            let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
                            let py_chop = {
                                &mut *(me as *mut #struct_name)
                            };
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

        pub use pyo3_ffi::*;
        pub use std::ffi::c_char;

        pub const METHODS: [pyo3_ffi::PyMethodDef; #size] = [
            #( #methods )*
            PyMethodDef::zeroed()
        ];

        #( #fns )*
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn py_meth(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input // just return the input unchanged
}