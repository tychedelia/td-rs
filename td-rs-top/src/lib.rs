#![feature(min_specialization)]
pub mod cxx;
pub mod mode;

use mode::cpu::{CpuMemPixelType, TopCpuInput, TopCpuOutput};
use std::pin::Pin;
pub use td_rs_base::top::*;
pub use td_rs_base::*;

pub trait TopInfo {
    const EXECUTE_MODE: ExecuteMode;
}

pub struct TopOutputSpecs<'execute> {
    specs: Pin<&'execute mut cxx::TOP_OutputFormatSpecs>,
}

impl<'execute> TopOutputSpecs<'execute> {
    pub fn new(specs: Pin<&'execute mut cxx::TOP_OutputFormatSpecs>) -> TopOutputSpecs {
        TopOutputSpecs { specs }
    }

    pub fn width(&self) -> usize {
        self.specs.width as usize
    }

    pub fn height(&self) -> usize {
        self.specs.height as usize
    }
}

pub struct TopContext<'execute> {
    context: Pin<&'execute mut cxx::TOP_Context>,
}

impl<'execute> TopContext<'execute> {
    pub fn new(context: Pin<&'execute mut cxx::TOP_Context>) -> TopContext {
        TopContext { context }
    }
}

#[derive(Debug, Default)]
pub struct TopGeneralInfo {
    pub cook_every_frame: bool,
    pub clear_buffers: bool,
    pub mipmap_all_tops: bool,
    pub cook_every_frame_if_asked: bool,
    pub input_size_index: i32,
    pub mem_pixel_type: CpuMemPixelType,
}

pub enum ExecuteMode {
    OpenGL,
    Cuda,
    CpuWrite,
    CpuReadWrite,
}

pub trait TopExecute {
    fn execute_opengl(
        &mut self,
        input: &OperatorInputs<TopInput>,
        output: TopOutputSpecs,
        context: TopContext,
    ) {
        unimplemented!("OpenGL execution not implemented for this operator")
    }

    fn execute_cuda(
        &mut self,
        input: &OperatorInputs<TopInput>,
        output: TopOutputSpecs,
        context: TopContext,
    ) {
        unimplemented!("CUDA execution not implemented for this operator")
    }

    fn execute_cpu(&mut self, input: &OperatorInputs<TopCpuInput>, output: TopCpuOutput) {
        unimplemented!("CPU execution not implemented for this operator")
    }
}

pub trait Top: Op + TopExecute {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn general_info(&self, input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo::default()
    }
}

#[macro_export]
macro_rules! top_plugin {
    ($plugin_ty:ty) => {
        use td_rs_top::cxx::OP_CustomOPInfo;

        #[no_mangle]
        pub extern "C" fn top_get_plugin_info_impl(
            mut op_info: Pin<&mut OP_CustomOPInfo>,
        ) -> cxx::TOP_ExecuteMode {
            unsafe {
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_TYPE).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.opType, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_LABEL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.opLabel, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_ICON).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.opIcon, new_string_ptr);
                op_info.minInputs = <$plugin_ty>::MIN_INPUTS as i32;
                op_info.maxInputs = <$plugin_ty>::MAX_INPUTS as i32;
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_NAME).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.authorName, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_EMAIL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.authorEmail, new_string_ptr);
                op_info.majorVersion = <$plugin_ty>::MAJOR_VERSION;
                op_info.minorVersion = <$plugin_ty>::MINOR_VERSION;
                let new_string = std::ffi::CString::new(<$plugin_ty>::PYTHON_VERSION).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_top::cxx::setString(op_info.pythonVersion, new_string_ptr);
                op_info.cookOnStart = <$plugin_ty>::COOK_ON_START;
                match <$plugin_ty>::EXECUTE_MODE {
                    td_rs_top::ExecuteMode::OpenGL => cxx::TOP_ExecuteMode::OpenGL_FBO,
                    td_rs_top::ExecuteMode::Cuda => cxx::TOP_ExecuteMode::CUDA,
                    td_rs_top::ExecuteMode::CpuWrite => cxx::TOP_ExecuteMode::CPUMemWriteOnly,
                    td_rs_top::ExecuteMode::CpuReadWrite => cxx::TOP_ExecuteMode::CPUMemReadWrite,
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn top_get_execute_mode_impl() -> ExecuteMode {
            <$plugin_ty>::EXECUTE_MODE
        }

        #[no_mangle]
        pub extern "C" fn top_new_impl() -> Box<dyn Top> {
            Box::new(<$plugin_ty>::new())
        }
    };
}
