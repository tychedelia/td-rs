use cudarc::driver::result::stream::StreamKind;
use cudarc::driver::result::stream::StreamKind::Default;
use cudarc::driver::sys::{
    CUarray, CUDA_RESOURCE_DESC, CUstream, CUsurfObject,
};

use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params)]
struct CudaTopParams {}

struct CudaTop {
    execute_count: u32,
    params: CudaTopParams,
    context: TopContext,
    stream: CUstream,
    input_surface: Option<CUsurfObject>,
    output_surfaces: Vec<CUsurfObject>,
}

impl CudaTop {
    unsafe fn setup_cuda_surface(&mut self, mut surface: CUsurfObject, arr: CUarray) {
        if !surface.is_null() {
            let mut desc: CUDA_RESOURCE_DESC = Default::default();
            cudarc::driver::sys::cuSurfObjectGetResourceDesc(
                (&mut desc) as *mut CUDA_RESOURCE_DESC,
                surface,
            )
            .result()
            .expect("Failed to get surface resource description");
            if desc.resType != cudarc::driver::sys::CUresourcetype_enum::CU_RESOURCE_TYPE_ARRAY {
                panic!("Surface is not an array");
            }
            cudarc::driver::sys::cuSurfObjectDestroy(surface);
            *surface = std::ptr::null_mut();
        } else {
            let mut desc: CUDA_RESOURCE_DESC = Default::default();
            desc.resType = cudarc::driver::sys::CUresourcetype_enum::CU_RESOURCE_TYPE_ARRAY;
            desc.res.array.hArray = arr;
            cudarc::driver::sys::cuSurfObjectCreate(
                (&mut surface) as *mut CUsurfObject,
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
            params: CudaTopParams {},
            output_surfaces: vec![],
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
        for surf in self.output_surfaces {
            cuda::surface::destroy(surf).expect("Failed to destroy output surface");
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

    fn execute(&mut self, _output: TopOutput, _input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

    }
}
