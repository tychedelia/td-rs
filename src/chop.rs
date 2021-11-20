use crate::{ChopOperatorInputs, ChopOutput, ChopOutputInfo, ChopParams};

pub trait Chop {
    fn on_reset(&mut self);
    fn get_params(&mut self) -> ChopParams;
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool;
    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs);
}