use cudarc::driver::result::stream::StreamKind;
use cudarc::driver::sys::{CUarray, CUDA_RESOURCE_DESC, cudaError_enum, CUresult, CUstream, CUsurfObject};

use td_rs_derive::Params;
use td_rs_top::*;
use td_rs_top::cxx::OP_TexDim;
use td_rs_top::sop::Color;

#[derive(Params, Default)]
struct CudaTopParams {
    #[param(label = "Color 1")]
    color_1: Color,
    #[param(label = "Color 2")]
    color_2: Color,
}

struct CudaTop {
    execute_count: u32,
    params: CudaTopParams,
    context: TopContext,
    stream: CUstream,
    input_surface: Option<CUsurfObject>,
    output_surfaces: [CUsurfObject; 2],
}

impl CudaTop {
    fn setup_cuda_surface(&mut self, index: usize, arr: &CUarray) {
        unsafe {
            let surface = self.output_surfaces[index];
            let mut desc: CUDA_RESOURCE_DESC = Default::default();
            desc.resType = cudarc::driver::sys::CUresourcetype_enum::CU_RESOURCE_TYPE_ARRAY;
            desc.res.array.hArray = *arr;
            cudarc::driver::sys::cuSurfObjectCreate(
                surface as *mut CUsurfObject,
                (&mut desc) as *mut CUDA_RESOURCE_DESC,
            )
                .result()
                .expect("Failed to create surface");
        }
    }
}

impl TopNew for CudaTop {
    fn new(info: NodeInfo, context: TopContext) -> Self {
        let stream = cudarc::driver::result::stream::create(StreamKind::Default).expect("Failed to create stream");
        let input_surface = None;
        Self {
            execute_count: 0,
            stream,
            input_surface,
            context,
            params: CudaTopParams::default(),
            output_surfaces: [0; 2],
        }
    }
}

impl Drop for CudaTop {
    fn drop(&mut self) {
        if !self.stream.is_null() {
            unsafe { cudarc::driver::result::stream::destroy(self.stream).expect("Failed to destroy stream"); }
        }
        if let Some(surf) = self.input_surface {
            cuda::surface::destroy(surf).expect("Failed to destroy input surface");
        }
        for surf in self.output_surfaces.iter() {
            cuda::surface::destroy(*surf).expect("Failed to destroy output surface");
        }
    }
}

impl Op for CudaTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl OpInfo for CudaTop {
    const OPERATOR_TYPE: &'static str = "Cudasample";
    const OPERATOR_LABEL: &'static str = "CUDA Sample";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
}

impl TopInfo for CudaTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cuda;
}

impl Top for CudaTop {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame_if_asked: true,
            ..Default::default()
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;
        
        let mut info = TopCudaOutputInfo {
            stream: self.stream as *mut std::ffi::c_void,
            texture_desc: TextureDesc {
                width: 256,
                height: 256,
                tex_dim: TexDim::E2D,
                pixel_format: PixelFormat::BGRA8Fixed,
                ..Default::default()
            },
            color_buffer_index: 0,
        };

        let ratio =  info.texture_desc.height as f32 / info.texture_desc.width as f32;

        if let Some(input) = input.input(0) {
            info.texture_desc = input.texture_desc();

            if !matches!(info.texture_desc.pixel_format, PixelFormat::BGRA8Fixed) {
                self.set_error("Input must be BGRA8Fixed");
                return;
            }

            let acquire_info = CudaAcquireInfo {
                stream: self.stream as *mut std::ffi::c_void,
            };
            let array = input.get_cuda_array(acquire_info);
            let Some(output_info) = output.create_cuda_array(&info) else {
                return;
            };

            let aux_info = TopCudaOutputInfo {
                stream: self.stream as *mut std::ffi::c_void,
                texture_desc: TextureDesc {
                    width: 1280,
                    height: 720,
                    tex_dim: TexDim::E2D,
                    pixel_format: PixelFormat::BGRA8Fixed,
                    ..Default::default()
                },
                color_buffer_index: 1,
            };
            let Some(aux_output_info) = output.create_cuda_array(&aux_info) else {
                return;
            };

            if !self.context.begin_cuda_operations() {
                return;
            }
            self.setup_cuda_surface(0, &output_info.cuda_array);

            unsafe {
                doCUDAOperation(
                    aux_info.texture_desc.width as i32,
                    aux_info.texture_desc.height as i32,
                    aux_info.texture_desc.depth as i32,
                    match aux_info.texture_desc.tex_dim {
                        TexDim::E2D => OP_TexDim::e2D,
                        TexDim::E2DArray => OP_TexDim::e2DArray,
                        TexDim::E3D => OP_TexDim::e3D,
                        TexDim::ECube => OP_TexDim::eCube,
                        _ => OP_TexDim::eInvalid,
                    },
                    self.params.color_1.r, self.params.color_1.g, self.params.color_1.b, self.params.color_1.a,
                    self.input_surface.unwrap(),
                    self.output_surfaces[0],
                    self.stream,
                );
            }

            self.context.end_cuda_oprations();

        }
    }
}

#[link(name = "baz", kind = "static")]
extern "C" {
    fn doCUDAOperation(
        width: i32,
        height: i32,
        depth: i32,
        dim: OP_TexDim,
        color_r: f32,
        color_g: f32,
        color_b: f32,
        color_a: f32,
        input: CUsurfObject,
        output: CUsurfObject,
        stream: CUstream,
    ) -> CUresult;
}

top_plugin!(CudaTop);
