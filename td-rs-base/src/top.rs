use crate::cxx::{OP_PixelFormat, OP_TOPInput, OP_TexDim};
use crate::{GetInput, OperatorInputs};
use ref_cast::RefCast;

#[cfg(feature = "cuda")]
use crate::cxx::OP_CUDAArrayInfo;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub enum TexDim {
    #[default]
    EInvalid,
    E2D,
    E2DArray,
    E3D,
    ECube,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
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

impl From<&OP_PixelFormat> for PixelFormat {
    fn from(pixel_format: &OP_PixelFormat) -> Self {
        match pixel_format {
            OP_PixelFormat::Invalid => PixelFormat::Invalid,
            OP_PixelFormat::BGRA8Fixed => PixelFormat::BGRA8Fixed,
            OP_PixelFormat::RGBA8Fixed => PixelFormat::RGBA8Fixed,
            OP_PixelFormat::RGBA16Fixed => PixelFormat::RGBA16Fixed,
            OP_PixelFormat::RGBA16Float => PixelFormat::RGBA16Float,
            OP_PixelFormat::RGBA32Float => PixelFormat::RGBA32Float,
            OP_PixelFormat::Mono8Fixed => PixelFormat::Mono8Fixed,
            OP_PixelFormat::Mono16Fixed => PixelFormat::Mono16Fixed,
            OP_PixelFormat::Mono16Float => PixelFormat::Mono16Float,
            OP_PixelFormat::Mono32Float => PixelFormat::Mono32Float,
            OP_PixelFormat::RG8Fixed => PixelFormat::RG8Fixed,
            OP_PixelFormat::RG16Fixed => PixelFormat::RG16Fixed,
            OP_PixelFormat::RG16Float => PixelFormat::RG16Float,
            OP_PixelFormat::RG32Float => PixelFormat::RG32Float,
            OP_PixelFormat::A8Fixed => PixelFormat::A8Fixed,
            OP_PixelFormat::A16Fixed => PixelFormat::A16Fixed,
            OP_PixelFormat::A16Float => PixelFormat::A16Float,
            OP_PixelFormat::A32Float => PixelFormat::A32Float,
            OP_PixelFormat::MonoA8Fixed => PixelFormat::MonoA8Fixed,
            OP_PixelFormat::MonoA16Fixed => PixelFormat::MonoA16Fixed,
            OP_PixelFormat::MonoA16Float => PixelFormat::MonoA16Float,
            OP_PixelFormat::MonoA32Float => PixelFormat::MonoA32Float,
            OP_PixelFormat::SBGRA8Fixed => PixelFormat::SBGRA8Fixed,
            OP_PixelFormat::SRGBA8Fixed => PixelFormat::SRGBA8Fixed,
            OP_PixelFormat::RGB10A2Fixed => PixelFormat::RGB10A2Fixed,
            OP_PixelFormat::RGB11Float => PixelFormat::RGB11Float,
        }
    }
}

impl From<&PixelFormat> for OP_PixelFormat {
    fn from(pixel_format: &PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::Invalid => OP_PixelFormat::Invalid,
            PixelFormat::BGRA8Fixed => OP_PixelFormat::BGRA8Fixed,
            PixelFormat::RGBA8Fixed => OP_PixelFormat::RGBA8Fixed,
            PixelFormat::RGBA16Fixed => OP_PixelFormat::RGBA16Fixed,
            PixelFormat::RGBA16Float => OP_PixelFormat::RGBA16Float,
            PixelFormat::RGBA32Float => OP_PixelFormat::RGBA32Float,
            PixelFormat::Mono8Fixed => OP_PixelFormat::Mono8Fixed,
            PixelFormat::Mono16Fixed => OP_PixelFormat::Mono16Fixed,
            PixelFormat::Mono16Float => OP_PixelFormat::Mono16Float,
            PixelFormat::Mono32Float => OP_PixelFormat::Mono32Float,
            PixelFormat::RG8Fixed => OP_PixelFormat::RG8Fixed,
            PixelFormat::RG16Fixed => OP_PixelFormat::RG16Fixed,
            PixelFormat::RG16Float => OP_PixelFormat::RG16Float,
            PixelFormat::RG32Float => OP_PixelFormat::RG32Float,
            PixelFormat::A8Fixed => OP_PixelFormat::A8Fixed,
            PixelFormat::A16Fixed => OP_PixelFormat::A16Fixed,
            PixelFormat::A16Float => OP_PixelFormat::A16Float,
            PixelFormat::A32Float => OP_PixelFormat::A32Float,
            PixelFormat::MonoA8Fixed => OP_PixelFormat::MonoA8Fixed,
            PixelFormat::MonoA16Fixed => OP_PixelFormat::MonoA16Fixed,
            PixelFormat::MonoA16Float => OP_PixelFormat::MonoA16Float,
            PixelFormat::MonoA32Float => OP_PixelFormat::MonoA32Float,
            PixelFormat::SBGRA8Fixed => OP_PixelFormat::SBGRA8Fixed,
            PixelFormat::SRGBA8Fixed => OP_PixelFormat::SRGBA8Fixed,
            PixelFormat::RGB10A2Fixed => OP_PixelFormat::RGB10A2Fixed,
            PixelFormat::RGB11Float => OP_PixelFormat::RGB11Float,
        }
    }
}

