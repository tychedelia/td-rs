mod frame_queue;

use crate::frame_queue::FrameQueue;
use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params, Default, Clone, Debug)]
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
    step: f64,
    params: CpuMemoryTopParams,
    pub ctx: Arc<Mutex<TopContext>>,
    pub frame_queue: FrameQueue,
    previous_result: Option<TopDownloadResult>,
}

impl CpuMemoryTop {
    fn fill_buffer(
        buf: &mut TopBuffer,
        byte_offset: usize,
        width: usize,
        height: usize,
        step: f64,
        brightness: f64,
    ) {
        let required_size = byte_offset + width * height * 4 * std::mem::size_of::<f32>();
        assert!(buf.size() >= required_size);

        let byte_slice: &mut [f32] = &mut buf.data_mut()[byte_offset..];
        let mem: &mut [f32] = bytemuck::cast_slice_mut(byte_slice);

        let xstep =
            ((step as isize).wrapping_rem(width as isize)).rem_euclid(width as isize) as usize;
        let ystep =
            ((step as isize).wrapping_rem(height as isize)).rem_euclid(height as isize) as usize;

        for y in 0..height {
            for x in 0..width {
                let pixel = &mut mem[4 * (y * width + x)..4 * (y * width + x + 1)];

                // RGBA
                pixel[0] = if x > xstep { brightness as f32 } else { 0.0 };
                pixel[1] = if y > ystep { brightness as f32 } else { 0.0 };
                pixel[2] = ((xstep % 50) as f32 / 50.0) * brightness as f32;
                pixel[3] = 1.0;
            }
        }
    }

    fn fill_and_upload(
        &mut self,
        output: &mut TopOutput,
        speed: f64,
        width: usize,
        height: usize,
        tex_dim: TexDim,
        mut num_layers: usize,
        color_buffer_index: usize,
    ) {
        let depth = match tex_dim {
            TexDim::E2DArray | TexDim::E3D => num_layers,
            _ => 1,
        };

        if tex_dim == TexDim::ECube {
            num_layers = 6;
        };
        let info = UploadInfo {
            buffer_offset: 0,
            texture_desc: TextureDesc {
                tex_dim,
                width,
                height,
                pixel_format: PixelFormat::RGBA32Float,
                aspect_x: 0.0,
                depth,
                aspect_y: 0.0,
            },
            first_pixel: Default::default(),
            color_buffer_index,
        };

        let layer_bytes =
            (info.texture_desc.width * info.texture_desc.height * 4 * std::mem::size_of::<f32>())
                as u64;
        let byte_size = layer_bytes * num_layers as u64;
        let mut ctx = self.ctx.lock().unwrap();
        let mut buf = ctx.create_output_buffer(byte_size as usize, TopBufferFlags::None);

        let mut byte_offset = 0;
        for _ in 0..num_layers {
            self.step += speed;
            Self::fill_buffer(
                &mut buf,
                byte_offset as usize,
                info.texture_desc.width,
                info.texture_desc.height,
                self.step,
                self.params.brightness,
            );
            byte_offset += layer_bytes;
        }

        output.upload_buffer(&mut buf, &info);
    }
}

impl TopNew for CpuMemoryTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        let ctx = Arc::new(Mutex::new(context));
        Self {
            frame_queue: FrameQueue::new(ctx.clone()),
            execute_count: 0,
            ctx,
            params: CpuMemoryTopParams::default(),
            step: 0.0,
            previous_result: None,
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

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reset" {
            self.step = 0.0;
        }
    }
}

impl Top for CpuMemoryTop {
    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

        if let Some(buf_info) = self.frame_queue.get_buffer_to_upload() {
            if let Some(mut buf) = buf_info.buf {
                output.upload_buffer(&mut buf, &buf_info.upload_info)
            }
        }

        self.fill_and_upload(&mut output, self.params.speed, 256, 256, TexDim::E2D, 1, 0);

        if let Some(input) = input.input(0) {
            let download_opts = DownloadOptions::default();
            let res = input.download_texture(download_opts);
            if let Some(prev) = &mut self.previous_result {
                let upload_info = UploadInfo {
                    color_buffer_index: 3,
                    texture_desc: prev.texture_desc(),
                    ..Default::default()
                };
                let mut ctx = self.ctx.lock().unwrap();
                let mut buf = ctx.create_output_buffer(prev.size(), TopBufferFlags::None);
                buf.data_mut::<f32>().copy_from_slice(prev.data());
                output.upload_buffer(&mut buf, &upload_info);
            }

            self.previous_result = Some(res);
        }
    }
}

top_plugin!(CpuMemoryTop);
