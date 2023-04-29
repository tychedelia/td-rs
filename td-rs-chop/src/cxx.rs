use std::cell::RefCell;
use std::ffi::c_void;
use autocxx::prelude::*;
use autocxx::subclass::*;
use std::pin::Pin;
use std::rc::Rc;
use cxx::memory::UniquePtrTarget;
use crate::{Chop, OperatorInfo};

include_cpp! {
    #include "CHOP_CPlusPlusBase.h"
    #include "RustChopPlugin.h"
    safety!(unsafe)
    // generate!("OP_Inputs")
    // generate_pod!("OP_InfoCHOPChan")
    // generate_pod!("OP_InfoDATSize")
    // generate_pod!("OP_InfoDATEntries")
    // generate!("OP_ParameterManager")
    generate_pod!("CHOP_PluginInfo")
    generate_pod!("CHOP_GeneralInfo")
    generate_pod!("CHOP_OutputInfo")
    generate_pod!("CHOP_Output")
}

extern "C" {
    fn chop_get_plugin_info_impl(plugin_info: Pin<&mut CHOP_PluginInfo>);
    fn chop_new_impl() -> Box<dyn Chop>;
}

pub use ffi::*;
use td_rs_base::OperatorInput;

#[subclass(superclass("RustChopPluginWrapper"))]
pub struct RustChopPluginImpl {
    inner: Box<dyn Chop>,
}

impl Default for RustChopPluginImpl {
    fn default() -> Self {
        println!("RustChopPluginImpl::default");
        unsafe {
            Self {
                inner: chop_new_impl(),
                cpp_peer: Default::default(),
            }
        }
    }
}

#[autocxx::extern_rust::extern_rust_function]
pub fn chop_get_plugin_info(chop_info: Pin<&mut CHOP_PluginInfo>) {
    unsafe {
        chop_get_plugin_info_impl(chop_info)
    }
}

#[no_mangle]
extern "C" fn chop_new() -> *mut RustChopPluginImplCpp {
    println!("chop_new");
    RustChopPluginImpl::default_cpp_owned().into_raw()
}

impl RustChopPluginWrapper_methods for RustChopPluginImpl {
    fn getGeneralInfo(&mut self, _info: Pin<&mut CHOP_GeneralInfo>, _input: &OP_Inputs) {
        println!("getGeneralInfo1");
    }

    fn getOutputInfo(&mut self, info: Pin<&mut CHOP_OutputInfo>, _input: &OP_Inputs) -> bool {
        println!("getOutputInfo1");
        true
    }

    fn getChannelName(&mut self, index: i32, name: Pin<&mut OP_String>, _input: &OP_Inputs) {
        println!("getChannelName1");
    }


    fn execute(&mut self, output: Pin<&mut CHOP_Output>, input: &OP_Inputs) {
        println!("execute1");
    }

    fn getNumInfoCHOPChans(&mut self) -> i32 {
        println!("getNumInfoCHOPChans1");
        666
    }

    fn getInfoCHOPChan(&mut self, _index: i32, _info: Pin<&mut OP_InfoCHOPChan>) {
        println!("getInfoCHOPChan1");
    }

    fn getInfoDATSize(&mut self, _info: Pin<&mut OP_InfoDATSize>) -> bool {
        println!("getInfoDATSize1");
        false
    }

    fn getInfoDATEntries(&mut self, index: i32, nEntries: i32, entries: Pin<&mut OP_InfoDATEntries>) {
        println!("getInfoDATEntries1");
    }

    fn getWarningString(&mut self, warning: Pin<&mut OP_String>) {
        // warning.setString(self.inner.warning_string());
    }

    fn getErrorString(&mut self, error: Pin<&mut OP_String>) {
        // error.setString(self.inner.error_string());
    }

    fn getInfoPopupString(&mut self, info: Pin<&mut OP_String>) {
        // info.setString(self.inner.info_popup_string());
    }

    fn setupParameters(&mut self, _parameters: Pin<&mut OP_ParameterManager>) {
        println!("setupParameters1");
    }

    unsafe fn pulsePressed(&mut self, name: *const std::ffi::c_char) {
        println!("pulsePressed1");
        // self.inner.pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }
}