impl From<&TexDim> for OP_TexDim {
    fn from(tex_dim: &TexDim) -> Self {
        match tex_dim {
            TexDim::EInvalid => OP_TexDim::eInvalid,
            TexDim::E2D => OP_TexDim::e2D,
            TexDim::E2DArray => OP_TexDim::e2DArray,
            TexDim::E3D => OP_TexDim::e3D,
            TexDim::ECube => OP_TexDim::eCube,
        }
    }
}

#[derive(Debug, Default)]
pub struct DownloadOptions {
    pub vertical_flip: bool,
    pub pixel_format: PixelFormat,
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct TopInput {
    input: OP_TOPInput,
}

impl TopInput {
    pub fn texture_desc(&self) -> TextureDesc {
        let desc = crate::cxx::getTOPInputTextureDesc(&self.input);
        TextureDesc {
            width: desc.width as usize,
            height: desc.height as usize,
            depth: desc.depth as usize,
            tex_dim: match desc.texDim {
                OP_TexDim::eInvalid => TexDim::EInvalid,
                OP_TexDim::e2D => TexDim::E2D,
                OP_TexDim::e2DArray => TexDim::E2DArray,
                OP_TexDim::e3D => TexDim::E3D,
                OP_TexDim::eCube => TexDim::ECube,
            },
            pixel_format: PixelFormat::from(&desc.pixelFormat),
            aspect_x: desc.aspectX,
            aspect_y: desc.aspectY,
        }
    }

    pub fn download_texture(&self, opts: DownloadOptions) -> TopDownloadResult {
        let opts = crate::cxx::OP_TOPInputDownloadOptions {
            verticalFlip: false,
            pixelFormat: (&opts.pixel_format).into(),
        };
        let download = unsafe { self.input.downloadTexture(&opts, std::ptr::null_mut()) };
        TopDownloadResult::new(download)
    }

    #[cfg(feature = "cuda")]
    pub fn get_cuda_array(
        &self,
        stream: cudarc::runtime::sys::cudaStream_t,
    ) -> Result<CudaArrayInfo, anyhow::Error> {
        use autocxx::moveit::moveit;

        moveit! { let acquire_info = unsafe {
            crate::cxx::createCUDAAcquireInfo(stream as *mut autocxx::c_void)
        } }

        let array_info = unsafe {
            self.input
                .getCUDAArray(acquire_info.as_ref().get_ref(), std::ptr::null_mut())
        };

        CudaArrayInfo::new(array_info)
    }
}

pub struct TopDownloadResult {
    result: cxx::UniquePtr<crate::cxx::TD_OP_SmartRef_TD_OP_TOPDownloadResult_AutocxxConcrete>,
}

impl TopDownloadResult {
    pub fn new(
        result: cxx::UniquePtr<crate::cxx::TD_OP_SmartRef_TD_OP_TOPDownloadResult_AutocxxConcrete>,
    ) -> Self {
        Self { result }
    }

