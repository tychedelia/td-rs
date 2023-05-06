use crate::{ChopInfoChan, ChopOperatorInputs, ChopOutput, ChopOutputInfo, ChopParams, StringParameter};

pub trait Chop {
    fn on_reset(&mut self);
    fn get_params(&mut self) -> ChopParams;
    fn get_num_info_chop_chans(&self) -> i32 {
        0
    }
    fn get_info_chop_chan(&self, index: i32) -> ChopInfoChan {
        // must be implemented if getNumInfoCHOPChans is called with > 0
        unimplemented!()
    }
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool;
    fn get_channel_name(&self, index: i32, inputs: &ChopOperatorInputs) -> String;
    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs);
}