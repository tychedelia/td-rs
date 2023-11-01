use crate::cxx::{SOP_CustomAttribData, VBOBufferMode, Vector};

use std::pin::Pin;

pub use td_rs_base::sop::*;
pub use td_rs_base::*;

pub mod cxx;

#[derive(Debug, Default)]
pub struct SopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
    pub direct_to_gpu: bool,
}

pub struct SopOutput<'execute> {
    output: Pin<&'execute mut cxx::SOP_Output>,
}

impl<'execute> SopOutput<'execute> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::SOP_Output>) -> SopOutput<'execute> {
        Self { output }
    }

    pub fn add_point(&mut self, pos: impl Into<Position>) -> usize {
        self.output.as_mut().addPoint(&pos.into()) as usize
    }

    pub fn add_points(&mut self, positions: &[Position]) {
        unsafe {
            self.output.as_mut().addPoints(
                positions.as_ptr() as *const cxx::Position,
                positions.len() as i32,
            );
        }
    }

    pub fn num_points(&mut self) -> usize {
        self.output.as_mut().getNumPoints() as usize
    }

    pub fn set_normal(&mut self, normal: impl Into<Vec3>, start_idx: usize) {
        unsafe {
            self.output
                .as_mut()
                .setNormal(normal.into().as_ref(), start_idx as i32);
        }
    }

    pub fn set_normals(&mut self, normals: &[Vec3], start_idx: usize) {
        unsafe {
            self.output.as_mut().setNormals(
                normals.as_ptr() as *const Vector,
                normals.len() as i32,
                start_idx as i32,
            );
        }
    }

    pub fn has_normals(&mut self) -> bool {
        self.output.as_mut().hasNormal()
    }



    pub fn set_color(&mut self, color: impl Into<Color>, start_idx: usize) {
        unsafe {
            self.output.as_mut().setColor(
                &*(color.into().as_ref() as *const cxx::Color),
                start_idx as i32,
            );
        }
    }

    pub fn set_colors(&mut self, colors: &[Color], start_idx: usize) {
        unsafe {
            self.output.as_mut().setColors(
                colors.as_ptr() as *const cxx::Color,
                colors.len() as i32,
                start_idx as i32,
            );
        }
    }

    pub fn has_color(&mut self) -> bool {
        self.output.as_mut().hasColor()
    }

    pub fn set_tex_coord(
        &mut self,
        texture: impl Into<TexCoord>,
        num_layers: usize,
        start_idx: usize,
    ) {
        unsafe {
            self.output.as_mut().setTexCoord(
                &*(texture.into().as_ref() as *const cxx::TexCoord),
                num_layers as i32,
                start_idx as i32,
            );
        }
    }

    pub fn set_tex_coords(&mut self, textures: &[TexCoord], num_layers: usize, start_idx: usize) {
        unsafe {
            self.output.as_mut().setTexCoords(
                textures.as_ptr() as *const cxx::TexCoord,
                textures.len() as i32,
                num_layers as i32,
                start_idx as i32,
            );
        }
    }

    pub fn has_tex_coord(&mut self) -> bool {
        self.output.as_mut().hasTexCoord()
    }

    pub fn num_tex_coord_layers(&mut self) -> usize {
        self.output.as_mut().getNumTexCoordLayers() as usize
    }



    pub fn set_custom_attribute(&mut self, attr: &CustomAttributeData, num_pts: usize) {
        unsafe {
            let attr: *const CustomAttributeData = attr;
            self.output
                .as_mut()
                .setCustomAttribute(attr as *const SOP_CustomAttribData, num_pts as i32);
        }
    }

    pub fn has_custom_attribute(&mut self) -> bool {
        self.output.as_mut().hasCustomAttibutes()
    }


    pub fn add_triangle(&mut self, x: u32, y: u32, z: u32) {
        unsafe {
            self.output
                .as_mut()
                .addTriangle(x as i32, y as i32, z as i32);
        }
    }

    pub fn add_triangles(&mut self, indices: &[u32]) {
        unsafe {
            self.output
                .as_mut()
                .addTriangles(indices.as_ptr() as *const i32, indices.len() as i32);
        }
    }

    pub fn add_particle_system(&mut self, num_pts: usize, start_idx: usize) {
        unsafe {
            self.output
                .as_mut()
                .addParticleSystem(num_pts as i32, start_idx as i32);
        }
    }

    pub fn add_line(&mut self, indices: &[u32]) {
        unsafe {
            self.output
                .as_mut()
                .addLine(indices.as_ptr() as *const i32, indices.len() as i32);
        }
    }

    pub fn add_lines(&mut self, indices: &[u32], sizes: &[u32]) {
        unsafe {
            self.output.as_mut().addLines(
                indices.as_ptr() as *const i32,
                sizes.as_ptr() as *const i32 as *mut i32,
                indices.len() as i32,
            );
        }
    }

    pub fn num_primitives(&mut self) -> usize {
        self.output.as_mut().getNumPrimitives() as usize
    }

    pub fn set_bounding_box(&mut self, b: impl Into<BoundingBox>) {
        unsafe {
            self.output.as_mut().setBoundingBox(&b.into());
        }
    }

    pub fn add_group(&mut self, type_: GroupType, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output.as_mut().addGroup(&type_.into(), name.as_ptr());
        }
    }

    pub fn destroy_group(&mut self, type_: GroupType, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output.as_mut().destroyGroup(type_.into(), name.as_ptr());
        }
    }

    pub fn add_point_to_group(&mut self, point: usize, group: &str) {
        let group = std::ffi::CString::new(group).unwrap();
        unsafe {
            self.output
                .as_mut()
                .addPointToGroup(autocxx::c_int(point as std::ffi::c_int), group.as_ptr());
        }
    }

    pub fn add_prim_to_group(&mut self, prim: usize, group: &str) {
        let group = std::ffi::CString::new(group).unwrap();
        unsafe {
            self.output
                .as_mut()
                .addPrimToGroup(autocxx::c_int(prim as std::ffi::c_int), group.as_ptr());
        }
    }

    pub fn add_to_group(&mut self, idx: usize, type_: GroupType, group: &str) {
        let group = std::ffi::CString::new(group).unwrap();
        unsafe {
            self.output.as_mut().addToGroup(
                autocxx::c_int(idx as std::ffi::c_int),
                type_.into(),
                group.as_ptr(),
            );
        }
    }

    pub fn discard_from_point_group(&mut self, idx: usize, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output.as_mut().discardFromPointGroup(
                autocxx::c_int(idx as std::ffi::c_int),
                name.as_ptr(),
            );
        }
    }

    pub fn discard_from_prim_group(&mut self, idx: usize, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output.as_mut().discardFromPrimGroup(
                autocxx::c_int(idx as std::ffi::c_int),
                name.as_ptr(),
            );
        }
    }

    pub fn discard_from_group(&mut self, idx: usize, type_: GroupType, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output.as_mut().discardFromGroup(
                autocxx::c_int(idx as std::ffi::c_int),
                type_.into(),
                name.as_ptr(),
            );
        }
    }
}

