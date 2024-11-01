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

pub struct SopOutput<'cook> {
    output: Pin<&'cook mut cxx::SOP_Output>,
}

impl<'cook> SopOutput<'cook> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'cook mut cxx::SOP_Output>) -> SopOutput<'cook> {
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
        self.output
            .as_mut()
            .setNormal(normal.into().as_ref(), start_idx as i32);
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

    pub fn set_tex_coord2(&mut self, texture: &TexCoord, num_layers: usize, start_idx: usize) {
        unsafe {
            self.output.as_mut().setTexCoord(
                texture.as_ref() as *const cxx::TexCoord,
                num_layers as i32,
                start_idx as i32,
            );
        }
    }

    pub fn set_tex_coords(&mut self, textures: &[TexCoord], num_layers: usize, start_idx: usize) {
        unsafe {
            let textures = textures
                .iter()
                .map(|t| cxx::TexCoord {
                    u: t.u,
                    v: t.v,
                    w: t.w,
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let num_points = self.num_points() as i32;
            self.output.as_mut().setTexCoords(
                textures.as_ptr() as *const cxx::TexCoord,
                num_points,
                num_layers as i32,
                start_idx as i32,
            );
            Box::leak(textures);
        }
    }

    pub fn has_tex_coord(&mut self) -> bool {
        self.output.as_mut().hasTexCoord()
    }

    pub fn num_tex_coord_layers(&mut self) -> usize {
        self.output.as_mut().getNumTexCoordLayers() as usize
    }

    pub fn set_custom_attribute(&mut self, info: CustomAttributeInfo, data: CustomAttributeData, num_pts: usize) {
        unsafe {
            let name = std::ffi::CString::new(info.name).unwrap();
            let info = cxx::SOP_CustomAttribInfo {
                name: name.as_ptr(),
                numComponents: info.num_components as i32,
                attribType: (&info.attr_type).into(),
            };
            let attr = match data {
                CustomAttributeData::Float(mut data) => cxx::SOP_CustomAttribData {
                    _base: info,
                    floatData: data.as_mut_ptr(),
                    intData: std::ptr::null_mut(),
                },
                CustomAttributeData::Int(mut data) => cxx::SOP_CustomAttribData {
                    _base: info,
                    floatData: std::ptr::null_mut(),
                    intData: data.as_mut_ptr(),
                },
            };
            self.output
                .as_mut()
                .setCustomAttribute(&attr as *const SOP_CustomAttribData, num_pts as i32);
        }
    }

    pub fn has_custom_attribute(&mut self) -> bool {
        self.output.as_mut().hasCustomAttibutes()
    }

    pub fn add_triangle(&mut self, x: u32, y: u32, z: u32) {
        self.output
            .as_mut()
            .addTriangle(x as i32, y as i32, z as i32);
    }

    pub fn add_triangles(&mut self, indices: &[u32]) {
        unsafe {
            self.output
                .as_mut()
                .addTriangles(indices.as_ptr() as *const i32, (indices.len() / 3) as i32);
        }
    }

    pub fn add_particle_system(&mut self, num_pts: usize, start_idx: usize) {
        self.output
            .as_mut()
            .addParticleSystem(num_pts as i32, start_idx as i32);
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
                sizes.len() as i32,
            );
        }
    }

    pub fn num_primitives(&mut self) -> usize {
        self.output.as_mut().getNumPrimitives() as usize
    }

