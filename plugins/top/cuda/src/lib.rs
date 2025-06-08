use cudarc::runtime::sys::{cudaError::cudaSuccess, cudaStream_t, cudaSurfaceObject_t};
use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params, Default, Clone, Debug)]
struct CudaExampleParams {
    #[param(label = "Brightness", min = 0.0, max = 1.0, default = 1.0)]
    brightness: f64,
    #[param(label = "Speed", min = -10.0, max = 10.0, default = 1.0)]
    speed: f64,
    #[param(label = "Reset")]
    reset: Pulse,
}

pub struct CudaExample {
    execute_count: u32,
    params: CudaExampleParams,

    stream: Option<cudaStream_t>,
    surface_cache: td_rs_top::cuda::SurfaceCache,
    context: Arc<Mutex<TopContext>>,
}

impl TopNew for CudaExample {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            execute_count: 0,
            params: CudaExampleParams::default(),
            stream: None,
            surface_cache: td_rs_top::cuda::SurfaceCache::new(),
            context: Arc::new(Mutex::new(context)),
        }
    }
}

impl OpInfo for CudaExample {
    const OPERATOR_LABEL: &'static str = "CUDA Example";
    const OPERATOR_TYPE: &'static str = "Cudaexample";
    const OPERATOR_ICON: &'static str = "CDA";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
}

impl TopInfo for CudaExample {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cuda;
}

impl Op for CudaExample {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        match name {
            "Reset" => {
                self.execute_count = 0;
            }
            _ => {}
        }
    }
}

impl Top for CudaExample {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame_if_asked: true,
            ..Default::default()
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

        if let Err(e) = self.execute_cuda(&mut output, input) {
            eprintln!("CUDA execution failed: {}", e);
        }
    }
}

impl CudaExample {
    fn execute_cuda(
        &mut self,
        output: &mut TopOutput,
        _input: &OperatorInputs<TopInput>,
    ) -> Result<(), anyhow::Error> {
        use td_rs_top::cuda::CudaOutputInfo;

        let brightness = self.params.brightness as f32;
        let speed = self.params.speed as f32;
        let time = self.execute_count as f32 * speed * 0.01;

        let cuda_info = CudaOutputInfo {
            stream: std::ptr::null_mut(),
            texture_desc: td_rs_top::TextureDesc {
                width: 512,
                height: 512,
                depth: 1,
                tex_dim: td_rs_top::TexDim::E2D,
                pixel_format: td_rs_top::PixelFormat::BGRA8Fixed,
                aspect_x: 0.0,
                aspect_y: 0.0,
            },
            color_buffer_index: 0,
        };

        let array_info = output.create_cuda_array(&cuda_info)?;

        let began_successfully = {
            let mut ctx = self.context.lock().unwrap();
            ctx.begin_cuda_operations()
        };

        if !began_successfully {
            return Err(anyhow::anyhow!("Failed to begin CUDA operations"));
        }

        if self.stream.is_none() {
            let mut stream = std::ptr::null_mut();
            unsafe {
                let result = cudarc::runtime::sys::cudaStreamCreate(&mut stream);
                if result != cudaSuccess {
                    self.context.lock().unwrap().end_cuda_operations();
                    return Err(anyhow::anyhow!(
                        "Failed to create CUDA stream: {:?}",
                        result
                    ));
                }
            }
            self.stream = Some(stream);
        }

        let cuda_array = unsafe { array_info.cuda_array() };

        let mut surface_obj = 0;
        unsafe {
            use cudarc::runtime::sys::*;

            let mut res_desc: cudaResourceDesc = std::mem::zeroed();
            res_desc.resType = cudarc::runtime::sys::cudaResourceType::cudaResourceTypeArray;
            res_desc.res.array.array = cuda_array as cudaArray_t;

            let result = cudaCreateSurfaceObject(&mut surface_obj, &res_desc);
            if result != cudaSuccess {
                self.context.lock().unwrap().end_cuda_operations();
                return Err(anyhow::anyhow!(
                    "Failed to create surface object: {:?}",
                    result
                ));
            }
        }

        self.execute_pattern_kernel(surface_obj, 512, 512, brightness, time)?;

        unsafe {
            cudarc::runtime::sys::cudaDestroySurfaceObject(surface_obj);
        }

        self.context.lock().unwrap().end_cuda_operations();

        Ok(())
    }

    fn execute_pattern_kernel(
        &self,
        surface: cudaSurfaceObject_t,
        width: u32,
        height: u32,
        brightness: f32,
        time: f32,
    ) -> Result<(), anyhow::Error> {
        let kernel_src = r#"
extern "C" __global__ void pattern_kernel(
    cudaSurfaceObject_t surface,
    unsigned int width,
    unsigned int height,
    float brightness,
    float time
) {
    unsigned int x = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int y = blockIdx.y * blockDim.y + threadIdx.y;
    
    if (x >= width || y >= height) return;
    
    
    float fx = (float)x / width;
    float fy = (float)y / height;
    
    
    float wave1 = sinf((fx * 10.0f + time) * 3.14159f * 2.0f) * 0.5f + 0.5f;
    float wave2 = cosf((fy * 8.0f + time * 0.7f) * 3.14159f * 2.0f) * 0.5f + 0.5f;
    float pattern = wave1 * wave2 * brightness;
    
    
    uchar4 color = make_uchar4(
        (unsigned char)(pattern * 100.0f),          
        (unsigned char)(pattern * 150.0f),          
        (unsigned char)(pattern * 255.0f),          
        255                                         
    );
    
    
    surf2Dwrite(color, surface, x * sizeof(uchar4), y);
}
"#;

        use cudarc::driver::CudaContext;
        use cudarc::nvrtc::compile_ptx;

        let ptx = compile_ptx(kernel_src)
            .map_err(|e| anyhow::anyhow!("Failed to compile CUDA kernel: {:?}", e))?;

        let ctx = CudaContext::new(0)
            .map_err(|e| anyhow::anyhow!("Failed to create CUDA context: {:?}", e))?;

        let module = ctx
            .load_module(ptx)
            .map_err(|e| anyhow::anyhow!("Failed to load CUDA module: {:?}", e))?;

        let kernel = module
            .load_function("pattern_kernel")
            .map_err(|e| anyhow::anyhow!("Failed to load kernel function: {:?}", e))?;

        let block_size = (16, 16, 1);
        let grid_size = (
            (width + block_size.0 - 1) / block_size.0,
            (height + block_size.1 - 1) / block_size.1,
            1,
        );

        let stream = ctx.default_stream();

        unsafe {
            use cudarc::driver::{LaunchConfig, PushKernelArg};

            let config = LaunchConfig {
                grid_dim: grid_size,
                block_dim: block_size,
                shared_mem_bytes: 0,
            };

            stream
                .launch_builder(&kernel)
                .arg(&surface)
                .arg(&width)
                .arg(&height)
                .arg(&brightness)
                .arg(&time)
                .launch(config)
                .map_err(|e| anyhow::anyhow!("Failed to launch kernel: {:?}", e))?;
        }

        stream
            .synchronize()
            .map_err(|e| anyhow::anyhow!("Failed to synchronize stream: {:?}", e))?;

        Ok(())
    }
}

impl Drop for CudaExample {
    fn drop(&mut self) {
        if let Some(stream) = self.stream {
            if !stream.is_null() {
                unsafe {
                    cudarc::runtime::sys::cudaStreamDestroy(stream);
                }
            }
        }
    }
}

top_plugin!(CudaExample);
