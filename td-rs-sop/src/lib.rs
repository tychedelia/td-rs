use std::ops::{Index, IndexMut};
use std::pin::Pin;
use std::sync::Arc;
use autocxx::prelude::*;
pub use td_rs_base::*;

pub mod cxx;

#[derive(Debug, Default)]
pub struct SopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
    pub direct_to_gpu: bool,
}

/// Trait for defining metadata for a sop operator.
pub trait SopInfo {
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
    /// Whether to cook on start.
    const COOK_ON_START: bool = false;
}

pub struct SopOutput<'execute> {
    output: Pin<&'execute mut cxx::SOP_Output>,
}

impl<'execute> SopOutput<'execute> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::SOP_Output>) -> SopOutput<'execute> {
        Self { output }
    }
}

pub struct SopVboOutput<'execute> {
    output: Pin<&'execute mut cxx::SOP_VBOOutput>,
}

impl<'execute> SopVboOutput<'execute> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::SOP_VBOOutput>) -> SopVboOutput<'execute> {
        Self { output }
    }
}

/// Trait for defining a custom operator.
pub trait Sop : Op {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn general_info(&self, input: &OperatorInputs) -> SopGeneralInfo {
        SopGeneralInfo::default()
    }

    fn execute(&mut self, output: &mut SopOutput, input: &OperatorInputs) {
        // Do nothing by default.
    }

    fn execute_vbo(&mut self, output: &mut SopVboOutput, input: &OperatorInputs) {
        // Do nothing by default.
    }
}

#[macro_export]
macro_rules! sop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_sop::cxx::OP_CustomOPInfo;

        #[no_mangle]
        pub extern "C" fn sop_get_plugin_info_impl(mut op_info: Pin<&mut OP_CustomOPInfo>) {
            unsafe {
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_TYPE).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.opType, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_LABEL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.opLabel, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_ICON).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.opIcon, new_string_ptr);
                op_info.minInputs = <$plugin_ty>::MIN_INPUTS as i32;
                op_info.maxInputs = <$plugin_ty>::MAX_INPUTS as i32;
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_NAME).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.authorName, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_EMAIL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.authorEmail, new_string_ptr);
                op_info.majorVersion = <$plugin_ty>::MAJOR_VERSION;
                op_info.minorVersion = <$plugin_ty>::MINOR_VERSION;
                let new_string = std::ffi::CString::new(<$plugin_ty>::PYTHON_VERSION).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_sop::cxx::setString(op_info.pythonVersion, new_string_ptr);
                op_info.cookOnStart = <$plugin_ty>::COOK_ON_START;
            }
        }

        #[no_mangle]
        pub extern "C" fn sop_new_impl() -> Box<dyn Sop> {
            Box::new(<$plugin_ty>::new())
        }
    };
}