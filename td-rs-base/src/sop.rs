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

    pub fn custom_attribute(&self, index: usize) -> &CustomAttributeData {
        unsafe {
            let custom_attribute = self.input.getCustomAttribute(index as i32);
            CustomAttributeData::ref_cast(&*custom_attribute)
        }
    }

    pub fn num_primitives(&self) -> usize {
        self.input.getNumPrimitives() as usize
    }

    pub fn num_vertices(&self) -> usize {
        self.input.getNumVertices() as usize
    }

    pub fn primitive(&self, index: usize) -> PrimitiveInfo {
        let info = self.input.getPrimitive(index as i32);
        PrimitiveInfo(info)
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct PrimitiveInfo(cxx::SOP_PrimitiveInfo);

impl PrimitiveInfo {
    pub fn vertices(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(self.pointIndices as *const u32, self.numVertices as usize)
        }
    }
}

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct CustomAttributeData(cxx::SOP_CustomAttribData);

impl<'execute> GetInput<'execute, SopInput> for OperatorInputs<'execute, SopInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'execute SopInput> {
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
    pub fn zero() -> Self {
        Self(cxx::Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
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

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct Position(cxx::Position);

impl_op_ex!(+ |a: &Position, b: &Vec3| -> Position {
    Position(cxx::Position {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    })
});

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct Color(cxx::Color);

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct TexCoord(cxx::TexCoord);

#[derive(RefCast, Deref, DerefMut, AsRef, From, Into)]
#[repr(transparent)]
pub struct BoundingBox(cxx::BoundingBox);
