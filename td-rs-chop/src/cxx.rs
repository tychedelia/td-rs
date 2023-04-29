use std::cell::RefCell;
use std::ffi::c_void;
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
    generate_pod!("CHOP_Output")
}

extern "C" {
    fn chop_get_plugin_info_impl(plugin_info: Pin<&mut CHOP_PluginInfo>);
    fn chop_new_impl() -> Box<dyn Chop>;
}

pub use ffi::*;
use td_rs_base::OperatorInput;

#[subclass(superclass("RustChopPlugin"))]
pub struct RustChopPluginImpl {
    inner: Box<dyn Chop>,
}

impl Default for RustChopPluginImpl {
    fn default() -> Self {
        unsafe {
            Self {
                inner: chop_new_impl(),
                ..Default::default()
            }
        }
    }
}
//
// impl RustChopPluginImpl {
//     fn new() -> UniquePtr<Self> {
//         unsafe {
//             Self::new_cpp_owned(Self {
//                 inner: chop_new_impl(),
//                 cpp_peer: Default::default(),
//             })
//         }
//     }
// }

#[autocxx::extern_rust::extern_rust_function]
pub fn chop_get_plugin_info(chop_info: Pin<&mut CHOP_PluginInfo>) {
    unsafe {
        chop_get_plugin_info_impl(chop_info)
    }
}

#[no_mangle]
extern "C" fn chop_new() -> UniquePtr<RustChopPluginImplCpp> {
    RustChopPluginImpl::default_cpp_owned()
}

impl RustChopPlugin_methods for RustChopPluginImpl {
    fn getGeneralInfo1(&mut self, _info: Pin<&mut CHOP_GeneralInfo>, _input: &OP_Inputs) {}

    fn getOutputInfo1(&mut self, info: Pin<&mut CHOP_OutputInfo>, _input: &OP_Inputs) -> bool{
        true
    }

    fn getChannelName1(&mut self, index: i32, name: Pin<&mut OP_String>, _input: &OP_Inputs) {
        // unimplemented!()
    }


    fn execute1(&mut self, output: Pin<&mut CHOP_Output>, input: &OP_Inputs) {
        println!("{}", output.numSamples);
    }

    fn getNumInfoCHOPChans1(&mut self) -> i32 {
        0
    }

    fn getInfoCHOPChan1(&mut self, _index: i32, _info: Pin<&mut OP_InfoCHOPChan>) {}

    fn getInfoDATSize1(&mut self, _info: Pin<&mut OP_InfoDATSize>) -> bool {
        false
    }

    fn getInfoDATEntries1(&mut self, index: i32, nEntries: i32, entries: Pin<&mut OP_InfoDATEntries>) {

    }

    fn getWarningString1(&mut self, warning: Pin<&mut OP_String>) {
        // warning.setString(self.inner.warning_string());
    }

    fn getErrorString1(&mut self, error: Pin<&mut OP_String>) {
        // error.setString(self.inner.error_string());
    }

    fn getInfoPopupString1(&mut self, info: Pin<&mut OP_String>) {
        // info.setString(self.inner.info_popup_string());
    }

    fn setupParameters1(&mut self, _parameters: Pin<&mut OP_ParameterManager>) {

    }

    unsafe fn pulsePressed1(&mut self, name: *const std::ffi::c_char) {
        // self.inner.pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }
}