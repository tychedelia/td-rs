use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_sop::*;
use td_rs_derive::Params;

#[derive(Params, Default, Clone)]
struct FilterSopParams {
    #[param(label = "Apply Scale", page = "Filter")]
    apply_scale: bool,
    #[param(label = "Scale", page = "Filter", min = -10.0, max = 10.0)]
    scale: f32,
    #[param(label = "Apply Offset", page = "Filter")]
    apply_offset: bool,
    #[param(label = "Offset", page = "Filter", min = -10.0, max = 10.0)]
    offset: f32,
}

/// Struct representing our SOP's state
pub struct FilterSop {
    params: FilterSopParams,
}

/// Impl block providing default constructor for plugin
impl FilterSop {
    pub(crate) fn new() -> Self {
        Self {
            params: FilterSopParams {
                apply_scale: true,
                scale: 1.0,
                apply_offset: false,
                offset: 0.0,
            }
        }
    }
}

impl SopInfo for FilterSop {
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 1;
}

impl Op for FilterSop {}

impl Sop for FilterSop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn execute(&mut self, output: &mut SopOutput, inputs: &OperatorInputs) {
        if let Some(input) = inputs.get_input(0) {
        }
    }

    fn general_info(&self, inputs: &OperatorInputs) -> SopGeneralInfo {
        SopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            direct_to_gpu: false,
        }
    }


}

sop_plugin!(FilterSop);