#[derive(Debug)]
pub enum GroupType {
    Point,
    Primitive,
}

impl From<GroupType> for &cxx::SOP_GroupType {
    fn from(t: GroupType) -> Self {
        match t {
            GroupType::Point => &cxx::SOP_GroupType::Point,
            GroupType::Primitive => &cxx::SOP_GroupType::Primitive,
        }
    }
}

impl From<GroupType> for cxx::SOP_GroupType {
    fn from(t: GroupType) -> Self {
        match t {
            GroupType::Point => cxx::SOP_GroupType::Point,
            GroupType::Primitive => cxx::SOP_GroupType::Primitive,
        }
    }
}

impl From<cxx::SOP_GroupType> for GroupType {
    fn from(t: cxx::SOP_GroupType) -> Self {
        match t {
            cxx::SOP_GroupType::Point => GroupType::Point,
            cxx::SOP_GroupType::Primitive => GroupType::Primitive,
        }
    }
}

pub struct SopVboOutput<'execute> {
    num_vertices: Option<usize>,
    output: Pin<&'execute mut cxx::SOP_VBOOutput>,
}

impl<'execute> SopVboOutput<'execute> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::SOP_VBOOutput>) -> SopVboOutput<'execute> {
        Self {
            num_vertices: None,
            output,
        }
    }

    pub fn enable_normal(&mut self) {
        self.output.as_mut().enableNormal();
    }

    pub fn enable_color(&mut self) {
        self.output.as_mut().enableColor();
    }

    pub fn enable_text_coord(&mut self, num_layers: usize) {
        self.output.as_mut().enableTexCoord(num_layers as i32);
    }

    pub fn has_normal(&mut self) -> bool {
        self.output.as_mut().hasNormal()
    }

    pub fn has_color(&mut self) -> bool {
        self.output.as_mut().hasColor()
    }

    pub fn has_tex_coord(&mut self) -> bool {
        self.output.as_mut().hasTexCoord()
    }

    pub fn has_custom_attributes(&mut self) -> bool {
        self.output.as_mut().hasCustomAttibutes()
    }

    pub fn add_custom_attribute(&mut self, attr: CustomAttributeInfo) {
        let name = std::ffi::CString::new(attr.name).unwrap();
        let attr = cxx::SOP_CustomAttribInfo {
            name: name.as_ptr(),
            numComponents: attr.num_components as i32,
            attribType: attr.attr_type.into(),
        };
        self.output.as_mut().addCustomAttribute(&attr);
    }

    pub fn alloc_vbo(&mut self, num_vertices: usize, num_indices: usize, buffer_mode: BufferMode) {
        self.num_vertices = Some(num_vertices);
        self.output
            .as_mut()
            .allocVBO(num_vertices as i32, num_indices as i32, buffer_mode.into());
    }

    pub fn get_pos(&mut self) -> &[Vec3] {
        if let Some(num_vertices) = self.num_vertices {
            let pos = unsafe { self.output.as_mut().getPos() };
            unsafe { std::slice::from_raw_parts(pos as *const Vec3, num_vertices) }
        } else {
            panic!("Must call alloc_vbo first!!")
        }
    }

    pub fn get_normals(&mut self) -> &[Vec3] {
        if let Some(num_vertices) = self.num_vertices {
            let normals = unsafe { self.output.as_mut().getNormals() };
            unsafe { std::slice::from_raw_parts(normals as *const Vec3, num_vertices) }
        } else {
            panic!("Must call alloc_vbo first!!")
        }
    }

    pub fn get_colors(&mut self) -> &[Color] {
        if let Some(num_vertices) = self.num_vertices {
            let colors = unsafe { self.output.as_mut().getColors() };
            unsafe { std::slice::from_raw_parts(colors as *const Color, num_vertices) }
        } else {
            panic!("Must call alloc_vbo first!!")
        }
    }

    pub fn get_tex_coords(&mut self) -> &[TexCoord] {
        if let Some(num_vertices) = self.num_vertices {
            let tex_coords = unsafe { self.output.as_mut().getTexCoords() };
            unsafe { std::slice::from_raw_parts(tex_coords as *const TexCoord, num_vertices) }
        } else {
            panic!("Must call alloc_vbo first!!")
        }
    }

    pub fn get_num_text_coord_layers(&mut self) -> usize {
        unsafe { self.output.as_mut().getNumTexCoordLayers() as usize }
    }

    pub fn add_triangles(&mut self, num_triangles: usize) {
        self.output.as_mut().addTriangles(num_triangles as i32);
    }

    pub fn add_particle_system(&mut self, num_particles: usize) {
        self.output.as_mut().addParticleSystem(num_particles as i32);
    }

    pub fn add_lines(&mut self, num_lines: usize) {
        self.output.as_mut().addLines(num_lines as i32);
    }

    pub fn update_complete(&mut self) {
        self.output.as_mut().updateComplete();
    }

    pub fn set_bounding_box(&mut self, bounds: impl Into<BoundingBox>) {
        self.output.as_mut().setBoundingBox(&bounds.into());
    }
}

