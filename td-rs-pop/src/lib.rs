pub mod cxx;
pub mod prelude;

pub use ::cxx::UniquePtr;
pub use autocxx::prelude::*;
use std::ffi::CString;
use std::pin::Pin;
pub use td_rs_base::pop::*;
pub use td_rs_base::*;

#[derive(Debug, Default)]
pub struct PopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum BufferMode {
    #[default]
    SequentialWrite = 0,
    ReadWrite = 1,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum BufferUsage {
    #[default]
    Attribute = 0,
    IndexBuffer = 1,
    PointInfoBuffer = 2,
    TopologyInfoBuffer = 3,
    LineStripsInfoBuffer = 4,
    GridInfoBuffer = 5,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum AttributeType {
    #[default]
    Float = 0,
    Double = 1,
    Int32 = 2,
    UInt32 = 3,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum AttributeQualifier {
    #[default]
    None = 0,
    Direction = 1,
    TransformMatrix = 2,
    Color = 3,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum AttributeClass {
    Vertex = 0,
    #[default]
    Point = 1,
    Primitive = 2,
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub name: String,
    pub num_components: u32,
    pub num_columns: u32,
    pub array_size: u32,
    pub attribute_type: AttributeType,
    pub qualifier: AttributeQualifier,
    pub class: AttributeClass,
}

impl Default for AttributeInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            num_components: 4,
            num_columns: 1,
            array_size: 0,
            attribute_type: AttributeType::Float,
            qualifier: AttributeQualifier::None,
            class: AttributeClass::Point,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TopologyInfo {
    pub triangles_start_index: u32,
    pub triangles_count: u32,
    pub quads_start_index: u32,
    pub quads_count: u32,
    pub line_strips_start_index: u32,
    pub line_strips_count: u32,
    pub line_strips_num_vertices: u32,
    pub lines_start_index: u32,
    pub lines_count: u32,
    pub point_primitives_start_index: u32,
    pub point_primitives_count: u32,
}

pub struct PopBuffer {
    buffer: UniquePtr<cxx::TD_OP_SmartRef_TD_POP_Buffer_AutocxxConcrete>,
}

impl PopBuffer {
    pub fn size(&self) -> usize {
        cxx::popBufferSize(&self.buffer) as usize
    }

    pub fn data_mut<T>(&mut self) -> &mut [T] {
        let size = self.size() / std::mem::size_of::<T>();
        let data = cxx::popBufferData(self.buffer.pin_mut());
        unsafe { std::slice::from_raw_parts_mut(data as *mut T, size) }
    }

    fn into_raw(mut self) -> *mut cxx::TD_OP_SmartRef_TD_POP_Buffer_AutocxxConcrete {
        std::mem::replace(&mut self.buffer, UniquePtr::null()).into_raw()
    }
}

impl Drop for PopBuffer {
    fn drop(&mut self) {
        if self.buffer.is_null() {
            return;
        }
        cxx::popReleaseBuffer(self.buffer.pin_mut());
    }
}

#[derive(Default)]
pub struct InfoBuffers {
    pub point_info: Option<PopBuffer>,
    pub topology_info: Option<PopBuffer>,
    pub line_strips_info: Option<PopBuffer>,
    pub line_strips_prim_indices: Option<PopBuffer>,
    pub grid_info: Option<PopBuffer>,
}

pub struct PopContext {
    context: Pin<&'static mut cxx::POP_Context>,
}

impl PopContext {
    pub fn new(context: Pin<&'static mut cxx::POP_Context>) -> Self {
        Self { context }
    }

    pub fn create_buffer(
        &mut self,
        size: usize,
        mode: BufferMode,
        usage: BufferUsage,
    ) -> Option<PopBuffer> {
        let buffer = unsafe {
            cxx::popCreateBuffer(
                self.context.as_mut(),
                size as u64,
                mode as i32,
                usage as i32,
            )
        };
        let buffer = PopBuffer { buffer };
        if buffer.size() == 0 {
            None
        } else {
            Some(buffer)
        }
    }

    pub fn create_point_info_buffer(&mut self, num_points: u32) -> Option<PopBuffer> {
        let mut buffer = self.create_buffer(
            cxx::popPointInfoSize() as usize,
            BufferMode::ReadWrite,
            BufferUsage::PointInfoBuffer,
        )?;
        cxx::popWritePointInfo(buffer.buffer.pin_mut(), num_points);
        Some(buffer)
    }

    pub fn create_topology_info_buffer(&mut self, topology: &TopologyInfo) -> Option<PopBuffer> {
        let mut buffer = self.create_buffer(
            cxx::popTopologyInfoSize() as usize,
            BufferMode::ReadWrite,
            BufferUsage::TopologyInfoBuffer,
        )?;
        cxx::popWriteTopologyInfo(
            buffer.buffer.pin_mut(),
            topology.triangles_start_index,
            topology.triangles_count,
            topology.quads_start_index,
            topology.quads_count,
            topology.line_strips_start_index,
            topology.line_strips_count,
            topology.line_strips_num_vertices,
            topology.lines_start_index,
            topology.lines_count,
            topology.point_primitives_start_index,
            topology.point_primitives_count,
        );
        Some(buffer)
    }
}

pub struct PopOutput<'cook> {
    output: Pin<&'cook mut cxx::POP_Output>,
}

impl<'cook> PopOutput<'cook> {
    pub fn new(output: Pin<&'cook mut cxx::POP_Output>) -> PopOutput<'cook> {
        Self { output }
    }

    pub fn set_attribute(&mut self, buffer: PopBuffer, info: &AttributeInfo) {
        let name = CString::new(info.name.as_str()).unwrap();
        unsafe {
            cxx::popSetAttribute(
                self.output.as_mut(),
                buffer.into_raw(),
                name.as_ptr(),
                info.num_components,
                info.num_columns,
                info.array_size,
                info.attribute_type as i32,
                info.qualifier as i32,
                info.class as i32,
            );
        }
    }

    pub fn set_index_buffer(&mut self, buffer: PopBuffer) {
        unsafe {
            cxx::popSetIndexBuffer(self.output.as_mut(), buffer.into_raw());
        }
    }

    pub fn set_info_buffers(&mut self, buffers: InfoBuffers) {
        fn raw(
            buffer: Option<PopBuffer>,
        ) -> *mut cxx::TD_OP_SmartRef_TD_POP_Buffer_AutocxxConcrete {
            buffer.map_or(std::ptr::null_mut(), PopBuffer::into_raw)
        }
        unsafe {
            cxx::popSetInfoBuffers(
                self.output.as_mut(),
                raw(buffers.point_info),
                raw(buffers.topology_info),
                raw(buffers.line_strips_info),
                raw(buffers.line_strips_prim_indices),
                raw(buffers.grid_info),
            );
        }
    }
}

pub trait PopNew {
    fn new(info: NodeInfo, context: PopContext) -> Self;
}

pub trait Pop: Op {
    fn general_info(&self, _input: &OperatorInputs<PopInput>) -> PopGeneralInfo {
        PopGeneralInfo::default()
    }

    fn execute(&mut self, _output: &mut PopOutput, _inputs: &OperatorInputs<PopInput>) {}

    fn build_dynamic_menu(
        &mut self,
        _inputs: &OperatorInputs<PopInput>,
        _menu_info: &mut DynamicMenuInfo,
    ) {
    }
}

#[macro_export]
macro_rules! pop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_pop::cxx::c_void;
        use td_rs_pop::cxx::OP_CustomOPInfo;
        use td_rs_pop::NodeInfo;
        use td_rs_pop::PopContext;

        #[no_mangle]
        pub extern "C" fn pop_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) {
            unsafe {
                td_rs_pop::op_info::<$plugin_ty>(op_info);
            }
        }

        #[no_mangle]
        pub extern "C" fn pop_new_impl(info: NodeInfo, context: PopContext) -> Box<dyn Pop> {
            op_init();
            Box::new(<$plugin_ty>::new(info, context))
        }
    };
}
