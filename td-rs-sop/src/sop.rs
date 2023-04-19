pub use crate::cxx::ffi::*;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_base::operator_input::OperatorInput;
use td_rs_base::{OperatorParams, ParameterManager};

/// Trait for defining metadata for a sop operator.
pub trait SopInfo {
    /// The type of the operator.
    const OPERATOR_TYPE: &'static str = "";
    /// The label of the operator.
    const OPERATOR_LABEL: &'static str = "";
    /// The icon of the operator.
    const OPERATOR_ICON: &'static str = "";
    /// The minimum number of inputs the operator accepts.
    const MIN_INPUTS: i32 = 0;
    /// The maximum number of inputs the operator accepts.
    const MAX_INPUTS: i32 = 0;
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

/// Trait for defining a custom operator.
pub trait Sop {
    /// Called on plugin init to declare parameters required for plugin.
    fn get_params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    /// Execute the chop, filling the output channels.
    fn execute(&mut self, output: &mut SopOutput, input: &SopOperatorInput);

    /// Execute the chop, filling the output channels.
    fn execute_vbo(&mut self, output: &mut SopOutput, input: &SopOperatorInput);
    
    /// Called on plugin init for the chop's general info.
    fn get_general_info(&self) -> SopGeneralInfo {
        SopGeneralInfo::default()
    }


    /// Called each cook to provide a warning (or blank).
    fn get_warning(&self) -> String {
        "".to_string()
    }

}
