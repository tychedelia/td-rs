#![allow(non_snake_case)]

use crate::cxx::PrimitiveType;
use crate::{cxx, GetInput, OperatorInputs};
use auto_ops::impl_op_ex;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use ref_cast::RefCast;

/// A sop input.
#[repr(transparent)]
#[derive(RefCast)]
pub struct SopInput {
    input: cxx::OP_SOPInput,
}

impl SopInput {
    pub fn point_positions(&self) -> &[Position] {
        let num_points = self.num_points();
        unsafe {
            std::slice::from_raw_parts(
                Position::ref_cast(&*self.input.getPointPositions()),
                num_points,
            )
        }
    }

    pub fn num_points(&self) -> usize {
        self.input.getNumPoints() as usize
    }

    pub fn has_normals(&self) -> bool {
        self.input.hasNormals()
    }

    pub fn normals(&self) -> &[Vec3] {
        unsafe {
            let normals = self.input.getNormals();
            let num_normals = (*normals).numNormals;
            let normals = (*normals).normals;

            std::slice::from_raw_parts(Vec3::ref_cast(&*normals), num_normals as usize)
        }
    }

    pub fn has_colors(&self) -> bool {
        self.input.hasColors()
    }

    pub fn colors(&self) -> &[Color] {
        unsafe {
            let colors = self.input.getColors();
            let num_colors = (*colors).numColors;
            let colors = (*colors).colors;

            std::slice::from_raw_parts(Color::ref_cast(&*colors), num_colors as usize)
        }
    }

    pub fn textures(&self) -> (&[TexCoord], usize) {
        unsafe {
            let textures = self.input.getTextures();
            let num_textures = (*textures).numTextureLayers;
            let textures = (*textures).textures;
            let textures =
                std::slice::from_raw_parts(TexCoord::ref_cast(&*textures), num_textures as usize);
            (textures, num_textures as usize)
        }
    }

    pub fn num_custom_attributes(&self) -> usize {
        self.input.getNumCustomAttributes() as usize
    }

    pub fn custom_attribute(&self, index: usize) -> (CustomAttributeInfo, CustomAttributeData) {
        unsafe {
            let custom_attribute = self.input.getCustomAttribute(index as i32);
            if custom_attribute.is_null() {
                panic!("Custom attribute is null");
            }
            let custom_attribute = &*custom_attribute;

            let name = custom_attribute._base.name;
            let name = std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned();
            let num_components = custom_attribute._base.numComponents as usize;
            let attr_type = &custom_attribute._base.attribType;
            let attr_type = attr_type.into();
            let info = CustomAttributeInfo {
                name,
                num_components,
                attr_type,
            };
            let data = match info.attr_type {
                AttributeType::Float => {
                    let data = std::slice::from_raw_parts(
                        custom_attribute.floatData as *const f32,
                        num_components * self.num_points(),
                    );
                    CustomAttributeData::Float(data.to_vec())
                }
                AttributeType::Int => {
                    let data = std::slice::from_raw_parts(
                        custom_attribute.floatData as *const i32,
                        num_components * self.num_points(),
                    );
                    CustomAttributeData::Int(data.to_vec())
                }
            };
            (info, data)
        }
    }

    pub fn custom_attributes(&self) -> impl Iterator<Item = (CustomAttributeInfo, CustomAttributeData)> + '_ {
        let num_custom_attributes = self.num_custom_attributes();
        (0..num_custom_attributes).map(move |i| self.custom_attribute(i))
    }

    pub fn num_primitives(&self) -> usize {
        self.input.getNumPrimitives() as usize
    }

    pub fn primitive(&self, index: usize) -> PrimitiveInfo {
        let info = self.input.getPrimitive(index as i32);
        PrimitiveInfo(info)
    }

    pub fn primitives(&self) -> impl Iterator<Item = PrimitiveInfo> + '_ {
        let num_primitives = self.num_primitives();
        (0..num_primitives).map(move |i| self.primitive(i))
    }

    pub fn num_vertices(&self) -> usize {
        self.input.getNumVertices() as usize
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct PrimitiveInfo<'a>(&'a cxx::SOP_PrimitiveInfo);

impl<'a> PrimitiveInfo<'a> {
    pub fn vertices(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(self.pointIndices as *const u32, self.numVertices as usize)
        }
    }

    pub fn point_indices(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(self.pointIndices as *const u32, self.numVertices as usize)
        }
    }

    pub fn point_indices_offset(&self) -> usize {
        self.pointIndicesOffset as usize
    }

    pub fn primitive_type(&self) -> &PrimitiveType {
        &self.type_
    }

    pub fn is_closed(&self) -> bool {
        self.isClosed
    }
}

