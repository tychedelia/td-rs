use crate::{ChopOperatorInputs, ChopOutput, ChopOutputInfo, ChopParams, StringParameter};

pub trait Chop {
    fn on_reset(&mut self);
    fn get_params(&mut self) -> ChopParams;
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool;
    fn get_channel_name(&self, index: i32, inputs: &ChopOperatorInputs) -> String;
    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs);
}