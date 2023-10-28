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
    fn general_info(&self, input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo::default()
    }

    fn execute(&mut self, output: TopOutput, input: &OperatorInputs<TopInput>) {}
}

#[macro_export]
macro_rules! top_plugin {
    ($plugin_ty:ty) => {
        use td_rs_top::cxx::OP_CustomOPInfo;
        use td_rs_top::cxx::c_void;

        #[no_mangle]
        pub extern "C" fn top_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) -> cxx::TOP_ExecuteMode {
            unsafe {
                td_rs_top::op_info::<$plugin_ty>(op_info);
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
