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

    pub fn num_rows(&self) -> usize {
        self.input.numRows as usize
    }

    pub fn num_cols(&self) -> usize {
        self.input.numCols as usize
    }

    pub fn table_size(&self) -> [usize; 2] {
        let rows = self.num_rows();
        let cols = self.num_cols();
        [rows, cols]
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&str> {
        if row >= self.num_rows() || col >= self.num_cols() {
            None
        } else {
            let cell = unsafe { self.input.getCell(row as i32, col as i32) };
            if cell.is_null() {
                None
            } else {
                Some(unsafe { std::ffi::CStr::from_ptr(cell) }.to_str().expect("invalid utf8"))
            }
        }
    }

    pub fn text(&self) -> &str {
        self.cell(0, 0).unwrap()
    }
}

impl<'execute> GetInput<'execute, DatInput> for OperatorInputs<'execute, DatInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'execute DatInput> {
        let input = self.inputs.getInputDAT(index as i32);
        if input.is_null() {
            None
        } else {
            Some(DatInput::ref_cast(unsafe { &*input }))
        }
    }
}