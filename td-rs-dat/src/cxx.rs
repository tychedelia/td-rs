#![allow(non_snake_case)]
use crate::{Dat, DatOutput};
use autocxx::prelude::*;
use autocxx::subclass::*;
use cxx::memory::UniquePtrTarget;
use std::ffi::CString;
use std::ops::DerefMut;
use std::pin::Pin;
use td_rs_base::{param::ParameterManager, OperatorInputs, InfoDat, InfoChop};

include_cpp! {
    #include "DAT_CPlusPlusBase.h"
    #include "RustDatPlugin.h"
    safety!(unsafe)
    extern_cpp_type!("TD::OP_ParameterManager", td_rs_base::cxx::OP_ParameterManager)
    extern_cpp_type!("TD::OP_String", td_rs_base::cxx::OP_String)
    extern_cpp_type!("TD::OP_InfoDATSize", td_rs_base::cxx::OP_InfoDATSize)
    extern_cpp_type!("TD::OP_InfoCHOPChan", td_rs_base::cxx::OP_InfoCHOPChan)
    extern_cpp_type!("TD::OP_Inputs", td_rs_base::cxx::OP_Inputs)
    generate_pod!("TD::OP_CustomOPInfo")
    generate!("TD::DAT_Output")
    generate_pod!("TD::DAT_GeneralInfo")
}

pub use autocxx::c_void;
pub use ffi::TD::*;
pub use ffi::*;
pub use td_rs_base::cxx::getPyContext;
pub use td_rs_base::cxx::setString;
pub use td_rs_base::cxx::PY_GetInfo;
pub use td_rs_base::cxx::PY_Struct;
pub use td_rs_base::cxx::OP_CustomOPInfo;

extern "C" {
    fn dat_new_impl() -> Box<dyn Dat>;
}

#[subclass(superclass("RustDatPlugin"))]
pub struct RustDatPluginImpl {
    inner: Box<dyn Dat>,
}

impl Default for RustDatPluginImpl {
    fn default() -> Self {
        unsafe {
            Self {
                inner: dat_new_impl(),
                cpp_peer: Default::default(),
            }
        }
    }
}

#[no_mangle]
extern "C" fn dat_new() -> *mut RustDatPluginImplCpp {
    RustDatPluginImpl::default_cpp_owned().into_raw()
}

impl RustDatPlugin_methods for RustDatPluginImpl {
    fn getGeneralInfo(&mut self, mut info: Pin<&mut DAT_GeneralInfo>, inputs: &OP_Inputs) {
        let input = OperatorInputs::new(inputs);
        let gen_info = self.inner.general_info(&input);
        info.cookEveryFrame = gen_info.cook_every_frame;
        info.cookEveryFrameIfAsked = gen_info.cook_every_frame_if_asked;
    }

    fn execute(&mut self, outputs: Pin<&mut DAT_Output>, inputs: &OP_Inputs) {
        let input = OperatorInputs::new(inputs);
        let mut output = DatOutput::new(outputs);
        if let Some(mut params) = self.inner.params_mut() {
            params.update(&input.params());
        }
        self.inner.execute(output, &input);
    }

    fn getNumInfoCHOPChans(&mut self) -> i32 {
        if let Some(info_chop) = self.inner.info_chop() {
            info_chop.size() as i32
        } else {
            0
        }
    }

    fn getInfoCHOPChan(&mut self, index: i32, name: Pin<&mut OP_String>, mut value: Pin<&mut f32>) {
        if let Some(info_chop) = self.inner.info_chop() {
            let (info_name, info_value) = info_chop.channel(index as usize);
            unsafe {
                let new_string = CString::new(info_name.as_str()).unwrap();
                let new_string_ptr = new_string.as_ptr();
                name.setString(new_string_ptr);
            }
            value.set(info_value);
        }
    }

    fn getInfoDATSize(&mut self, mut info: Pin<&mut OP_InfoDATSize>) -> bool {
        if let Some(info_dat) = self.inner.info_dat() {
            let (rows, cols) = info_dat.size();
            info.rows = rows as i32;
            info.cols = cols as i32;
            true
        } else {
            false
        }
    }

    fn getInfoDATEntry(&mut self, index: i32, entryIndex: i32, entry: Pin<&mut OP_String>) {
        if let Some(info_dat) = self.inner.info_dat() {
            let entry_str = info_dat.entry(index as usize, entryIndex as usize);
            if entry_str.is_empty() {
                return;
            }
            unsafe {
                let new_string = CString::new(entry_str.as_str()).unwrap();
                let new_string_ptr = new_string.as_ptr();
                entry.setString(new_string_ptr);
            }
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
        self.inner
            .pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }
}
