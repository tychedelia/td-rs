use crate::cxx::OP_POPInput;
use crate::{GetInput, OperatorInputs};
use ref_cast::RefCast;

#[repr(transparent)]
#[derive(RefCast)]
pub struct PopInput {
    input: OP_POPInput,
}

impl<'cook> GetInput<'cook, PopInput> for OperatorInputs<'cook, PopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'cook PopInput> {
        let input = self.inputs.getInputPOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(PopInput::ref_cast(unsafe { &*input }))
        }
    }
}
