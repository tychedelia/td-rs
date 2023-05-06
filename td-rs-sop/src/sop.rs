pub use crate::cxx::ffi::*;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_base::operator_input::OperatorInput;
use td_rs_base::{OperatorParams, ParameterManager};

/// Trait for defining metadata for a sop operator.
pub trait SopInfo {
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
