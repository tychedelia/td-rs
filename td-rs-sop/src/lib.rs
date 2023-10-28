use crate::cxx::{SOP_CustomAttribData, Vector};

use std::pin::Pin;

pub use td_rs_base::param::OperatorParams;
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

    pub fn add_point(&mut self, pos: &Position) -> usize {
        self.output.as_mut().addPoint(pos) as usize
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

    pub fn add_triangles(&mut self, indices: &[u32]) {
        unsafe {
            self.output
                .as_mut()
                .addTriangles(indices.as_ptr() as *const i32, indices.len() as i32);
        }
    }

    pub fn num_points(&mut self) -> usize {
        self.output.as_mut().getNumPoints() as usize
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

    pub fn set_colors(&mut self, colors: &[Color], start_idx: usize) {
        unsafe {
            self.output.as_mut().setColors(
                colors.as_ptr() as *const cxx::Color,
                colors.len() as i32,
                start_idx as i32,
            );
        }
    }

    pub fn set_textures(&mut self, textures: &[TexCoord], num_layers: usize, start_idx: usize) {
        unsafe {
            self.output.as_mut().setTexCoords(
                textures.as_ptr() as *const cxx::TexCoord,
                textures.len() as i32,
                num_layers as i32,
                start_idx as i32,
            );
        }
    }

    pub fn set_custom_attribute(&mut self, attr: &CustomAttributeData, num_pts: usize) {
        unsafe {
            let attr: *const CustomAttributeData = attr;
            self.output
                .as_mut()
                .setCustomAttribute(attr as *const SOP_CustomAttribData, num_pts as i32);
        }
    }
}

pub struct SopVboOutput<'execute> {
    output: Pin<&'execute mut cxx::SOP_VBOOutput>,
}

impl<'execute> SopVboOutput<'execute> {
    /// Create a new `SopOutput` from a pinning reference to a
    /// `SopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::SOP_VBOOutput>) -> SopVboOutput<'execute> {
        Self { output }
    }
}

/// Trait for defining a custom operator.
pub trait Sop: Op {
    fn general_info(&self, _input: &OperatorInputs<SopInput>) -> SopGeneralInfo {
        SopGeneralInfo::default()
    }

    fn execute(&mut self, _output: &mut SopOutput, _input: &OperatorInputs<SopInput>) {
        // Do nothing by default.
    }

    fn execute_vbo(&mut self, _output: &mut SopVboOutput, _input: &OperatorInputs<SopInput>) {
        // Do nothing by default.
    }
}

#[macro_export]
macro_rules! sop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_sop::cxx::c_void;
        use td_rs_sop::cxx::OP_CustomOPInfo;

        #[no_mangle]
        pub extern "C" fn sop_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) {
            unsafe {
                td_rs_sop::op_info::<$plugin_ty>(op_info);
            }
        }

        #[no_mangle]
        pub extern "C" fn sop_new_impl() -> Box<dyn Sop> {
            Box::new(<$plugin_ty>::new())
        }
    };
}
