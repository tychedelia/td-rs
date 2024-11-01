pub mod cxx;

pub use ::cxx::UniquePtr;

use std::pin::Pin;
pub use td_rs_base::top::*;
pub use td_rs_base::*;

pub struct TopOutput<'cook> {
    output: Pin<&'cook mut cxx::TOP_Output>,
}

impl<'cook> TopOutput<'cook> {
    pub fn new(output: Pin<&'cook mut cxx::TOP_Output>) -> TopOutput<'cook> {
        Self { output }
    }

    pub fn upload_buffer(&mut self, buffer: &mut TopBuffer, info: &UploadInfo) {
        let info = crate::cxx::TOP_UploadInfo {
            bufferOffset: info.buffer_offset as u64,
            textureDesc: crate::cxx::OP_TextureDesc {
                aspectX: info.texture_desc.aspect_x,
                aspectY: info.texture_desc.aspect_y,
                depth: info.texture_desc.depth as u32,
                height: info.texture_desc.height as u32,
                width: info.texture_desc.width as u32,
                texDim: match info.texture_desc.tex_dim {
                    TexDim::EInvalid => cxx::OP_TexDim::eInvalid,
                    TexDim::E2D => cxx::OP_TexDim::e2D,
                    TexDim::E2DArray => cxx::OP_TexDim::e2DArray,
                    TexDim::E3D => cxx::OP_TexDim::e3D,
                    TexDim::ECube => cxx::OP_TexDim::eCube,
                },
                pixelFormat: (&info.texture_desc.pixel_format).into(),
                reserved: Default::default(),
            },
            firstPixel: match info.first_pixel {
                FirstPixel::BottomLeft => cxx::TOP_FirstPixel::BottomLeft,
                FirstPixel::TopLeft => cxx::TOP_FirstPixel::TopLeft,
            },
            colorBufferIndex: info.color_buffer_index as u32,
            reserved: Default::default(),
        };

        // uploadBuffer takes ownership of the buffer
        let buf = std::mem::replace(&mut buffer.buffer, UniquePtr::null());
        unsafe {
            self.output
                .as_mut()
                .uploadBuffer(buf.into_raw(), &info, std::ptr::null_mut())
        };
    }
}

pub trait TopNew {
    fn new(info: NodeInfo, context: TopContext) -> Self;
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
pub struct TopContext {
    context: Pin<&'static mut cxx::TOP_Context>,
}

impl TopContext {
    pub fn new(context: Pin<&'static mut cxx::TOP_Context>) -> Self {
        Self { context }
    }

    pub fn create_output_buffer(&mut self, size: usize, flags: TopBufferFlags) -> TopBuffer {
        let flags = match flags {
            TopBufferFlags::None => cxx::TOP_BufferFlags::None,
            TopBufferFlags::Readable => cxx::TOP_BufferFlags::Readable,
        };
        let buf = unsafe {
            self.context
                .as_mut()
                .createOutputBuffer(size as u64, flags, std::ptr::null_mut())
        };
        TopBuffer::new(buf)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum TopBufferFlags {
    #[default]
    None,
    Readable,
}

pub struct TopBuffer {
    buffer: UniquePtr<cxx::TD_OP_SmartRef_TD_TOP_Buffer_AutocxxConcrete>,
}

impl TopBuffer {
    pub fn new(buffer: UniquePtr<cxx::TD_OP_SmartRef_TD_TOP_Buffer_AutocxxConcrete>) -> Self {
        Self { buffer }
    }

    pub fn size(&self) -> usize {
        crate::cxx::getBufferSize(&self.buffer) as usize
    }

    pub fn data_mut<T>(&mut self) -> &mut [T] {
        let size = self.size();
        let data = crate::cxx::getBufferData(self.buffer.pin_mut());
        unsafe { std::slice::from_raw_parts_mut(data as *mut T, size) }
    }

    pub fn flags(&self) -> TopBufferFlags {
        let flags = crate::cxx::getBufferFlags(&self.buffer);
        match flags {
            cxx::TOP_BufferFlags::None => TopBufferFlags::None,
            cxx::TOP_BufferFlags::Readable => TopBufferFlags::Readable,
        }
    }
}

impl Drop for TopBuffer {
    fn drop(&mut self) {
        if self.buffer.is_null() {
            return;
        }
        cxx::releaseBuffer(self.buffer.pin_mut())
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
        use td_rs_top::TopContext;

        #[no_mangle]
        pub extern "C" fn top_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) -> cxx::TOP_ExecuteMode {
            unsafe {
                td_rs_top::op_info::<$plugin_ty>(op_info);
                match <$plugin_ty>::EXECUTE_MODE {
                    td_rs_top::ExecuteMode::Cuda => panic!("Cuda is not supported yet"),
                    td_rs_top::ExecuteMode::Cpu => cxx::TOP_ExecuteMode::CPUMem,
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn top_new_impl(info: NodeInfo, context: TopContext) -> Box<dyn Top> {
            op_init();
            Box::new(<$plugin_ty>::new(info, context))
        }
    };
}
