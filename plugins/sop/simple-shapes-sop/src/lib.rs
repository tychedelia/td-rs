#![feature(let_chains)]

use td_rs_derive::{Param, Params};
use td_rs_sop::*;

#[derive(Param, Debug, Default)]
enum Shape {
    #[default]
    Cube,
    Triangle,
    Line,
}

#[derive(Params, Debug, Default)]
struct SimpleShapesSopParams {
    #[param(label = "Chop")]
    chop: ChopParam,
    #[param(label = "Scale")]
    scale: f32,
    #[param(label = "Shape")]
    shape: Shape,
    #[param(label = "GPU Direct")]
    gpu_direct: bool,
    #[param(label = "Reset")]
    reset: Pulse,
}

#[derive(Debug, Default)]
struct SimpleShapesSop {
    params: SimpleShapesSopParams,
    execute_count: u32,
    offset: f64,
    vbo_tex_layers: i32,
}

impl SimpleShapesSop {
    fn cube_geometry(output: &mut SopOutput, scale: f32) {
        // front
        output.add_point((-1.0 * scale, -1.0, 1.0));
        output.add_point((3.0 * scale, -1.0, 1.0));
        output.add_point((3.0 * scale, 1.0, 1.0));
        output.add_point((1.0 * scale, 1.0, 1.0));
        // back
        output.add_point((1.0 * scale, -1.0, -1.0));
        output.add_point((3.0 * scale, -1.0, -1.0));
        output.add_point((3.0 * scale, 1.0, -1.0));
        output.add_point((1.0 * scale, 1.0, -1.0));

        let normal = [
            // front
            (-0.5, -0.5, 0.5),
            (0.5, -0.5, 0.5),
            (0.5, 0.5, 0.5),
            (-0.5, 0.5, 0.5),
            // back
            (-0.5, -0.5, -0.5),
            (0.5, -0.5, -0.5),
            (0.5, 0.5, -0.5),
            (-0.5, 0.5, -0.5),
        ];

        let color = [
            // front colors
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            // back colors
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
        ];

        let color2: [f32; 32] = [
            // front colors
            1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            // back colors
            1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
        ];

        let verticies: [u32; 36] = [
            // front
            0, 1, 2, 2, 3, 0, // right
            1, 5, 6, 6, 2, 1, // back
            7, 6, 5, 5, 4, 7, // left
            4, 0, 3, 3, 7, 4, // bottom
            4, 5, 1, 1, 0, 4, // top
            3, 2, 6, 6, 7, 3,
        ];

        let sz = 8;
        for i in 0..sz {
            output.set_normal(normal[i], i);
            output.set_color(color[i], i);
        }

        let custom_attr = CustomAttributeData::new_float("CustomColor", &color2, 4);
        output.set_custom_attribute(&custom_attr, sz);
        for i in 0..verticies.len() {
            output.add_triangle(verticies[i * 3], verticies[i * 3 + 1], verticies[i * 3 + 2]);
        }
    }

    fn line_geometry(output: &mut SopOutput) {
        // line 1 = 9 vertices
        output.add_point((-0.8, 0.0, 1.0));
        output.add_point((-0.6, 0.4, 1.0));
        output.add_point((-0.4, 0.8, 1.0));
        output.add_point((-0.2, 0.4, 1.0));
        output.add_point((0.0, 0.0, 1.0));
        output.add_point((0.2, -0.4, 1.0));
        output.add_point((0.4, -0.8, 1.0));
        output.add_point((0.6, -0.4, 1.0));
        output.add_point((0.8, 0.0, 1.0));

        // line 2 = 8 vertices
        output.add_point((-0.8, 0.2, 1.0));
        output.add_point((-0.6, 0.6, 1.0));
        output.add_point((-0.4, 1.0, 1.0));
        output.add_point((-0.2, 0.6, 1.0));
        output.add_point((0.0, 0.2, 1.0));
        output.add_point((0.2, -0.2, 1.0));
        output.add_point((0.4, -0.6, 1.0));
        output.add_point((0.6, -0.2, 1.0));

        let normal = [
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
        ];

        let color = [
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 0.0, 0.0, 1.0),
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0),
        ];

        let color2 = [
            1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
            0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
        ];

        let vertices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let line_sizes: [u32; 2] = [9, 8];
        let sz = line_sizes[0] + line_sizes[1];

        for i in 0..sz as usize {
            output.set_normal(normal[i], i);
            output.set_color(color[i], i);
        }

        let custom_attr = CustomAttributeData::new_float("CustomColor", &color2, 4);
        output.set_custom_attribute(&custom_attr, sz as usize);
        output.add_lines(&vertices, &line_sizes);
    }

    pub fn triangle_geometry(output: &mut SopOutput) {
        let vertices = [0, 1, 2];
        output.add_point((0.0, 0.0, 0.0));
        output.add_point((2.0, 0.0, 0.0));
        output.add_point((0.0, 2.0, 0.0));

        let normal = (0.0, 0.0, 1.0);

        output.set_normal(normal, 0);
        output.set_normal(normal, 1);
        output.set_normal(normal, 2);

        output.add_triangle(vertices[0], vertices[1], vertices[2]);
    }

    pub fn fill_face_vbo(
        output: &mut SopVboOutput,
        in_vert: &Position,
        in_normal: &Vec3,
        in_color: &Color,
        in_tex_coord: &TexCoord,
        idx: usize,
        vert_size: usize,
        tri_size: usize,
        num_tex_layers: usize,
        scale: f32,
    ) {
    }
}

