use std::ops::Index;
use crate::{GetInput, OperatorInputs};
use crate::cxx::OP_DATInput;
use ref_cast::RefCast;

/// A dat input.
#[repr(transparent)]
#[derive(RefCast)]
pub struct DatInput {
    input: OP_DATInput,
}

pub enum DatOutputType {
    Table,
    Text,
}

impl DatInput {
    pub fn is_table(&self) -> bool {
        self.input.isTable
    }
}

impl<'execute> GetInput<'execute, DatInput> for OperatorInputs<'execute, DatInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn get_input(&self, index: usize) -> Option<&'execute DatInput> {
        let input = self.inputs.getInputDAT(index as i32);
        if input.is_null() {
            None
        } else {
            Some(DatInput::ref_cast(unsafe { &*input }))
        }
    }
}

// impl Index<usize> for DatInput {
//     type Output = [f32];
//
//     fn index(&self, index: usize) -> &Self::Output {
//         self.channel(index)
//     }
// }