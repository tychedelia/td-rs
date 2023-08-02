use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::ops::DerefMut;
use autocxx::prelude::*;
use autocxx::subclass::*;
use std::pin::Pin;
use std::rc::Rc;
use cxx::let_cxx_string;
use cxx::memory::UniquePtrTarget;
use crate::{Chop, ChopOutput};
use td_rs_base::{OperatorInputs, ParameterManager};

include_cpp! {
    #include "CHOP_CPlusPlusBase.h"
    #include "RustChopPlugin.h"
    safety!(unsafe)
    extern_cpp_type!("TD::OP_ParameterManager", td_rs_base::cxx::OP_ParameterManager)
    extern_cpp_type!("TD::OP_String", td_rs_base::cxx::OP_String)
    extern_cpp_type!("TD::OP_InfoDATSize", td_rs_base::cxx::OP_InfoDATSize)
    extern_cpp_type!("TD::OP_InfoCHOPChan", td_rs_base::cxx::OP_InfoCHOPChan)
    extern_cpp_type!("TD::OP_Inputs", td_rs_base::cxx::OP_Inputs)
    generate_pod!("TD::CHOP_PluginInfo")
    generate_pod!("TD::CHOP_GeneralInfo")
    generate_pod!("TD::CHOP_OutputInfo")
    generate_pod!("TD::CHOP_Output")
}

pub use ffi::*;
pub use ffi::TD::*;
pub use td_rs_base::cxx::setString;

extern "C" {
    fn chop_new_impl() -> Box<dyn Chop>;
}

#[subclass(superclass("RustChopPlugin"))]
pub struct RustChopPluginImpl {
    inner: Box<dyn Chop>,
}

impl Default for RustChopPluginImpl {
    fn default() -> Self {
        unsafe {
            Self {
                inner: chop_new_impl(),
                cpp_peer: Default::default(),
            }
        }
    }
}

#[no_mangle]
extern "C" fn chop_new() -> *mut RustChopPluginImplCpp {
    RustChopPluginImpl::default_cpp_owned().into_raw()
}

impl RustChopPlugin_methods for RustChopPluginImpl {
    fn getGeneralInfo(&mut self, mut info: Pin<&mut CHOP_GeneralInfo>, input: &OP_Inputs) {
        let input = OperatorInputs::new(input);
        let gen_info = self.inner.general_info(&input);
        info.cookEveryFrame = gen_info.cook_every_frame;
        info.cookEveryFrameIfAsked = gen_info.cook_every_frame_if_asked;
        info.timeslice = gen_info.timeslice;
        info.inputMatchIndex = gen_info.input_match_index;
    }

    fn getOutputInfo(&mut self, mut info: Pin<&mut CHOP_OutputInfo>, input: &OP_Inputs) -> bool {
        let input = OperatorInputs::new(input);
        if let Some(mut params) = self.inner.params_mut() {
            params.update(&input.params());
        }
        let out_info = self.inner.output_info(&input);
        if let Some(out_info) = out_info {
            info.numChannels = out_info.num_channels as i32;
            info.sampleRate = out_info.sample_rate;
            info.numSamples = out_info.num_samples as i32;
            info.startIndex = out_info.start_index as u32;
            true
        } else {
            false
        }
    }

    fn getChannelName(&mut self, index: i32, name: Pin<&mut OP_String>, input: &OP_Inputs) {
        let input = OperatorInputs::new(input);
        let chan_name = self.inner.channel_name(index as usize, &input);
        unsafe {
            let new_string = CString::new(chan_name.as_str()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            name.setString(new_string_ptr);
        }
    }


    fn execute(&mut self, output: Pin<&mut CHOP_Output>, input: &OP_Inputs) {
        let input = OperatorInputs::new(input);
        let mut output = ChopOutput::new(output);
        if let Some(mut params) = self.inner.params_mut() {
            params.update(&input.params());
        }
        self.inner.execute(&mut output, &input);
    }

    fn getNumInfoCHOPChans(&mut self) -> i32 {
        self.inner.num_info_chop_chans() as i32
    }

    fn getInfoCHOPChan(&mut self, index: i32, name: Pin<&mut OP_String>, mut value: Pin<&mut f32>) {
        let (info_name, info_value) = self.inner.info_chop_chan(index as usize);
        unsafe {
            let new_string = CString::new(info_name.as_str()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            name.setString(new_string_ptr);
        }
        value.set(info_value);
    }

    fn getInfoDATSize(&mut self, mut info: Pin<&mut OP_InfoDATSize>) -> bool {
        let (rows, cols) = self.inner.info_dat_size();
        if rows == 0 && cols == 0 {
            false
        } else {
            info.rows = rows as i32;
            info.cols = cols as i32;
            true
        }
    }

    fn getInfoDATEntry(&mut self, index: i32, entryIndex: i32, entry: Pin<&mut OP_String>) {
        let entry_str = self.inner.info_dat_entry(index as usize, entryIndex as usize);
        if entry_str.is_empty() {
            return;
        }
        unsafe {
            let new_string = CString::new(entry_str.as_str()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            entry.setString(new_string_ptr);
        }
    }

    fn getWarningString(&mut self, warning: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.warning()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            warning.setString(new_string_ptr);
        }
    }

    fn getErrorString(&mut self, error: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.error()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            error.setString(new_string_ptr);
        }
    }

    fn getInfoPopupString(&mut self, info: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.info()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            info.setString(new_string_ptr);
        }
    }

    fn setupParameters(&mut self, manager: Pin<&mut OP_ParameterManager>) {
        let params = self.inner.params_mut();
        if let Some(mut params) = params {
            let mut manager = ParameterManager::new(manager);
            params.register(&mut manager);
        }
    }

    unsafe fn pulsePressed(&mut self, name: *const std::ffi::c_char) {
        self.inner.pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }
}