    pub fn set_bounding_box(&mut self, b: impl Into<BoundingBox>) {
        self.output.as_mut().setBoundingBox(&b.into());
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
            self.output
                .as_mut()
                .destroyGroup(type_.into(), name.as_ptr());
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
            self.output
                .as_mut()
                .discardFromPointGroup(autocxx::c_int(idx as std::ffi::c_int), name.as_ptr());
        }
    }

    pub fn discard_from_prim_group(&mut self, idx: usize, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.output
                .as_mut()
                .discardFromPrimGroup(autocxx::c_int(idx as std::ffi::c_int), name.as_ptr());
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

pub struct Unalloc;

pub struct ColorEnabled;
pub struct NormalEnabled;
pub struct TexCoordEnabled;

pub struct Alloc<NormalEnabled, ColorEnabled, TexCoordEnabled> {
    pub vertices: usize,
    pub indices: usize,
    pub buffer_mode: BufferMode,
    _normal: std::marker::PhantomData<NormalEnabled>,
    _color: std::marker::PhantomData<ColorEnabled>,
    _tex_coords: std::marker::PhantomData<TexCoordEnabled>,
}

pub struct Complete;

pub type AllocAll = Alloc<NormalEnabled, ColorEnabled, TexCoordEnabled>;

pub struct SopVboOutput<'cook, State> {
    pub state: State,
    output: Pin<&'cook mut cxx::SOP_VBOOutput>,
}

impl<'cook, State> SopVboOutput<'cook, State> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'cook mut cxx::SOP_VBOOutput>) -> SopVboOutput<'cook, Unalloc> {
        SopVboOutput {
            state: Unalloc,
            output,
        }
    }

    pub fn has_custom_attributes(&mut self) -> bool {
        self.output.as_mut().hasCustomAttibutes()
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
}

impl<'cook> SopVboOutput<'cook, Unalloc> {
    pub fn add_custom_attribute(&mut self, attr: CustomAttributeInfo) {
        let name = std::ffi::CString::new(attr.name).unwrap();
        let attr = cxx::SOP_CustomAttribInfo {
            name: name.as_ptr(),
            numComponents: attr.num_components as i32,
            attribType: attr.attr_type.into(),
        };
        self.output.as_mut().addCustomAttribute(&attr);
    }

    fn alloc_inner(
        &mut self,
        vertices: usize,
        indices: usize,
        enable_normal: bool,
        enable_color: bool,
        tex_coords: usize,
        buffer_mode: BufferMode,
    ) {
        if enable_color {
            self.output.as_mut().enableColor();
        }
        if enable_normal {
            self.output.as_mut().enableNormal();
        }
        if tex_coords > 0 {
            assert!(tex_coords <= 8);
            self.output.as_mut().enableTexCoord(tex_coords as i32);
        }

        self.output
            .as_mut()
            .allocVBO(vertices as i32, indices as i32, buffer_mode.into());
    }