impl OpNew for SimpleShapesSop {
    fn new(info: NodeInfo) -> Self {
        Default::default()
    }
}

impl OpInfo for SimpleShapesSop {
    const OPERATOR_TYPE: &'static str = "Simpleshapes";
    const OPERATOR_LABEL: &'static str = "Simple Shapes";
    const OPERATOR_ICON: &'static str = "SSP";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
}

impl Op for SimpleShapesSop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Sop for SimpleShapesSop {
    fn general_info(&self, input: &OperatorInputs<SopInput>) -> SopGeneralInfo {
        SopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            direct_to_gpu: self.params.gpu_direct,
        }
    }

    fn execute(&mut self, output: &mut SopOutput, inputs: &OperatorInputs<SopInput>) {
        self.execute_count += 1;

        if let Some(input) = inputs.input(0) {
            inputs.params().enable_param("Reset", false);
            inputs.params().enable_param("Shape", false);
            inputs.params().enable_param("Scale", false);

            let pts = input.point_positions();
            let mut normals = None;
            let mut colors = None;
            let mut textures = None;
            let mut num_textures = 0;

            if input.has_normals() {
                normals = Some(input.normals());
            }
            if input.has_colors() {
                colors = Some(input.colors());
            }
            if let (tex, size) = input.textures()
                && size > 0
            {
                textures = Some(tex);
                num_textures = size;
            }

            for i in 0..input.num_points() {
                output.add_point(&pts[i]);
                if let Some(normals) = normals {
                    output.set_normal(&normals[i], i);
                }
                if let Some(colors) = &colors {
                    output.set_color(&colors[i], i);
                }
                if let Some(textures) = &textures {
                    output.set_tex_coord(&textures[i], num_textures, i);
                }
            }

            for attr in input.custom_attributes() {
                output.set_custom_attribute(attr, input.num_points());
            }

            for prim in input.primitives() {
                let prim_vert = prim.point_indices();
                output.add_triangle(prim_vert[0], prim_vert[1], prim_vert[2]);
            }
        } else {
            inputs.params().enable_param("Reset", true);
            inputs.params().enable_param("Shape", true);
            inputs.params().enable_param("Scale", true);

            if let Some(chop) = self.params.chop.input() {
                let num_samples = chop.num_samples();
                let idx = 0;
                self.params.scale = chop.channel(0)[0] * self.params.scale;
            }

            match self.params.shape {
                Shape::Cube => {
                    Self::cube_geometry(output, self.params.scale);
                    output.set_bounding_box((1.0, -1.0, -1.0, 3.0, 1.0, 1.0));

                    let num_pts = output.num_points();
                    let gr_pts = (num_pts / 2) as i32;
                    let num_pr = output.num_primitives();

                    let gr1 = "pointGroup";
                    let gr2 = "primGroup";

                    output.add_group(GroupType::Point, gr1);
                    output.add_group(GroupType::Primitive, gr2);

                    for i in 0..gr_pts as usize {
                        output.add_point_to_group(i, gr1);
                    }
                    for i in 0..num_pr as usize {
                        output.add_prim_to_group(i, gr2);
                    }
                }
                Shape::Triangle => {
                    Self::triangle_geometry(output);
                }
                Shape::Line => {
                    Self::line_geometry(output);
                }
                _ => {
                    Self::cube_geometry(output, self.params.scale);
                    output.set_bounding_box((1.0, -1.0, -1.0, 3.0, 1.0, 1.0));
                }
            }
        }
    }
}

sop_plugin!(SimpleShapesSop);
