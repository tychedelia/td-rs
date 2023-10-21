extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{
    DeriveInput, parse_macro_input,
};

#[proc_macro_attribute]
pub fn py_op(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let gen = quote! {
        use std::os::raw::c_char;
        use std::ptr;
        use pyo3_ffi::*;

        pub const METHODS: [pyo3_ffi::PyMethodDef; 2] = [
            PyMethodDef {
                ml_name: "sum_as_string\0".as_ptr().cast::<c_char>(),
                ml_meth: PyMethodDefPointer {
                    _PyCFunctionFast: sum_as_string,
                },
                ml_flags: METH_FASTCALL,
                ml_doc: "returns the sum of two integers as a string\0"
                .as_ptr()
                .cast::<c_char>(),
            },
            PyMethodDef::zeroed()
        ];

        pub unsafe extern "C" fn sum_as_string(
            _self: *mut PyObject,
            args: *mut *mut PyObject,
            nargs: Py_ssize_t,
        ) -> *mut PyObject {
            let py_struct = _self as *mut td_rs_chop::cxx::PY_Struct;
            println!("  Hello from Rust!");
            println!("${:?}", (*py_struct).context);
            let info = td_rs_chop::cxx::PY_GetInfo {
                autoCook: false,
                reserved: [0; 50]
            };
            let mut ctx = td_rs_chop::cxx::getPyContext(py_struct);
            let me = ctx.pin_mut().getNodeInstance(&info, std::ptr::null_mut());
            let py_chop = {
                  &mut *(me as *mut $struct_name)
            };
            py_chop.wow();

            if nargs != 2 {
                PyErr_SetString(
                    PyExc_TypeError,
                    "sum_as_string() expected 2 positional arguments\0"
                        .as_ptr()
                        .cast::<c_char>(),
                );
                return std::ptr::null_mut();
            }

            let arg1 = *args;
            if PyLong_Check(arg1) == 0 {
                PyErr_SetString(
                    PyExc_TypeError,
                    "sum_as_string() expected an int for positional argument 1\0"
                        .as_ptr()
                        .cast::<c_char>(),
                );
                return std::ptr::null_mut();
            }

            let arg1 = PyLong_AsLong(arg1);
            if !PyErr_Occurred().is_null() {
                return ptr::null_mut();
            }

            let arg2 = *args.add(1);
            if PyLong_Check(arg2) == 0 {
                PyErr_SetString(
                    PyExc_TypeError,
                    "sum_as_string() expected an int for positional argument 2\0"
                        .as_ptr()
                        .cast::<c_char>(),
                );
                return std::ptr::null_mut();
            }

            let arg2 = PyLong_AsLong(arg2);
            if !PyErr_Occurred().is_null() {
                return ptr::null_mut();
            }

            match arg1.checked_add(arg2) {
                Some(sum) => {
                    let string = sum.to_string();
                    PyUnicode_FromStringAndSize(string.as_ptr().cast::<c_char>(), string.len() as isize)
                }
                None => {
                    PyErr_SetString(
                        PyExc_OverflowError,
                        "arguments too large to add\0".as_ptr().cast::<c_char>(),
                    );
                    std::ptr::null_mut()
                }
            }
        }

        // impl #impl_generics OperatorParams for #struct_name #ty_generics #where_clause {
        //
        // }
    };
    gen.into()
}