#[derive(Debug)]
pub enum AttributeType {
    Float,
    Int,
}

impl From<cxx::AttribType> for AttributeType {
    fn from(t: cxx::AttribType) -> Self {
        match t {
            cxx::AttribType::Float => AttributeType::Float,
            cxx::AttribType::Int => AttributeType::Int,
        }
    }
}

impl From<&cxx::AttribType> for AttributeType {
    fn from(t: &cxx::AttribType) -> Self {
        match t {
            cxx::AttribType::Float => AttributeType::Float,
            cxx::AttribType::Int => AttributeType::Int,
        }
    }
}

impl From<AttributeType> for cxx::AttribType {
    fn from(t: AttributeType) -> Self {
        match t {
            AttributeType::Float => cxx::AttribType::Float,
            AttributeType::Int => cxx::AttribType::Int,
        }
    }
}

impl From<&AttributeType> for cxx::AttribType {
    fn from(t: &AttributeType) -> Self {
        match t {
            AttributeType::Float => cxx::AttribType::Float,
            AttributeType::Int => cxx::AttribType::Int,
        }
    }
}

#[derive(Debug)]
pub struct CustomAttributeInfo {
    pub name: String,
    pub num_components: usize,
    pub attr_type: AttributeType,
}

#[derive(Debug)]
pub enum CustomAttributeData {
    Float(Vec<f32>),
    Int(Vec<i32>),
}

impl<'cook> GetInput<'cook, SopInput> for OperatorInputs<'cook, SopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'cook SopInput> {
        let input = self.inputs.getInputSOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(SopInput::ref_cast(unsafe { &*input }))
        }
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct Vec3(cxx::Vector);

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(cxx::Vector { x, y, z })
    }

    pub fn zero() -> Self {
        Self(cxx::Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self(cxx::Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        })
    }
}

impl From<&Vec3> for Vec3 {
    fn from(v: &Vec3) -> Self {
        Self(cxx::Vector {
            x: v.x,
            y: v.y,
            z: v.z,
        })
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self(cxx::Vector { x, y, z })
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(cxx::Vector {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        })
    }
}

impl From<(f32, f32, f64)> for Vec3 {
    fn from((x, y, z): (f32, f32, f64)) -> Self {
        Self(cxx::Vector { x, y, z: z as f32 })
    }
}

impl From<(f32, f64, f32)> for Vec3 {
    fn from((x, y, z): (f32, f64, f32)) -> Self {
        Self(cxx::Vector { x, y: y as f32, z })
    }
}

impl From<(f64, f32, f32)> for Vec3 {
    fn from((x, y, z): (f64, f32, f32)) -> Self {
        Self(cxx::Vector { x: x as f32, y, z })
    }
}

impl From<(f64, f64, f32)> for Vec3 {
    fn from((x, y, z): (f64, f64, f32)) -> Self {
        Self(cxx::Vector {
            x: x as f32,
            y: y as f32,
            z,
        })
    }
}

impl From<(f64, f32, f64)> for Vec3 {
    fn from((x, y, z): (f64, f32, f64)) -> Self {
        Self(cxx::Vector {
            x: x as f32,
            y,
            z: z as f32,
        })
    }
}

impl From<(f32, f64, f64)> for Vec3 {
    fn from((x, y, z): (f32, f64, f64)) -> Self {
        Self(cxx::Vector {
            x,
            y: y as f32,
            z: z as f32,
        })
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3(cxx::Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    })
});
impl_op_ex!(*|a: &Vec3, b: f32| -> Vec3 {
    Vec3(cxx::Vector {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    })
});

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct Position(cxx::Position);

impl Position {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(cxx::Position { x, y, z })
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self(cxx::Position {
            x: self.x,
            y: self.y,
            z: self.z,
        })
    }
}

impl From<&Position> for Position {
    fn from(p: &Position) -> Self {
        Self(cxx::Position {
            x: p.x,
            y: p.y,
            z: p.z,
        })
    }
}

impl From<(f32, f32, f32)> for Position {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self(cxx::Position { x, y, z })
    }
}

impl From<(f64, f64, f64)> for Position {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(cxx::Position {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        })
    }
}

impl From<(f32, f32, f64)> for Position {
    fn from((x, y, z): (f32, f32, f64)) -> Self {
        Self(cxx::Position { x, y, z: z as f32 })
    }
}

impl From<(f32, f64, f32)> for Position {
    fn from((x, y, z): (f32, f64, f32)) -> Self {
        Self(cxx::Position { x, y: y as f32, z })
    }
}

