extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{
    DeriveInput, parse_macro_input,
};

#[proc_macro_derive(PyOp, attributes(method, get, set))]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_python(&input)
}

fn impl_python(input: &DeriveInput) -> TokenStream {
    pub use pyo3_macros_backend::*;
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let gen = quote! {
        static mut METHODS: [PyMethodDef; 2] = [
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

        impl #impl_generics OperatorParams for #struct_name #ty_generics #where_clause {

        }
    };
    gen.into()
}