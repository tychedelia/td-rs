#![feature(min_specialization)]

use std::any::Any;
use std::ops::DerefMut;
use td_rs_chop::param::MenuParam;
use td_rs_chop::*;
use td_rs_derive::{Param, Params};
use td_rs_derive_py::*;

#[derive(PyOp, Debug)]
pub struct PythonChop {
    #[py(get, set)]
    bar: i64,
    baz: u64,
    #[py]
    qux: bool,
}

/// Impl block providing default constructor for plugin
#[py_op_methods]
impl PythonChop {
    pub(crate) fn new() -> Self {
        Self {
            bar: 666,
            baz: 1234,
            qux: false,
        }
    }

    pub fn get_bar(&self) -> i64 {
        println!("get_bar: {:p}", self);
        self.bar
    }

    #[py_meth]
    pub unsafe fn foo(&mut self, args: &mut pyo3_ffi::PyObject, nargs: usize) -> *mut pyo3_ffi::PyObject {
        println!("foo: {:p}", self);
        self.bar = 1;
        self.baz = 2;
        pyo3_ffi::PyLong_FromLong(42)
    }
}

impl OpInfo for PythonChop {
    const OPERATOR_LABEL: &'static str = "Basic Python";
    const OPERATOR_TYPE: &'static str = "Basicgenerator";
}

impl Op for PythonChop {}

impl Chop for PythonChop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        println!("execute: {:p} bar: {}", self, self.get_bar());
    }

    fn general_info(&self, inputs: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            timeslice: false,
            input_match_index: 0,
        }
    }
}

chop_plugin!(PythonChop);