impl From<(f64, f32, f32)> for Position {
    fn from((x, y, z): (f64, f32, f32)) -> Self {
        Self(cxx::Position { x: x as f32, y, z })
    }
}

impl From<(f64, f64, f32)> for Position {
    fn from((x, y, z): (f64, f64, f32)) -> Self {
        Self(cxx::Position {
            x: x as f32,
            y: y as f32,
            z,
        })
    }
}

impl From<(f64, f32, f64)> for Position {
    fn from((x, y, z): (f64, f32, f64)) -> Self {
        Self(cxx::Position {
            x: x as f32,
            y,
            z: z as f32,
        })
    }
}

impl From<(f32, f64, f64)> for Position {
    fn from((x, y, z): (f32, f64, f64)) -> Self {
        Self(cxx::Position {
            x,
            y: y as f32,
            z: z as f32,
        })
    }
}

impl_op_ex!(+ |a: &Position, b: &Vec3| -> Position {
    Position(cxx::Position {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    })
});
impl_op_ex!(*|a: &Position, b: f32| -> Position {
    Position(cxx::Position {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    })
});

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct Color(cxx::Color);

impl Clone for Color {
    fn clone(&self) -> Self {
        Self(cxx::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        })
    }
}

impl From<&Color> for Color {
    fn from(c: &Color) -> Self {
        Self(cxx::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        })
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self(cxx::Color { r, g, b, a })
    }
}

impl From<(f64, f64, f64, f64)> for Color {
    fn from((r, g, b, a): (f64, f64, f64, f64)) -> Self {
        Self(cxx::Color {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        })
    }
}

impl From<(u32, u32, u32, u32)> for Color {
    fn from((r, g, b, a): (u32, u32, u32, u32)) -> Self {
        Self(cxx::Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from((r, g, b, a): (i32, i32, i32, i32)) -> Self {
        Self(cxx::Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct TexCoord(cxx::TexCoord);

impl TexCoord {
    pub const fn new(u: f32, v: f32, w: f32) -> Self {
        Self(cxx::TexCoord { u, v, w })
    }
}

impl Clone for TexCoord {
    fn clone(&self) -> Self {
        Self(cxx::TexCoord {
            u: self.u,
            v: self.v,
            w: self.w,
        })
    }
}

impl From<&TexCoord> for TexCoord {
    fn from(t: &TexCoord) -> Self {
        Self(cxx::TexCoord {
            u: t.u,
            v: t.v,
            w: t.w,
        })
    }
}

impl From<(f32, f32, f32)> for TexCoord {
    fn from((u, v, w): (f32, f32, f32)) -> Self {
        Self(cxx::TexCoord { u, v, w })
    }
}

impl From<(f64, f64, f64)> for TexCoord {
    fn from((u, v, w): (f64, f64, f64)) -> Self {
        Self(cxx::TexCoord {
            u: u as f32,
            v: v as f32,
            w: w as f32,
        })
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct BoundingBox(cxx::BoundingBox);

impl From<&BoundingBox> for BoundingBox {
    fn from(b: &BoundingBox) -> Self {
        Self(cxx::BoundingBox {
            minX: b.minX,
            minY: b.minY,
            minZ: b.minZ,
            maxX: b.maxX,
            maxY: b.maxY,
            maxZ: b.maxZ,
        })
    }
}

impl From<(f32, f32, f32, f32, f32, f32)> for BoundingBox {
    fn from((minX, minY, minZ, maxX, maxY, maxZ): (f32, f32, f32, f32, f32, f32)) -> Self {
        Self(cxx::BoundingBox {
            minX,
            minY,
            minZ,
            maxX,
            maxY,
            maxZ,
        })
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for BoundingBox {
    fn from((minX, minY, minZ, maxX, maxY, maxZ): (f64, f64, f64, f64, f64, f64)) -> Self {
        Self(cxx::BoundingBox {
            minX: minX as f32,
            minY: minY as f32,
            minZ: minZ as f32,
            maxX: maxX as f32,
            maxY: maxY as f32,
            maxZ: maxZ as f32,
        })
    }
}

impl From<(i32, i32, i32, i32, i32, i32)> for BoundingBox {
    fn from((minX, minY, minZ, maxX, maxY, maxZ): (i32, i32, i32, i32, i32, i32)) -> Self {
        Self(cxx::BoundingBox {
            minX: minX as f32,
            minY: minY as f32,
            minZ: minZ as f32,
            maxX: maxX as f32,
            maxY: maxY as f32,
            maxZ: maxZ as f32,
        })
    }
}
