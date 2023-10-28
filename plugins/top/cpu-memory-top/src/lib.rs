mod frame_queue;

use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;
use crate::frame_queue::FrameQueue;

#[derive(Params, Default, Clone)]
struct CpuMemoryTopParams {
    #[param(label = "Brightness", min = 0.0, max = 1.0)]
    brightness: f64,
    #[param(label = "Speed", min = -10.0, max = 10.0, default = 1.0)]
    speed: f64,
    #[param(label = "Reset")]
    reset: Pulse,
}

/// Struct representing our SOP's state
pub struct CpuMemoryTop {
    execute_count: u32,
    params: CpuMemoryTopParams,
    pub ctx: Arc<Mutex<TopContext>>,
    pub frame_queue: FrameQueue,
}

impl CpuMemoryTop {
    // fn fill_buffer(buf: &mut TOP_Buffer, byte_offset: usize, width: usize, height: usize, step: f64, brightness: f64) {
    //     let required_size = byte_offset + width * height * 4 * std::mem::size_of::<f32>();
    //     assert!(buf.size >= required_size);
    //
    //     let byte_slice = &mut buf.data[byte_offset..];
    //     let mem: &mut [f32] = bytemuck::cast_slice_mut(byte_slice);
    //
    //     let xstep = ((step as isize).wrapping_rem(width as isize)).rem_euclid(width as isize) as usize;
    //     let ystep = ((step as isize).wrapping_rem(height as isize)).rem_euclid(height as isize) as usize;
    //
    //     for y in 0..height {
    //         for x in 0..width {
    //             let pixel = &mut mem[4 * (y * width + x)..4 * (y * width + x + 1)];
    //
    //             // RGBA
    //             pixel[0] = if x > xstep { brightness as f32 } else { 0.0 };
    //             pixel[1] = if y > ystep { brightness as f32 } else { 0.0 };
    //             pixel[2] = ((xstep % 50) as f32 / 50.0) * brightness as f32;
    //             pixel[3] = 1.0;
    //         }
    //     }
    // }
}

impl TopNew for CpuMemoryTop {
    fn new(info: NodeInfo, context: TopContext) -> Self {
        let ctx = Arc::new(Mutex::new(context));
        Self {
            frame_queue: FrameQueue::new(ctx.clone()),
            execute_count: 0,
            ctx,
            params: CpuMemoryTopParams::default(),
        }
    }
}

impl OpInfo for CpuMemoryTop {
    const OPERATOR_LABEL: &'static str = "CPU Mem Sample";
    const OPERATOR_TYPE: &'static str = "Cpumemsample";
    const OPERATOR_ICON: &'static str = "CPM";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 0;
}

impl TopInfo for CpuMemoryTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cpu;
}

impl Op for CpuMemoryTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Top for CpuMemoryTop {
    fn execute(&mut self, output: TopOutput, _input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

        if let Some(buf_info) = self.frame_queue.get_buffer_to_upload() {
            if let Some(buf) = buf_info.buf {
                // output.up
            }
        }

        let mut upload = UploadInfo::default();
        upload.texture_desc.width = 256;
        upload.texture_desc.height = 256;
        upload.texture_desc.tex_dim = TexDim::E2D;
        upload.texture_desc.pixel_format = PixelFormat::RGBA32Float;
        let size = upload.texture_desc.width * upload.texture_desc.height * 4;

        if let Some(input) = _input.input(0) {

        }

    }
}

top_plugin!(CpuMemoryTop);
