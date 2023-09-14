use crate::cxx::OP_TOPInput;
use crate::{GetInput, OperatorInputs};
use ref_cast::RefCast;

#[repr(transparent)]
#[derive(RefCast)]
pub struct TopInput {
    input: OP_TOPInput,
}

impl TopInput {}

impl<'execute> GetInput<'execute, TopInput> for OperatorInputs<'execute, TopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'execute TopInput> {
        let input = self.inputs.getInputTOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(TopInput::ref_cast(unsafe { &*input }))
        }
    }
}
