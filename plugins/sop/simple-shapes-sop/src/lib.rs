#![feature(let_chains)]
#![feature(iter_array_chunks)]

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
    vbo_tex_layers: usize,
}

impl SimpleShapesSop {
    fn cube_geometry(output: &mut SopOutput, scale: f32) {
        // front
        output.add_point((1.0 * scale, -1.0, 1.0));
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
        for [x, y, z] in verticies.into_iter().array_chunks() {
            output.add_triangle(x, y, z);
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

        normal.iter().enumerate().for_each(|(i, n)| output.set_normal(*n, i));
        color.iter().enumerate().for_each(|(i, c)| output.set_color(*c, i));

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

    fn fill_face_vbo(
        output: &mut SopVboOutput,
        in_vert: Option<&[Position]>,
        in_normal: Option<&[Vec3]>,
        in_color: Option<&[Color]>,
        in_tex_coord: Option<&[TexCoord]>,
        in_idx: &[u32],
        vert_sz: usize,
        tri_size: usize,
        num_tex_layers: usize,
        scale: f32,
    ) {
        if let Some(vert_out) = in_vert.map(|_| output.get_pos()) {
            // Correctly handle vert_sz as the count of components.
            for (k, v) in in_vert.unwrap().iter().enumerate().take(vert_sz) {
                vert_out[k] = v * scale;
            }
        }

        if let Some(normal_out) = in_normal.filter(|_| output.has_normal()) {
            let normals = output.get_normals();
            // Assume vert_sz is the total number of components (e.g., vert_sz = num_vertices * 3 for Vec3 normals)
            for (k, normal) in in_normal.unwrap().iter().enumerate().take(vert_sz) {
                normals[k] = normal.clone();
            }
        }

        if let Some(color_out) = in_color.filter(|_| output.has_color()) {
            let colors = output.get_colors();
            // Again, use vert_sz to copy the correct number of color components.
            for (k, color) in in_color.unwrap().iter().enumerate().take(vert_sz) {
                colors[k] = color.clone();
            }
        }

        if let Some(tex_coords) = in_tex_coord.filter(|_| output.has_tex_coord()) {
            let tex_coord_out = output.get_tex_coords();
            for k in 0..vert_sz {
                for t in 0..num_tex_layers + 1 {
                    tex_coord_out[k * (num_tex_layers + 1) + t] =
                        tex_coords[k * (num_tex_layers + 1) + t].clone();
                }
            }
        }

        let index_buffer = output.add_triangles(tri_size);
        for (i, idx_chunk) in in_idx.chunks(3).enumerate().take(tri_size) {
            let base_index = i * 3;
            index_buffer[base_index..base_index + 3].copy_from_slice(idx_chunk);
        }
    }

    fn fill_line_vbo(
        output: &mut SopVboOutput,
        in_vert: Option<&[Position]>,
        in_normal: Option<&[Vec3]>,
        in_color: Option<&[Color]>,
        in_tex_coord: Option<&[TexCoord]>,
        in_idx: &[u32],
        vert_sz: usize,
        line_size: usize,
        num_tex_layers: usize,
    ) {
        if let Some(vert_out) = in_vert.map(|_| output.get_pos()) {
            vert_out[..vert_sz].clone_from_slice(in_vert.unwrap());
        }

        if let Some(normal_out) = in_normal.filter(|_| output.has_normal()) {
            let normals = output.get_normals();
            normals[..vert_sz].clone_from_slice(normal_out);
        }

        if let Some(color_out) = in_color.filter(|_| output.has_color()) {
            let colors = output.get_colors();
            colors[..vert_sz].clone_from_slice(color_out);
        }

        if let Some(tex_coords) = in_tex_coord.filter(|_| output.has_tex_coord()) {
            let tex_coord_out = output.get_tex_coords();
            for k in 0..vert_sz {
                for t in 0..num_tex_layers + 1 {
                    tex_coord_out[k * (num_tex_layers + 1) + t] =
                        tex_coords[k * (num_tex_layers + 1) + t].clone();
                }
            }
        }

        let index_buffer = output.add_lines(line_size);
        index_buffer[..line_size].clone_from_slice(&in_idx[..line_size]);
    }

    fn fill_particle_vbo(
        output: &mut SopVboOutput,
        in_vert: Option<&[Position]>,
        in_normal: Option<&[Vec3]>,
        in_color: Option<&[Color]>,
        in_tex_coord: Option<&[TexCoord]>,
        in_idx: &[u32],
        vert_sz: usize,
        size: usize,
        num_tex_layers: usize,
    ) {
        if let Some(vert_out) = in_vert.map(|_| output.get_pos()) {
            vert_out[..vert_sz].clone_from_slice(in_vert.unwrap());
        }

        if let Some(normal_out) = in_normal.filter(|_| output.has_normal()) {
            let normals = output.get_normals();
            normals[..vert_sz].clone_from_slice(normal_out);
        }

        if let Some(color_out) = in_color.filter(|_| output.has_color()) {
            let colors = output.get_colors();
            colors[..vert_sz].clone_from_slice(color_out);
        }

        if let Some(tex_coords) = in_tex_coord.filter(|_| output.has_tex_coord()) {
            let tex_coord_out = output.get_tex_coords();
            for k in 0..vert_sz {
                for t in 0..num_tex_layers + 1 {
                    tex_coord_out[k * (num_tex_layers + 1) + t] =
                        tex_coords[k * (num_tex_layers + 1) + t].clone();
                }
            }
        }

        let index_buffer = output.add_particle_system(size);
        index_buffer[..size].clone_from_slice(&in_idx[..size]);
    }

    fn cube_geometry_vbo(&self, output: &mut SopVboOutput, scale: f32) {
        let point_arr = [
            //front
            (1.0, -1.0, 1.0).into(), //v0
            (3.0, -1.0, 1.0).into(), //v1
            (3.0, 1.0, 1.0).into(),  //v2
            (1.0, 1.0, 1.0).into(),  //v3
            //right
            (3.0, 1.0, 1.0).into(),   //v2
            (3.0, 1.0, -1.0).into(),  //v6
            (3.0, -1.0, -1.0).into(), //v5
            (3.0, -1.0, 1.0).into(),  //v1
            //back
            (1.0, -1.0, -1.0).into(), //v4
            (3.0, -1.0, -1.0).into(), //v5
            (3.0, 1.0, -1.0).into(),  //v6
            (1.0, 1.0, -1.0).into(),  //v7
            //left
            (1.0, -1.0, -1.0).into(), //v4
            (1.0, -1.0, 1.0).into(),  // v0
            (1.0, 1.0, 1.0).into(),   //v3
            (1.0, 1.0, -1.0).into(),  //v7
            //upper
            (3.0, 1.0, 1.0).into(),  //v1
            (1.0, 1.0, 1.0).into(),  //v3
            (1.0, 1.0, -1.0).into(), //v7
            (3.0, 1.0, -1.0).into(), //v6
            //bottom
            (1.0, -1.0, -1.0).into(), //v4
            (3.0, -1.0, -1.0).into(), //v5
            (3.0, -1.0, 1.0).into(),  //v1
            (1.0, -1.0, 1.0).into(),  //v0
        ];

        let normals = [
            //front
            (1.0, 0.0, 0.0).into(), //v0
            (0.0, 1.0, 0.0).into(), //v1
            (0.0, 0.0, 1.0).into(), //v2
            (1.0, 1.0, 1.0).into(), //v3
            //right
            (0.0, 0.0, 1.0).into(), //v2
            (0.0, 0.0, 1.0).into(), //v6
            (0.0, 1.0, 0.0).into(), //v5
            (0.0, 1.0, 0.0).into(), //v1
            //back
            (1.0, 0.0, 0.0).into(), //v4
            (0.0, 1.0, 0.0).into(), //v5
            (0.0, 0.0, 1.0).into(), //v6
            (1.0, 1.0, 1.0).into(), //v7
            //left
            (1.0, 0.0, 0.0).into(), //v4
            (1.0, 0.0, 0.0).into(), // v0
            (1.0, 1.0, 1.0).into(), //v3
            (1.0, 1.0, 1.0).into(), //v7
            //upper
            (0.0, 1.0, 0.0).into(), //v1
            (1.0, 1.0, 1.0).into(), //v3
            (1.0, 1.0, 1.0).into(), //v7
            (0.0, 0.0, 1.0).into(), //v6
            //bottom
            (1.0, 0.0, 0.0).into(), //v4
            (0.0, 1.0, 0.0).into(), //v5
            (0.0, 1.0, 0.0).into(), //v1
            (1.0, 0.0, 0.0).into(), //v0
        ];

        let colors = [
            //front
            (0, 0, 1, 1).into(),
            (1, 0, 1, 1).into(),
            (1, 1, 1, 1).into(),
            (0, 1, 1, 1).into(),
            //right
            (1, 1, 1, 1).into(),
            (1, 1, 0, 1).into(),
            (1, 0, 0, 1).into(),
            (1, 0, 1, 1).into(),
            //back
            (0, 0, 0, 1).into(),
            (1, 0, 0, 1).into(),
            (1, 1, 0, 1).into(),
            (0, 1, 0, 1).into(),
            //left
            (0, 0, 0, 1).into(),
            (0, 0, 1, 1).into(),
            (0, 1, 1, 1).into(),
            (0, 1, 0, 1).into(),
            //up
            (1, 1, 1, 1).into(),
            (0, 1, 1, 1).into(),
            (0, 1, 0, 1).into(),
            (1, 1, 0, 1).into(),
            //bottom
            (0, 0, 0, 1).into(),
            (1, 0, 0, 1).into(),
            (1, 0, 1, 1).into(),
            (0, 0, 1, 1).into(),
        ];

        let tex_coords = [
            //front
            (0.0, 0.0, 0.0).into(), //v0
            (0.0, 1.0, 0.0).into(), //v1
            (1.0, 1.0, 0.0).into(), //v2
            (1.0, 0.0, 0.0).into(), //v3
            //right
            (1.0, 0.0, 0.0).into(), //v2
            (1.0, 1.0, 0.0).into(), //v6
            (1.0, 1.0, 0.0).into(), //v5
            (1.0, 0.0, 0.0).into(), //v1
            //back
            (1.0, 0.0, 0.0).into(), //v4
            (1.0, 1.0, 0.0).into(), //v5
            (0.0, 1.0, 0.0).into(), //v6
            (0.0, 0.0, 0.0).into(), //v7
            //left
            (0.0, 0.0, 0.0).into(), //v4
            (0.0, 1.0, 0.0).into(), //v0
            (0.0, 1.0, 0.0).into(), //v3
            (0.0, 0.0, 0.0).into(), //v7
            //upper
            (0.0, 0.0, 0.0).into(), //v1
            (0.0, 0.0, 0.0).into(), //v3
            (1.0, 0.0, 0.0).into(), //v7
            (1.0, 0.0, 0.0).into(), //v6
            //bottom
            (0.0, 0.0, 0.0).into(), //v4
            (0.0, 1.0, 0.0).into(), //v5
            (1.0, 1.0, 0.0).into(), //v1
            (1.0, 1.0, 0.0).into(), //v0
        ];

        let vertices = [
            0, 1, 2, 0, 2, 3, //front
            4, 5, 6, 4, 6, 7, //right
            8, 9, 10, 8, 10, 11, //back
            12, 13, 14, 12, 14, 15, //left
            16, 17, 18, 16, 18, 19, //upper
            20, 21, 22, 20, 22, 23,
        ];

        println!("Fill face vbo");
        Self::fill_face_vbo(
            output,
            Some(&point_arr),
            Some(&normals),
            Some(&colors),
            Some(&tex_coords),
            &vertices,
            8,
            12,
            self.vbo_tex_layers,
            scale,
        );
    }

    fn line_geometry_vbo(&self, output: &mut SopVboOutput) {
        let point_arr = [
            (-0.8, 0.0, 1.0).into(),
            (-0.6, 0.4, 1.0).into(),
            (-0.4, 0.8, 1.0).into(),
            (-0.2, 0.4, 1.0).into(),
            (0.0, 0.0, 1.0).into(),
            (0.2, -0.4, 1.0).into(),
            (0.4, -0.8, 1.0).into(),
            (0.6, -0.4, 1.0).into(),
            (0.8, 0.0, 1.0).into(),
        ];

        let normals = [
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
        ];

        let colors = [
            (0, 0, 1, 1).into(),
            (1, 0, 1, 1).into(),
            (1, 1, 1, 1).into(),
            (0, 1, 1, 1).into(),
            (1, 1, 1, 1).into(),
            (1, 1, 0, 1).into(),
            (1, 0, 0, 1).into(),
            (1, 0, 1, 1).into(),
            (0, 0, 0, 1).into(),
        ];

        let tex_coords = [
            (0.0, 0.0, 0.0).into(),
            (0.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
        ];

        let vertices = [0, 1, 2, 3, 4, 5, 6, 7, 8];

        Self::fill_line_vbo(
            output,
            Some(&point_arr),
            Some(&normals),
            Some(&colors),
            Some(&tex_coords),
            &vertices,
            9,
            9,
            self.vbo_tex_layers,
        );
    }

    fn triangle_geometry_vbo(&self, output: &mut SopVboOutput) {
        let normals = [
            (1.0, 0.0, 0.0).into(), //v0
            (0.0, 1.0, 0.0).into(), //v1
            (0.0, 0.0, 1.0).into(), //v2
        ];

        let color = [
            (0.0, 0.0, 1.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
        ];

        let point_arr = [
            (0.0, 0.0, 0.0).into(),
            (0.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
        ];

        let vertices = [0, 1, 2];

        Self::fill_face_vbo(
            output,
            Some(&point_arr),
            Some(&normals),
            Some(&color),
            None,
            &vertices,
            1,
            self.vbo_tex_layers,
            1,
            1.0,
        );
    }

    fn particle_geometry_vbo(&self, output: &mut SopVboOutput) {
        let point_arr = [
            (-0.8, 0.0, 1.0).into(),
            (-0.6, 0.4, 1.0).into(),
            (-0.4, 0.8, 1.0).into(),
            (-0.2, 0.4, 1.0).into(),
            (0.0, 0.0, 1.0).into(),
            (0.2, -0.4, 1.0).into(),
            (0.4, -0.8, 1.0).into(),
            (0.6, -0.4, 1.0).into(),
            (0.8, -0.2, 1.0).into(),
            (-0.8, 0.2, 1.0).into(),
            (-0.6, 0.6, 1.0).into(),
            (-0.4, 1.0, 1.0).into(),
            (-0.2, 0.6, 1.0).into(),
            (0.0, 0.2, 1.0).into(),
            (0.2, -0.2, 1.0).into(),
            (0.4, -0.6, 1.0).into(),
            (0.6, -0.2, 1.0).into(),
            (0.8, 0.0, 1.0).into(),
            (-0.8, -0.2, 1.0).into(),
            (-0.6, 0.2, 1.0).into(),
            (-0.4, 0.6, 1.0).into(),
            (-0.2, 0.2, 1.0).into(),
            (0.0, -0.2, 1.0).into(),
            (0.2, -0.6, 1.0).into(),
            (0.4, -1.0, 1.0).into(),
            (0.6, -0.6, 1.0).into(),
            (0.8, -0.4, 1.0).into(),
        ];

        let normals = [
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0).into(),
        ];

        let colors = [
            (0.0, 0.0, 1.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (0.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 0.0, 1.0).into(),
            (1.0, 0.0, 0.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (0.0, 0.0, 0.0, 1.0).into(),
            (0.0, 0.0, 1.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (0.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 0.0, 1.0).into(),
            (1.0, 0.0, 0.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (0.0, 0.0, 0.0, 1.0).into(),
            (0.0, 0.0, 1.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (0.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 1.0, 1.0).into(),
            (1.0, 1.0, 0.0, 1.0).into(),
            (1.0, 0.0, 0.0, 1.0).into(),
            (1.0, 0.0, 1.0, 1.0).into(),
            (0.0, 0.0, 0.0, 1.0).into(),
        ];

        let tex_coords = [
            (0.0, 0.0, 0.0).into(),
            (0.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (0.0, 0.0, 0.0).into(),
            (0.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (0.0, 0.0, 0.0).into(),
            (0.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 1.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
            (1.0, 0.0, 0.0).into(),
        ];

        let vertices = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ];

        Self::fill_particle_vbo(
            output,
            Some(&point_arr),
            Some(&normals),
            Some(&colors),
            Some(&tex_coords),
            &vertices,
            27,
            27,
            self.vbo_tex_layers,
        );
    }
}

impl OpNew for SimpleShapesSop {
    fn new(_info: NodeInfo) -> Self {
        Self {
            params: SimpleShapesSopParams {
                chop: Default::default(),
                scale: 0.0,
                shape: Default::default(),
                gpu_direct: false,
                reset: Default::default(),
            },
            execute_count: 0,
            offset: 0.0,
            vbo_tex_layers: 1,
        }
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
    fn general_info(&self, _input: &OperatorInputs<SopInput>) -> SopGeneralInfo {
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

    fn execute_vbo(&mut self, output: &mut SopVboOutput, inputs: &OperatorInputs<SopInput>) {
        self.execute_count += 1;

        if inputs.num_inputs() > 0 {
            inputs.params().enable_param("Reset", false);
            inputs.params().enable_param("Shape", false);
            inputs.params().enable_param("Scale", false);
        } else {
            inputs.params().enable_param("Shape", false);
            inputs.params().enable_param("Scale", true);

            if let Some(chop) = self.params.chop.input() {
                self.params.scale = chop.channel(0)[0] * self.params.scale;
            }

            output.enable_normal();
            output.enable_color();

            self.vbo_tex_layers = 1;
            output.enable_tex_coord(self.vbo_tex_layers);

            let cu1 = CustomAttributeInfo {
                name: "customColor".to_string(),
                num_components: 4,
                attr_type: AttributeType::Float,
            };
            output.add_custom_attribute(cu1);
            let cu2 = CustomAttributeInfo {
                name: "customVert".to_string(),
                num_components: 1,
                attr_type: AttributeType::Float,
            };
            output.add_custom_attribute(cu2);

            #[cfg(windows)]
            {
                let num_vertices = 36;
                let num_indices = 36;

                output.alloc_vbo(num_vertices, num_indices, BufferMode::Static);

                self.cube_geometry_vbo(output, self.params.scale);
                output.set_bounding_box((1.0, -1.0, -1.0, 3.0, 1.0, 1.0));
            }
            #[cfg(windows)]
            {
                let num_vertices = 10;
                let num_indices = 10;

                output.alloc_vbo(num_vertices, num_indices, BufferMode::Static);

                self.line_geometry_vbo(output);
            }
            {
                // draw Particle System:
                let num_vertices = 27;
                let num_indices = 27;

                output.alloc_vbo(num_vertices, num_indices, BufferMode::Static);

                self.particle_geometry_vbo(output);
            }
        }
    }
}

sop_plugin!(SimpleShapesSop);