    pub fn size(&mut self) -> usize {
        crate::cxx::getDownloadDataSize(self.result.pin_mut()) as usize
    }

    pub fn data<T>(&mut self) -> &[T] {
        let size = self.size();
        let data = crate::cxx::getDownloadData(self.result.pin_mut());
        unsafe { std::slice::from_raw_parts(data as *const T, size) }
    }

    pub fn texture_desc(&mut self) -> TextureDesc {
        let desc = crate::cxx::getDownloadTextureDesc(self.result.pin_mut());
        TextureDesc {
            width: desc.width as usize,
            height: desc.height as usize,
            depth: desc.depth as usize,
            tex_dim: match desc.texDim {
                OP_TexDim::eInvalid => TexDim::EInvalid,
                OP_TexDim::e2D => TexDim::E2D,
                OP_TexDim::e2DArray => TexDim::E2DArray,
                OP_TexDim::e3D => TexDim::E3D,
                OP_TexDim::eCube => TexDim::ECube,
            },
            pixel_format: PixelFormat::from(&desc.pixelFormat),
            aspect_x: 0.0,
            aspect_y: 0.0,
        }
    }
}

impl Drop for TopDownloadResult {
    fn drop(&mut self) {
        if self.result.is_null() {
            return;
        }
        crate::cxx::releaseDownloadResult(self.result.pin_mut())
    }
}

impl<'cook> GetInput<'cook, TopInput> for OperatorInputs<'cook, TopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'cook TopInput> {
        let input = self.inputs.getInputTOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(TopInput::ref_cast(unsafe { &*input }))
        }
    }
}

#[cfg(feature = "cuda")]
#[derive(Debug)]
pub struct CudaArrayInfo {
    pub(crate) info: *const OP_CUDAArrayInfo,
}

#[cfg(feature = "cuda")]
unsafe impl Send for CudaArrayInfo {}
#[cfg(feature = "cuda")]
unsafe impl Sync for CudaArrayInfo {}

#[cfg(feature = "cuda")]
impl CudaArrayInfo {
    pub fn new(info: *const OP_CUDAArrayInfo) -> Result<Self, anyhow::Error> {
        if info.is_null() {
            return Err(anyhow::anyhow!("Null CUDA array info pointer"));
        }
        Ok(Self { info })
    }

    pub fn texture_desc(&self) -> TextureDesc {
        unsafe {
            let cpp_desc = crate::cxx::getCUDAArrayInfoTextureDesc(self.info.as_ref().unwrap());
            TextureDesc {
                width: cpp_desc.width as usize,
                height: cpp_desc.height as usize,
                depth: cpp_desc.depth as usize,
                tex_dim: match cpp_desc.texDim {
                    OP_TexDim::eInvalid => TexDim::EInvalid,
                    OP_TexDim::e2D => TexDim::E2D,
                    OP_TexDim::e2DArray => TexDim::E2DArray,
                    OP_TexDim::e3D => TexDim::E3D,
                    OP_TexDim::eCube => TexDim::ECube,
                },
                pixel_format: (&cpp_desc.pixelFormat).into(),
                aspect_x: cpp_desc.aspectX,
                aspect_y: cpp_desc.aspectY,
            }
        }
    }

    #[cfg(feature = "cuda")]
    pub unsafe fn cuda_array(&self) -> *mut cudarc::runtime::sys::cudaArray {
        crate::cxx::getCUDAArrayInfoArray(self.info.as_ref().unwrap())
            as *mut cudarc::runtime::sys::cudaArray
    }
}
