use std::ops::Index;
use crate::{GetInput, OperatorInputs};
use crate::cxx::OP_CHOPInput;
use ref_cast::RefCast;

/// A chop input.
#[repr(transparent)]
#[derive(RefCast)]
pub struct ChopInput {
    input: OP_CHOPInput,
}

impl ChopInput {
    /// Get the number of channels in this input.
    pub fn num_channels(&self) -> usize {
        self.input.numChannels as usize
    }

    /// Get the number of samples in this input.
    pub fn num_samples(&self) -> usize {
        self.input.numSamples as usize
    }

    /// Get a channel.
    pub fn channel(&self, index: usize) -> &[f32] {
        if index >= self.num_channels() {
            panic!("index out of bounds");
        }

        unsafe { std::slice::from_raw_parts(*self.input.channelData.offset(index as isize), self.input.numSamples as usize) }
    }
}

impl<'execute> GetInput<'execute, ChopInput> for OperatorInputs<'execute, ChopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn get_input(&self, index: usize) -> Option<&'execute ChopInput> {
        let input = self.inputs.getInputCHOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(ChopInput::ref_cast(unsafe { &*input }))
        }
    }
}

impl Index<usize> for ChopInput {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        self.channel(index)
    }
}
