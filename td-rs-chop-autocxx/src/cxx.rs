use std::cell::RefCell;
use autocxx::prelude::*;
use autocxx::subclass::*;
use std::pin::Pin;
use std::rc::Rc;
use crate::Chop;

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

impl Default for Box<dyn Chop> {
    fn default() -> Self {
        unsafe {
            chop_new_impl()
        }
    }
}

#[subclass(superclass("RustChopPlugin"))]
#[derive(Default)]
pub struct ChopImpl {
    inner: Box<dyn Chop>
}

impl RustChopPlugin_methods for ChopImpl {
    unsafe fn setupParameters(&mut self, manager: *mut ffi::OP_ParameterManager, reserved: *mut c_void) {

    }

    unsafe fn execute(&mut self, outputs: *mut CHOP_Output, inputs: *const ffi::OP_Inputs, reserved: *mut c_void) {

    }
}