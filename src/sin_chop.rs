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
        todo!()
    }

    fn get_params(&mut self) -> ChopParams {
        todo!()
    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        todo!()
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
        todo!()
    }
}