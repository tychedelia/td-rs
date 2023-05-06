pub mod cxx;
pub mod chop;
pub use td_rs_param::{Param, ParameterManager, ParamOptions, OperatorInput, OperatorParams};
pub use chop::*;


#[macro_export]
macro_rules! chop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_chop::cxx::ffi::OperatorInfo;

        #[no_mangle]
        pub extern "C" fn chop_get_operator_info_impl() -> OperatorInfo {
            OperatorInfo {
                operator_type: <$plugin_ty>::OPERATOR_TYPE.to_string(),
                operator_label: <$plugin_ty>::OPERATOR_LABEL.to_string(),
                operator_icon: <$plugin_ty>::OPERATOR_ICON.to_string(),
                min_inputs: <$plugin_ty>::MIN_INPUTS,
                max_inputs: <$plugin_ty>::MAX_INPUTS,
                author_name: <$plugin_ty>::AUTHOR_NAME.to_string(),
                author_email: <$plugin_ty>::AUTHOR_EMAIL.to_string(),
                major_version: <$plugin_ty>::MAJOR_VERSION,
                minor_version: <$plugin_ty>::MINOR_VERSION,
                python_version: <$plugin_ty>::PYTHON_VERSION.to_string(),
                cook_on_start: <$plugin_ty>::COOK_ON_START,
            }
        }

        #[no_mangle]
        pub extern "C" fn chop_new_impl() -> Box<dyn Chop> {
            Box::new(<$plugin_ty>::new())
        }
    }
}