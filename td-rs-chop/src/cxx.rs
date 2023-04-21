use std::cell::RefCell;
use autocxx::prelude::*;
use autocxx::subclass::*;
use std::pin::Pin;
use std::rc::Rc;
use crate::{Chop, OperatorInfo};

include_cpp! {
    #include "CHOP_CPlusPlusBase.h"
    #include "RustChopPlugin.h"
    safety!(unsafe)
    generate_pod!("CHOP_PluginInfo")
    generate_pod!("CHOP_GeneralInfo")
    generate_pod!("CHOP_OutputInfo")
    generate!("CHOP_Output")
}

extern "C" {
    fn chop_get_plugin_info_impl() -> CHOP_PluginInfo;
    fn chop_new_impl() -> Box<dyn Chop>;
}

pub use ffi::*;
use td_rs_base::OperatorInput;

impl Default for Box<dyn Chop> {
    fn default() -> Self {
        unsafe {
            chop_new_impl()
        }
    }
}

#[subclass(superclass("RustChopPlugin"))]
#[derive(Default)]
pub struct RustChopPluginImpl {
    inner: Box<dyn Chop>
}

#[autocxx::extern_rust::extern_rust_function]
pub fn chop_get_plugin_info() -> CHOP_PluginInfo {
    unsafe {
        chop_get_plugin_info_impl()
    }
}

impl RustChopPlugin_methods for RustChopPluginImpl {
    unsafe fn execute(&mut self, output: *mut CHOP_Output, inputs: *const OP_Inputs, reserved1: *mut c_void) {
        println!("execute!");
    }
}