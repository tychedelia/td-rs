use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_dat::*;
use td_rs_derive::Params;

#[derive(Param, Default, Clone)]
pub enum FilterType {
    UpperCamelCase,
    #[default]
    LowerCase,
    UpperCase,
}

#[derive(Params, Default, Clone)]
struct FilterDatParams {
    #[param(label = "Apply Filter", page = "Filter")]
    apply_scale: bool,
    #[param(label = "Filter", page = "Filter")]
    filter: FilterType,
}

/// Struct representing our DAT's state
pub struct FilterDat {
    params: FilterDatParams,
}

/// Impl block providing default constructor for plugin
impl FilterDat {
    pub(crate) fn new() -> Self {
        Self {
            params: Default::default(),
        }
    }
}

impl OpInfo for FilterDat {
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    // This Dat takes no input
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 1;
}

impl Op for FilterDat {}

impl Dat for FilterDat {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn execute(&mut self, output: &mut DatOutput, inputs: &OperatorInputs<DatInput>) {
        if let Some(input) = inputs.input(0) {
            let output = if input.is_table() {
                DatTableOutput::new(output)
            } else {
                DatTextOutput::new(output)
            };

            output[[0, 1]] = 1.0;
        }
    }

    fn general_info(&self, inputs: &OperatorInputs<DatInput>) -> DatGeneralInfo {
        DatGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
        }
    }
}

dat_plugin!(FilterDat);
