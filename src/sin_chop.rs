use crate::Chop;
use crate::ffi::{ChopOperatorInputs, ChopOutput, ChopOutputInfo, ChopParams};

pub struct SinChop {

}

impl SinChop {
    pub(crate) fn new() -> Self {
        Self { }
    }
}

impl Chop for SinChop {
    fn on_reset(&mut self) {
    }

    fn get_params(&self) -> ChopParams {
        ChopParams {
            numeric_params: vec![],
            string_params: vec![]
        }
    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        false
    }

    fn get_channel_name(&self, index: i32, inputs: &ChopOperatorInputs) -> String {
        format!("chan{}", index)
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
    }
}