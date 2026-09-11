#![allow(non_snake_case)]
#![allow(ambiguous_glob_reexports)]

use autocxx::prelude::*;
use autocxx::subclass::*;
use std::ffi::CString;
use std::pin::Pin;
use td_rs_base::{param::ParameterManager, DynamicMenuInfo, NodeInfo, OperatorInputs};

use crate::{Pop, PopContext, PopOutput};

include_cpp! {
    #include "POP_CPlusPlusBase.h"
    #include "RustPopPlugin.h"
    safety!(unsafe)
    extern_cpp_type!("TD::OP_ParameterManager", td_rs_base::cxx::OP_ParameterManager)
    extern_cpp_type!("TD::OP_String", td_rs_base::cxx::OP_String)
    extern_cpp_type!("TD::OP_InfoDATSize", td_rs_base::cxx::OP_InfoDATSize)
    extern_cpp_type!("TD::OP_InfoCHOPChan", td_rs_base::cxx::OP_InfoCHOPChan)
    extern_cpp_type!("TD::OP_Inputs", td_rs_base::cxx::OP_Inputs)
    extern_cpp_type!("TD::OP_CustomOPInfo", td_rs_base::cxx::OP_CustomOPInfo)
    pod!("TD::OP_CustomOPInfo")
    extern_cpp_type!("TD::OP_BuildDynamicMenuInfo", td_rs_base::cxx::OP_BuildDynamicMenuInfo)
    generate_pod!("TD::POP_GeneralInfo")
    generate!("TD::POP_Output")
    generate!("TD::POP_Context")
    generate!("TD::POP_Buffer")

    generate!("popCreateBuffer")
    generate!("popBufferData")
    generate!("popBufferSize")
    generate!("popReleaseBuffer")
    generate!("popSetAttribute")
    generate!("popSetIndexBuffer")
    generate!("popSetInfoBuffers")
    generate!("popPointInfoSize")
    generate!("popTopologyInfoSize")
    generate!("popWritePointInfo")
    generate!("popWriteTopologyInfo")
}

pub use autocxx::c_void;
pub use ffi::TD::*;
pub use ffi::*;
pub use td_rs_base::cxx::*;

pub use ffi::TD_OP_SmartRef_TD_POP_Buffer_AutocxxConcrete;

extern "C" {
    // SAFETY: `pop_new_impl` is only ever called from Rust compiled
    // at the same time as the plugin, so the types are guaranteed to
    // match
    #[allow(improper_ctypes)]
    fn pop_new_impl(info: NodeInfo, context: PopContext) -> Box<dyn Pop>;
}

#[subclass(superclass("RustPopPlugin"))]
pub struct RustPopPluginImpl {
    inner: Box<dyn Pop>,
}

// SAFETY: This can only be used with pointers returned from getNodeInstance() and
// should not be used in plugin code.
pub unsafe fn plugin_cast(plugin: *mut c_void) -> &'static mut RustPopPluginImplCpp {
    &mut *(plugin as *mut RustPopPluginImplCpp)
}

impl AsPlugin for RustPopPluginImplCpp {
    type Plugin = RustPopPlugin;

    fn as_plugin(&self) -> &Self::Plugin {
        self.As_RustPopPlugin()
    }

    fn as_plugin_mut(&mut self) -> Pin<&mut Self::Plugin> {
        // Safety: self can't be moved during the lifetime of 'cook.
        unsafe { Pin::new_unchecked(self).As_RustPopPlugin_mut() }
    }
}

#[no_mangle]
extern "C" fn pop_new(
    info: &'static OP_NodeInfo,
    context: Pin<&'static mut POP_Context>,
) -> *mut RustPopPluginImplCpp {
    unsafe {
        let info = NodeInfo::new(info);
        let context = PopContext::new(context);
        RustPopPluginImpl::new_cpp_owned(RustPopPluginImpl {
            inner: pop_new_impl(info, context),
            cpp_peer: CppSubclassCppPeerHolder::Empty,
        })
        .into_raw()
    }
}

impl RustPopPlugin_methods for RustPopPluginImpl {
    fn inner(&self) -> *mut c_void {
        self.inner.as_ref() as *const dyn Pop as *mut c_void
    }

    fn innerMut(&mut self) -> *mut c_void {
        self.inner.as_mut() as *mut dyn Pop as *mut c_void
    }

    fn getGeneralInfo(&mut self, mut info: Pin<&mut POP_GeneralInfo>, inputs: &OP_Inputs) {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("getGeneralInfo").entered() };
        let input = OperatorInputs::new(inputs);
        if let Some(params) = self.inner.params_mut() {
            params.update(&input.params());
        }
        let gen_info = self.inner.general_info(&input);
        info.cookEveryFrame = gen_info.cook_every_frame;
        info.cookEveryFrameIfAsked = gen_info.cook_every_frame_if_asked;
    }

    fn execute(&mut self, output: Pin<&mut POP_Output>, inputs: &OP_Inputs) {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("execute").entered() };
        let input = OperatorInputs::new(inputs);
        let mut output = PopOutput::new(output);
        if let Some(params) = self.inner.params_mut() {
            params.update(&input.params());
        }
        self.inner.execute(&mut output, &input);
    }

    fn getNumInfoCHOPChans(&mut self) -> i32 {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("getNumInfoCHOPChans").entered() };
        if let Some(info_chop) = self.inner.info_chop() {
            info_chop.size() as i32
        } else {
            0
        }
    }

    fn getInfoCHOPChan(&mut self, index: i32, name: Pin<&mut OP_String>, mut value: Pin<&mut f32>) {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("getInfoCHOPChan").entered() };
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
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("getInfoDATSize").entered() };
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
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("getInfoDATEntry").entered() };
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
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("setupParameters").entered() };
        let params = self.inner.params_mut();
        if let Some(params) = params {
            let mut manager = ParameterManager::new(manager);
            params.register(&mut manager);
        }
    }

    unsafe fn pulsePressed(&mut self, name: *const std::ffi::c_char) {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("pulsePressed").entered() };
        self.inner
            .pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }

    fn buildDynamicMenu(&mut self, inputs: &OP_Inputs, info: Pin<&mut OP_BuildDynamicMenuInfo>) {
        #[cfg(feature = "tracing")]
        let _span = { tracing_base::trace_span!("buildDynamicMenu").entered() };
        let input = OperatorInputs::new(inputs);
        let mut info = DynamicMenuInfo::new(info);
        self.inner.build_dynamic_menu(&input, &mut info);
    }
}
