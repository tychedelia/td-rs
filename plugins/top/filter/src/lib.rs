use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_top::*;
use td_rs_derive::Params;
use td_rs_top::top_plugin;

#[derive(Params, Default, Clone)]
struct FilterChopParams {
}

/// Struct representing our CHOP's state
pub struct FilterChop {
    params: FilterChopParams,
}

/// Impl block providing default constructor for plugin
impl FilterChop {
    pub(crate) fn new() -> Self {
        Self {
            params: FilterChopParams {
            }
        }
    }
}

impl OpInfo for FilterChop {
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    const MIN_INPUTS: usize = 1;
    const MAX_INPUTS: usize = 1;
}

impl Op for FilterChop {}

impl Top for FilterChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    // fn general_info(&self, inputs: &OperatorInputs<TopInput>) -> TopGeneralInfo {
    //     TopGeneralInfo {
    //         cook_every_frame: false,
    //         clear_buffers: false,
    //         mipmap_all_tops: false,
    //         cook_every_frame_if_asked: false,
    //         input_size_index: 0,
    //         mem_pixel_type: Default::default(),
    //     }
    // }
}

top_plugin!(FilterChop);