    pub fn alloc_none(
        mut self,
        vertices: usize,
        indices: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<(), (), ()>> {
        self.alloc_inner(vertices, indices, false, false, 0, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_all(
        mut self,
        vertices: usize,
        indices: usize,
        tex_coords: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<NormalEnabled, ColorEnabled, TexCoordEnabled>> {
        self.alloc_inner(vertices, indices, true, true, tex_coords, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_normals(
        mut self,
        vertices: usize,
        indices: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<NormalEnabled, (), ()>> {
        self.alloc_inner(vertices, indices, true, false, 0, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_colors(
        mut self,
        vertices: usize,
        indices: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<(), ColorEnabled, ()>> {
        self.alloc_inner(vertices, indices, false, true, 0, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_tex_coords(
        mut self,
        vertices: usize,
        indices: usize,
        tex_coords: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<(), (), TexCoordEnabled>> {
        self.alloc_inner(vertices, indices, false, false, tex_coords, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_normal_and_colors(
        mut self,
        vertices: usize,
        indices: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<NormalEnabled, ColorEnabled, ()>> {
        self.alloc_inner(vertices, indices, true, true, 0, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_normal_and_tex_coords(
        mut self,
        vertices: usize,
        indices: usize,
        tex_coords: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<NormalEnabled, (), TexCoordEnabled>> {
        self.alloc_inner(vertices, indices, true, false, tex_coords, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }

    pub fn alloc_colors_and_tex_coords(
        mut self,
        vertices: usize,
        indices: usize,
        tex_coords: usize,
        buffer_mode: BufferMode,
    ) -> SopVboOutput<'cook, Alloc<(), ColorEnabled, TexCoordEnabled>> {
        self.alloc_inner(vertices, indices, false, true, tex_coords, buffer_mode);
        SopVboOutput {
            state: Alloc {
                vertices,
                indices,
                buffer_mode,
                _color: Default::default(),
                _normal: Default::default(),
                _tex_coords: Default::default(),
            },
            output: self.output,
        }
    }
}

impl<'cook, C, T> SopVboOutput<'cook, Alloc<NormalEnabled, C, T>> {
    pub fn normals(&mut self) -> &'cook mut [Vec3] {
        let normals = self.output.as_mut().getNormals();
        if normals.is_null() {
            panic!("normals is null")
        }
        unsafe { std::slice::from_raw_parts_mut(normals as *mut Vec3, self.state.vertices) }
    }
}

impl<'cook, N, T> SopVboOutput<'cook, Alloc<N, ColorEnabled, T>> {
    pub fn colors(&mut self) -> &'cook mut [Color] {
        let colors = self.output.as_mut().getColors();
        if colors.is_null() {
            panic!("colors is null")
        }
        unsafe { std::slice::from_raw_parts_mut(colors as *mut Color, self.state.vertices) }
    }
}

impl<'cook, N, C> SopVboOutput<'cook, Alloc<N, C, TexCoordEnabled>> {
    pub fn tex_coords(&mut self) -> &'cook mut [TexCoord] {
        let tex_coords = self.output.as_mut().getTexCoords();
        if tex_coords.is_null() {
            println!("tex_coords is null")
        }
        unsafe { std::slice::from_raw_parts_mut(tex_coords as *mut TexCoord, self.state.vertices) }
    }
    pub fn get_num_text_coord_layers(&mut self) -> usize {
        self.output.as_mut().getNumTexCoordLayers() as usize
    }
}

impl<'cook, N, C, T> SopVboOutput<'cook, Alloc<N, C, T>> {
    pub fn positions(&mut self) -> &'cook mut [Position] {
        let positions = self.output.as_mut().getPos();
        if positions.is_null() {
            panic!("positions is null")
        }
        unsafe { std::slice::from_raw_parts_mut(positions as *mut Position, self.state.vertices) }
    }
    pub fn add_triangles(&mut self, num_triangles: usize) -> &'cook mut [u32] {
        let triangles = self.output.as_mut().addTriangles(num_triangles as i32);
        unsafe { std::slice::from_raw_parts_mut(triangles as *mut u32, num_triangles * 3) }
    }
    pub fn add_particle_system(&mut self, num_particles: usize) -> &'cook mut [u32] {
        let particles = self.output.as_mut().addParticleSystem(num_particles as i32);
        unsafe { std::slice::from_raw_parts_mut(particles as *mut u32, num_particles) }
    }
    pub fn add_lines(&mut self, num_lines: usize) -> &'cook mut [u32] {
        let lines = self.output.as_mut().addLines(num_lines as i32);
        unsafe { std::slice::from_raw_parts_mut(lines as *mut u32, num_lines) }
    }
    pub fn update_complete(mut self) -> SopVboOutput<'cook, Complete> {
        self.output.as_mut().updateComplete();
        SopVboOutput {
            state: Complete,
            output: self.output,
        }
    }
    pub fn set_bounding_box(&mut self, bounds: impl Into<BoundingBox>) {
        self.output.as_mut().setBoundingBox(&bounds.into());
    }
}

#[derive(Debug, Copy, Clone)]
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

    fn execute_vbo(&mut self, _output: SopVboOutput<Unalloc>, _inputs: &OperatorInputs<SopInput>) {
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
            op_init();
            Box::new(<$plugin_ty>::new(info))
        }
    };
}
