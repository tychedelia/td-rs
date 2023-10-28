pub mod cxx;

use std::pin::Pin;
pub use td_rs_base::top::*;
pub use td_rs_base::*;

pub struct TopOutput<'execute> {
    output: Pin<&'execute mut cxx::TOP_Output>,
}

impl<'execute> TopOutput<'execute> {
    pub fn new(output: Pin<&'execute mut cxx::TOP_Output>) -> TopOutput<'execute> {
        Self { output }
    }
}

pub trait TopInfo {
    const EXECUTE_MODE: ExecuteMode;
}

#[derive(Debug, Default)]
pub struct TopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
    pub input_size_index: i32,
}

pub enum ExecuteMode {
    Cpu,
    Cuda,
}

#[derive(Debug, Default)]
pub enum TexDim {
    #[default]
    EInvalid,
    E2D,
    E2DArray,
    E3D,
    ECube,
}

#[derive(Debug, Default)]
pub enum PixelFormat {
    #[default]
    Invalid,

    // 8-bit per color, BGRA pixels. This is preferred for 4 channel 8-bit data
    BGRA8Fixed,
    // 8-bit per color, RGBA pixels. Only use this one if absolutely nessessary.
    RGBA8Fixed,
    RGBA16Fixed,
    RGBA16Float,
    RGBA32Float,

    Mono8Fixed,
    Mono16Fixed,
    Mono16Float,
    Mono32Float,

    // RG two channel
    RG8Fixed,
    RG16Fixed,
    RG16Float,
    RG32Float,

    // Alpha only
    A8Fixed,
    A16Fixed,
    A16Float,
    A32Float,

    // Mono with Alpha
    MonoA8Fixed,
    MonoA16Fixed,
    MonoA16Float,
    MonoA32Float,

    // sRGB. use SBGRA if possible since that's what most GPUs use
    SBGRA8Fixed,
    SRGBA8Fixed,

    RGB10A2Fixed,
    // 11-bit float, positive values only. B is actually 10 bits
    RGB11Float,
}

pub struct TextureDesc {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub tex_dim: TexDim,
    pub pixel_format: PixelFormat,
    pub aspect_x: f32,
    pub aspect_y: f32,
}

impl Default for TextureDesc {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 1,
            tex_dim: TexDim::EInvalid,
            pixel_format: PixelFormat::Invalid,
            aspect_x: 0.0,
            aspect_y: 0.0,
        }
    }
}


#[derive(Debug, Default)]
pub enum FirstPixel {
    #[default]
    BottomLeft,
    TopLeft,
}

#[derive(Debug, Default)]
pub struct UploadInfo {
    pub buffer_offset: usize,
    pub texture_desc: TextureDesc,
    pub first_pixel: FirstPixel,
    pub color_buffer_index: usize,
}

pub trait Top: Op {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo::default()
    }

    fn execute(&mut self, _output: TopOutput, _input: &OperatorInputs<TopInput>) {}
}

#[macro_export]
macro_rules! top_plugin {
    ($plugin_ty:ty) => {
        use td_rs_top::cxx::c_void;
        use td_rs_top::cxx::OP_CustomOPInfo;
        use td_rs_top::NodeInfo;

        #[no_mangle]
        pub extern "C" fn top_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) -> cxx::TOP_ExecuteMode {
            unsafe {
                td_rs_top::op_info::<$plugin_ty>(op_info);
                match <$plugin_ty>::EXECUTE_MODE {
                    td_rs_top::ExecuteMode::Cuda => panic!("Cuda not supported"),
                    td_rs_top::ExecuteMode::Cpu => cxx::TOP_ExecuteMode::CPUMem,
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn top_new_impl(info: NodeInfo) -> Box<dyn Top> {
            Box::new(<$plugin_ty>::new(info))
        }
    };
}
