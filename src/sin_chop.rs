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

    fn get_params(&mut self) -> ChopParams {
        ChopParams {
            params: vec![]
        }
    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        false
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
    }
}