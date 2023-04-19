use crate::sop;
use crate::sop::Sop;
use crate::cxx::ffi::*;
use cxx::ExternType;
use std::pin::Pin;
use td_rs_base::cxx::ffi::OperatorInput;
use td_rs_base::cxx::ffi::ParameterManager;

unsafe impl ExternType for Box<dyn Sop> {
    type Id = cxx::type_id!("BoxDynSop");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct PtrBoxDynSop(*mut Box<dyn Sop>);

unsafe impl ExternType for PtrBoxDynSop {
    type Id = cxx::type_id!("PtrBoxDynSop");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug, Default)]
    pub struct OperatorInfo {
        pub operator_type: String,
        pub operator_label: String,
        pub operator_icon: String,
        pub min_inputs: i32,
        pub max_inputs: i32,
        pub author_name: String,
        pub author_email: String,
        pub major_version: i32,
        pub minor_version: i32,
        pub python_version: String,
        pub cook_on_start: bool,
    }

    #[derive(Debug, Default)]
    pub struct SopGeneralInfo {
        pub cook_every_frame: bool,
        pub cook_every_frame_if_asked: bool,
        pub direct_to_gpu: bool,
    }

    #[namespace = "td_rs_base::ffi"]
    extern "C++" {
        include!("parameter_manager/ParameterManager.h");
        type ParameterManager = td_rs_base::cxx::ffi::ParameterManager;
    }

    #[namespace = "td_rs_base::ffi"]
    extern "C++" {
        include!("operator_input/OperatorInput.h");
        type OperatorInput = td_rs_base::cxx::ffi::OperatorInput;
    }

    extern "C++" {
        include!("BoxDynSop.h");
        type BoxDynSop = Box<dyn crate::sop::Sop>;
        type PtrBoxDynSop = crate::cxx::PtrBoxDynSop;
    }

    unsafe extern "C++" {
        include!("SopOperatorInput.h");
        pub(crate) type SopOperatorInput;
        // pub fn getInput(&self, idx: usize) -> UniquePtr<sopInput>;
    }

    unsafe extern "C++" {
        include!("SopInput.h");
        pub(crate) type SopInput;
        // pub fn getPath(&self) -> &str;
        // pub fn getId(&self) -> u32;
        // pub fn getNumChannels(&self) -> i32;
        // pub fn getNumSamples(&self) -> i32;
        // pub fn getSampleRate(&self) -> f64;
        // pub fn getStartIndex(&self) -> f64;
        // pub fn getChannelNames(&self) -> &[&str];
        // pub fn getChannels(&self) -> &[&[f32]];
        // pub fn getTotalCooks(&self) -> i64;
    }

    unsafe extern "C++" {
        include!("SopOutput.h");
        pub(crate) type SopOutput;
        // pub fn getNumChannels(&self) -> i32;
        // pub fn getNumSamples(&self) -> i32;
        // pub fn getSampleRate(&self) -> i32;
        // pub fn getStartIndex(&self) -> usize;
        // pub fn getChannelNames(&self) -> &[&str];
        // pub fn getChannels(self: Pin<&mut sopOutput>) -> &mut [&mut [f32]];
    }

    extern "Rust" {
        unsafe fn dyn_sop_drop_in_place(ptr: PtrBoxDynSop);
        fn sop_setup_params(sop: &mut BoxDynSop, manager: Pin<&mut ParameterManager>);
        fn sop_execute(
            sop: &mut BoxDynSop,
            output: Pin<&mut SopOutput>,
            input: Pin<&OperatorInput>,
            sop_input: Pin<&SopOperatorInput>,
        );
        fn sop_execute_vbo(
            sop: &mut BoxDynSop,
            output: Pin<&mut SopOutput>,
            input: Pin<&OperatorInput>,
            sop_input: Pin<&SopOperatorInput>,
        );
        fn sop_get_general_info(sop: &BoxDynSop) -> SopGeneralInfo;
        fn sop_get_warning(sop: &BoxDynSop) -> String;
        fn sop_get_operator_info() -> OperatorInfo;
        fn sop_new() -> BoxDynSop;
    }
}

// FFI
fn sop_setup_params(sop: &mut Box<dyn Sop>, manager: Pin<&mut ParameterManager>) {
    let params = (**sop).get_params_mut();
    if let Some(mut params) = params {
        let mut mgr = td_rs_base::parameter_manager::ParameterManager::new(manager);
        params.register(&mut mgr);
    }
}

fn sop_execute(
    sop: &mut Box<dyn Sop>,
    output: Pin<&mut SopOutput>,
    input: Pin<&OperatorInput>,
    sop_input: Pin<&SopOperatorInput>,
) {
    // let mut input = td_rs_base::operator_input::OperatorInput::new(input);
    // let params = (**sop).get_params_mut();
    // if let Some(mut params) = params {
    //     params.update(&input);
    // }
    // let sop_input = crate::sop::SopOperatorInput::new(sop_input);
    // (**sop).execute(&mut sop::SopOutput::new(output), &sop_input);
}
fn sop_execute_vbo(
    sop: &mut Box<dyn Sop>,
    output: Pin<&mut SopOutput>,
    input: Pin<&OperatorInput>,
    sop_input: Pin<&SopOperatorInput>,
) {
    // let mut input = td_rs_base::operator_input::OperatorInput::new(input);
    // let params = (**sop).get_params_mut();
    // if let Some(mut params) = params {
    //     params.update(&input);
    // }
    // // let sop_input = crate::sop::SopOperatorInput::new(sop_input);
    // (**sop).execute(&mut sop::SopOutput::new(output), &sop_input);
}

fn sop_get_general_info(sop: &BoxDynSop) -> SopGeneralInfo {
    (**sop).get_general_info()
}

fn sop_get_warning(sop: &BoxDynSop) -> String {
    (**sop).get_warning()
}

unsafe fn dyn_sop_drop_in_place(ptr: PtrBoxDynSop) {
    std::ptr::drop_in_place(ptr.0);
}

extern "C" {
    fn sop_get_operator_info_impl() -> OperatorInfo;
    fn sop_new_impl() -> Box<dyn Sop>;
}

fn sop_get_operator_info() -> OperatorInfo {
    unsafe { sop_get_operator_info_impl() }
}

fn sop_new() -> Box<dyn Sop> {
    unsafe { sop_new_impl() }
}
