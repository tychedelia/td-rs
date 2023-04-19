pub mod sop;
pub mod cxx;
pub use sop::*;
pub use td_rs_base::{OperatorInput, OperatorParams, Param, ParamOptions, ParameterManager};

mod param {
    pub use td_rs_base::{File, Folder};
}

#[macro_export]
macro_rules! sop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_sop::cxx::ffi::OperatorInfo;

        #[no_mangle]
        pub extern "C" fn sop_get_operator_info_impl() -> OperatorInfo {
            OperatorInfo {

            }
        }

        #[no_mangle]
        pub extern "C" fn sop_new_impl() -> Box<dyn Sop> {
            Box::new(<$plugin_ty>::new())
        }
    };
}
