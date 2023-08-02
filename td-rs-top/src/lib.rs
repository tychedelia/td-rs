pub mod cxx;

use std::pin::Pin;
pub use td_rs_base::top::*;
pub use td_rs_base::*;

pub struct TopOutput<'execute> {
    output: Pin<&'execute mut cxx::TOP_Output>,
}

impl<'execute> TopOutput<'execute> {
    pub fn new(output: Pin<&'execute mut cxx::TOP_Output>) -> TopOutput<'execute> {
        Self { output }
    }
}

pub trait TopInfo {
    const EXECUTE_MODE: ExecuteMode;
}

#[derive(Debug, Default)]
pub struct TopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
    pub input_size_index: i32,
}

pub enum ExecuteMode {
    Cpu,
    Cuda,
}

pub trait Top: Op {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn general_info(&self, input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo::default()
    }

    fn execute(&mut self, output: TopOutput, input: &OperatorInputs<TopInput>) {}
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
                    td_rs_top::ExecuteMode::Cuda => cxx::TOP_ExecuteMode::CUDA,
                    td_rs_top::ExecuteMode::Cpu => cxx::TOP_ExecuteMode::CPUMem,
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn top_new_impl() -> Box<dyn Top> {
            Box::new(<$plugin_ty>::new())
        }
    };
}