pub enum BufferMode {
    Static,
    Dynamic,
}

impl From<VBOBufferMode> for BufferMode {
    fn from(value: VBOBufferMode) -> Self {
        match value {
            VBOBufferMode::Static => BufferMode::Static,
            VBOBufferMode::Dynamic => BufferMode::Dynamic,
        }
    }
}

impl From<BufferMode> for VBOBufferMode {
    fn from(value: BufferMode) -> Self {
        match value {
            BufferMode::Static => VBOBufferMode::Static,
            BufferMode::Dynamic => VBOBufferMode::Dynamic,
        }
    }
}

/// Trait for defining a custom operator.
pub trait Sop: Op {
    fn general_info(&self, _input: &OperatorInputs<SopInput>) -> SopGeneralInfo {
        SopGeneralInfo::default()
    }

    fn execute(&mut self, _output: &mut SopOutput, _inputs: &OperatorInputs<SopInput>) {
        // Do nothing by default.
    }

    fn execute_vbo(&mut self, _output: &mut SopVboOutput, _inputs: &OperatorInputs<SopInput>) {
        // Do nothing by default.
    }
}

#[macro_export]
macro_rules! sop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_sop::cxx::c_void;
        use td_rs_sop::cxx::OP_CustomOPInfo;
        use td_rs_sop::NodeInfo;

        #[no_mangle]
        pub extern "C" fn sop_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) {
            unsafe {
                td_rs_sop::op_info::<$plugin_ty>(op_info);
            }
        }

        #[no_mangle]
        pub extern "C" fn sop_new_impl(info: NodeInfo) -> Box<dyn Sop> {
            Box::new(<$plugin_ty>::new(info))
        }
    };
}
