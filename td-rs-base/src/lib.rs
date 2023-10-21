#![feature(associated_type_defaults)]

pub mod chop;
pub mod cxx;
pub mod dat;
pub mod param;
pub mod sop;
pub mod top;

use crate::cxx::OP_SOPInput;
pub use param::*;
use ref_cast::RefCast;
use std::cell::OnceCell;
use std::ffi;
use std::ops::{Add, Deref, DerefMut, Index};

static mut INFO_STR: OnceCell<String> = OnceCell::new();
static mut ERROR_STR: OnceCell<String> = OnceCell::new();
static mut WARNING_STR: OnceCell<String> = OnceCell::new();

/// Metadata describing the operator plugin.
pub trait OpInfo {
    /// The type of the operator.
    const OPERATOR_TYPE: &'static str = "";
    /// The label of the operator.
    const OPERATOR_LABEL: &'static str = "";
    /// The icon of the operator.
    const OPERATOR_ICON: &'static str = "";
    /// The minimum number of inputs the operator accepts.
    const MIN_INPUTS: usize = 0;
    /// The maximum number of inputs the operator accepts.
    const MAX_INPUTS: usize = 0;
    /// The author name of the operator.
    const AUTHOR_NAME: &'static str = "";
    /// The author email of the operator.
    const AUTHOR_EMAIL: &'static str = "";
    /// The major version of the operator.
    const MAJOR_VERSION: i32 = 0;
    /// The minor version of the operator.
    const MINOR_VERSION: i32 = 0;
    /// The python version of the operator.
    const PYTHON_VERSION: &'static str = "";
    const PYTHON_METHODS: &'static [pyo3_ffi::PyMethodDef] = &[];
    const PYTHON_GET_SETS: &'static [pyo3_ffi::PyGetSetDef] = &[];
    /// Whether to cook on start.
    const COOK_ON_START: bool = false;

}

pub trait InfoChop {
    fn size(&self) -> usize {
        0
    }

    fn channel(&self, index: usize) -> (String, f32) {
        unimplemented!()
    }
}

impl<T> InfoChop for T {}

pub trait InfoDat {
    fn size(&self) -> (u32, u32) {
        (0, 0)
    }

    fn entry(&self, index: usize, entry_index: usize) -> String {
        String::from("")
    }
}

impl<T> InfoDat for T {}

/// Functionality for all operator plugin types.
/// This can commonly be left as the default implementation for most plugins.
pub trait Op: InfoDat + InfoChop {
    fn set_info(&mut self, info: &str) {
        // # Safety
        // The plugin is held on a single thread, and setters
        // are only ever called from the body of the plugin
        // and not exposed to C++.
        unsafe {
            INFO_STR.get_mut().unwrap().replace_range(.., info);
        }
    }

    fn info(&self) -> &str {
        unsafe { INFO_STR.get_or_init(|| "".to_string()) }
    }

    fn set_error(&mut self, error: &str) {
        // # Safety
        // The plugin is held on a single thread, and setters
        // are only ever called from the body of the plugin
        // and not exposed to C++.
        unsafe {
            ERROR_STR.get_mut().unwrap().replace_range(.., error);
        }
    }

    fn error(&self) -> &str {
        unsafe { ERROR_STR.get_or_init(|| "".to_string()) }
    }

    fn set_warning(&mut self, warning: &str) {
        // # Safety
        // The plugin is held on a single thread, and setters
        // are only ever called from the body of the plugin
        // and not exposed to C++.
        unsafe {
            WARNING_STR.get_mut().unwrap().replace_range(.., warning);
        }
    }

    fn warning(&self) -> &str {
        unsafe { WARNING_STR.get_or_init(|| "".to_string()) }
    }

    fn pulse_pressed(&mut self, name: &str) {}
}

/// Input to an operator, which can be used to get parameters, channels,
/// and other information.
pub struct OperatorInputs<'execute, Op> {
    pub inputs: &'execute cxx::OP_Inputs,
    _marker: std::marker::PhantomData<Op>,
}

impl<'execute, Op> OperatorInputs<'execute, Op>
where
    Self: GetInput<'execute, Op>,
{
    /// Create a new operator input. This is only called by the operator.
    pub fn new(inputs: &'execute crate::cxx::OP_Inputs) -> OperatorInputs<'execute, Op> {
        Self {
            inputs,
            _marker: Default::default(),
        }
    }

    /// Get the parameters for this operator.
    pub fn params(&self) -> ParamInputs {
        ParamInputs::new(self.inputs)
    }

    /// Get an input channel.
    pub fn input(&self, index: usize) -> Option<&<Self as GetInput<'execute, Op>>::Input>
    where
        OperatorInputs<'execute, Op>: GetInput<'execute, Op>,
    {
        GetInput::input(self, index)
    }
}

/// Parameter inputs to an operator.
pub struct ParamInputs<'execute> {
    inputs: &'execute crate::cxx::OP_Inputs,
}

impl<'execute> ParamInputs<'execute> {
    /// Create a new operator input. This is only called by the operator.
    pub fn new(inputs: &'execute crate::cxx::OP_Inputs) -> ParamInputs<'execute> {
        Self { inputs }
    }

    /// Get a float parameter.
    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        unsafe {
            self.inputs
                .getParDouble(ffi::CString::new(name).unwrap().into_raw(), index as i32)
        }
    }

    /// Get an integer parameter.
    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        unsafe {
            self.inputs
                .getParInt(ffi::CString::new(name).unwrap().into_raw(), index as i32)
        }
    }

    /// Get a string parameter.
    pub fn get_string(&self, name: &str) -> &str {
        unsafe {
            let res = self
                .inputs
                .getParString(ffi::CString::new(name).unwrap().into_raw());
            ffi::CStr::from_ptr(res).to_str().unwrap()
        }
    }

    /// Get a toggle parameter.
    pub fn get_toggle(&self, name: &str) -> bool {
        unsafe {
            self.inputs
                .getParInt(ffi::CString::new(name).unwrap().into_raw(), 0)
                != 0
        }
    }

    /// Enable or disable a parameter.
    pub fn enable_param(&self, name: &str, enable: bool) {
        unsafe {
            self.inputs
                .enablePar(ffi::CString::new(name).unwrap().into_raw(), enable);
        }
    }

    /// Get a chop parameter.
    fn get_chop(&self, name: &str) -> ChopParam {
        unsafe {
            let chop = self
                .inputs
                .getParCHOP(ffi::CString::new(name).unwrap().into_raw());
            if chop.is_null() {
                ChopParam { input: None }
            } else {
                ChopParam { input: Some(chop) }
            }
        }
    }
}

/// Get an input to an operator.
pub trait GetInput<'execute, Op>: Index<usize, Output = Self::Input> {
    /// The type of the input.
    type Input = Op;
    /// The number of inputs available.
    fn num_inputs(&self) -> usize;
    /// Get an input.
    fn input(&self, index: usize) -> Option<&Self::Input>;
}

impl<'execute, Op> Index<usize> for OperatorInputs<'execute, Op>
where
    Self: GetInput<'execute, Op>,
{
    type Output = <Self as GetInput<'execute, Op>>::Input;

    fn index(&self, index: usize) -> &Self::Output {
        self.input(index).expect("Invalid input index")
    }
}
