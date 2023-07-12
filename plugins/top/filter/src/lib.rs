mod filter;

use crate::filter::Filter;
use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_derive::{Param, Params};
use td_rs_top::top_plugin;
use td_rs_top::*;

#[derive(Param, Default, Clone)]
enum DownloadTypes {
    #[default]
    Delayed,
    Instant,
}

#[derive(Params, Default, Clone)]
struct FilterChopParams {
    #[param(label = "Bits per color", min = 1.0, max = 8.0)]
    bits_per_color: u32,
    #[param(label = "Dither")]
    dither: bool,
    #[param(label = "Multithread")]
    multithread: bool,
    #[param(label = "Download type")]
    download_type: DownloadTypes,
}

/// Struct representing our CHOP's state
pub struct FilterChop {
    params: FilterChopParams,
}

/// Impl block providing default constructor for plugin
impl FilterChop {
    pub(crate) fn new() -> Self {
        Self {
            params: FilterChopParams::default(),
        }
    }
}

impl TopInfo for FilterChop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::CpuWrite;
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

    fn general_info(&self, inputs: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame_if_asked: false,
            mem_pixel_type: CpuMemPixelType::RGBA8Fixed,
            input_size_index: 0,
            ..Default::default()
        }
    }
}

impl TopExecute for FilterChop {
    fn execute_cpu(&mut self, input: &OperatorInputs<TopCpuInput>, output: TopCpuOutput) {
        let input_height = top.height();
        let input_width = top.width();

        let input_buf: Option<&[u32]> = input.texture(top, opts);
    }
}
// fn execute(&mut self, input: &OperatorInputs<TopInput>, output: TopOutputSpecs) {
// if let Some(top) = input.input(0) {
//
//     let opts = InputDownloadOptions {
//         download_type: match self.params.download_type {
//             DownloadTypes::Delayed => InputDownloadType::Delayed,
//             DownloadTypes::Instant => InputDownloadType::Instant,
//         },
//         cpu_mem_pixel_type: CpuMemPixelType::BGRA8Fixed,
//         ..Default::default()
//     };
//
//     let input_buffer: Option<&[u32]> = input.top_data_in_cpu_memory(top, opts);
//     if let Some(buf) = input_buffer {
//         if self.params.multithread {} else {
//             // Filter::do_filter_work(buf, input_width, input_height, specs., self.params.dither)
//         }
//     }
// }
// }

top_plugin!(FilterChop);
