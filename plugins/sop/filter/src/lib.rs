use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_sop::*;
use td_rs_derive::Params;

pub struct Filter {

}

impl Filter {
    pub fn new() -> Self {
        Self {}
    }
}

impl SopInfo for Filter {
    const OPERATOR_LABEL: &'static str = "Filter";
}

impl Sop for Filter {
    fn execute(&mut self, output: &mut td_rs_sop::cxx::ffi::SopOutput, input: &td_rs_sop::cxx::ffi::SopOperatorInput) {
        todo!()
    }

    fn execute_vbo(&mut self, output: &mut td_rs_sop::cxx::ffi::SopOutput, input: &td_rs_sop::cxx::ffi::SopOperatorInput) {
        todo!()
    }

    fn get_general_info(&self) -> SopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            timeslice: false,
            input_match_index: 0,
        }
    }
}

chop_plugin!(SinChop);
