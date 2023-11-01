use td_rs_derive::Params;
use td_rs_sop::param::ChopParam;
use td_rs_sop::*;

#[derive(Params, Default, Clone)]
struct FilterSopParams {
    translate: ChopParam,
}

/// Struct representing our SOP's state
#[derive(Default)]
pub struct FilterSop {
    warning: String,
    params: FilterSopParams,
}

impl OpNew for FilterSop {
    fn new(_info: NodeInfo) -> Self {
        Self {
            warning: "".to_string(),
            ..Default::default()
        }
    }
}

/// Impl block providing default constructor for plugin
impl FilterSop {
    fn translate(&mut self) -> Vec3 {
        if let Some(chop) = self.params.translate.input() {
            let mut t = Vec3::zero();
            let last_sample = chop.num_samples() - 1;
            let num_channels = chop.num_channels();
            if num_channels > 2 {
                t.z = chop[2][last_sample]
            } else {
                self.warning = "Translate CHOP should have at least 3 channels.".to_string();
            }

            if num_channels > 1 {
                t.y = chop[1][last_sample]
            }
            if num_channels > 0 {
                t.x = chop[0][last_sample]
            }
            t
        } else {
            self.warning = "Translate CHOP not set.".to_string();
            Vec3::zero()
        }
    }

    fn copy_points_translated(output: &mut SopOutput, input: &SopInput, t: &Vec3) {
        input.point_positions().iter().for_each(|p| {
            output.add_point(p + t);
        });
    }

    fn copy_normals(output: &mut SopOutput, input: &SopInput) {
        if !input.has_normals() {
            return;
        }

        let normals = input.normals();
        output.set_normals(normals, 0);
    }

    fn copy_colors(output: &mut SopOutput, input: &SopInput) {
        if !input.has_colors() {
            return;
        }

        let colors = input.colors();
        output.set_colors(colors, 0);
    }

    fn copy_textures(output: &mut SopOutput, input: &SopInput) {
        let (textures, num_layers) = input.textures();
        output.set_tex_coords(textures, num_layers, 0);
    }

    fn copy_custom_attributes(output: &mut SopOutput, input: &SopInput) {
        let num_points = input.num_points();
        for i in 0..input.num_custom_attributes() {
            let attr = input.custom_attribute(i);
            output.set_custom_attribute(attr, num_points);
        }
    }

    fn copy_primitives(&mut self, output: &mut SopOutput, input: &SopInput) {
        for i in 0..input.num_primitives() {
            let prim = input.primitive(i);
            let indices = prim.vertices();
            if indices.len() > 3 {
                self.warning = "Input geometry is not a triangulated polygon.".to_string();
            }

            // let mut fall_through = false;

            match indices.len() {
                1 => {
                    output.add_particle_system(1, indices[0] as usize);
                }
                2 => {
                    output.add_line(&indices[..2]);
                }
                3 => {
                    output.add_triangles(&indices[..]);
                    // fall_through = true;
                }
                _ => {}
            }

            // if indices.len() >= 3 || fall_through {
            //     let mut tmp = vec![0; indices.len() + 1];
            //     tmp[..].copy_from_slice(&indices[..]);
            //     output.add_line(&tmp);
            // }
        }
    }
}

impl OpInfo for FilterSop {
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 1;
}

impl Op for FilterSop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Sop for FilterSop {
    fn execute(&mut self, output: &mut SopOutput, inputs: &OperatorInputs<SopInput>) {
        if let Some(input) = inputs.input(0) {
            let t = self.translate();
            Self::copy_points_translated(output, input, &t);
            Self::copy_normals(output, input);
            Self::copy_colors(output, input);
            Self::copy_textures(output, input);
            Self::copy_custom_attributes(output, input);
            self.copy_primitives(output, input);
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<SopInput>) -> SopGeneralInfo {
        SopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            direct_to_gpu: false,
        }
    }
}

sop_plugin!(FilterSop);
