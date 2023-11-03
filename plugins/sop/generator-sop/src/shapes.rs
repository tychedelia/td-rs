use td_rs_sop::*;

pub(crate) const THE_CUBE_NUM_PTS: usize = 24;
pub(crate) const THE_CUBE_NUM_PRIM: usize = 12;
pub(crate) const THE_SQUARE_NUM_PTS: usize = 4;
pub(crate) const THE_SQUARE_NUM_PRIM: usize = 2;
pub(crate) const THE_LINE_NUM_PTS: usize = 2;

pub const THE_CUBE_POS: [Position; THE_CUBE_NUM_PTS] = [
    // front
    Position::new(-1.0, -1.0, 1.0),
    Position::new(-1.0, 1.0, 1.0),
    Position::new(1.0, -1.0, 1.0),
    Position::new(1.0, 1.0, 1.0),
    // back
    Position::new(-1.0, -1.0, -1.0),
    Position::new(-1.0, 1.0, -1.0),
    Position::new(1.0, -1.0, -1.0),
    Position::new(1.0, 1.0, -1.0),
    // top
    Position::new(-1.0, 1.0, -1.0),
    Position::new(1.0, 1.0, -1.0),
    Position::new(-1.0, 1.0, 1.0),
    Position::new(1.0, 1.0, 1.0),
    // bottom
    Position::new(-1.0, -1.0, -1.0),
    Position::new(1.0, -1.0, -1.0),
    Position::new(-1.0, -1.0, 1.0),
    Position::new(1.0, -1.0, 1.0),
    // right
    Position::new(1.0, -1.0, -1.0),
    Position::new(1.0, -1.0, 1.0),
    Position::new(1.0, 1.0, -1.0),
    Position::new(1.0, 1.0, 1.0),
    // left
    Position::new(-1.0, -1.0, -1.0),
    Position::new(-1.0, -1.0, 1.0),
    Position::new(-1.0, 1.0, -1.0),
    Position::new(-1.0, 1.0, 1.0),
];

pub const THE_CUBE_NORMALS: [Vec3; THE_CUBE_NUM_PTS] = [
    // front
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    // back
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
    // top
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    // bottom
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    // right
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    // left
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
];

pub const THE_CUBE_VERTICES: [u32; THE_CUBE_NUM_PRIM * 3] = [
    // front
    0, 1, 2, 3, 2, 1, // back
    6, 5, 4, 5, 6, 7, // top
    8, 9, 10, 11, 10, 9, // bottom
    14, 13, 12, 13, 14, 15, // right
    16, 17, 18, 19, 18, 17, // left
    22, 21, 20, 21, 22, 23,
];

pub const THE_CUBE_TEXTURE: [TexCoord; THE_CUBE_NUM_PTS] = [
    // front
    TexCoord::new(2.0 / 3.0, 0.0, 0.0),
    TexCoord::new(2.0 / 3.0, 0.5, 0.0),
    TexCoord::new(3.0 / 3.0, 0.0, 0.0),
    TexCoord::new(3.0 / 3.0, 0.5, 0.0),
    // back
    TexCoord::new(0.0 / 3.0, 0.5, 0.0),
    TexCoord::new(0.0 / 3.0, 0.0, 0.0),
    TexCoord::new(1.0 / 3.0, 0.5, 0.0),
    TexCoord::new(1.0 / 3.0, 0.0, 0.0),
    // top
    TexCoord::new(2.0 / 3.0, 1.0, 0.0),
    TexCoord::new(3.0 / 3.0, 1.0, 0.0),
    TexCoord::new(2.0 / 3.0, 0.5, 0.0),
    TexCoord::new(3.0 / 3.0, 0.5, 0.0),
    // bottom
    TexCoord::new(1.0 / 3.0, 0.5, 0.0),
    TexCoord::new(2.0 / 3.0, 0.5, 0.0),
    TexCoord::new(1.0 / 3.0, 1.0, 0.0),
    TexCoord::new(2.0 / 3.0, 1.0, 0.0),
    // right
    TexCoord::new(2.0 / 3.0, 0.0, 0.0),
    TexCoord::new(1.0 / 3.0, 0.0, 0.0),
    TexCoord::new(2.0 / 3.0, 0.5, 0.0),
    TexCoord::new(1.0 / 3.0, 0.5, 0.0),
    // left
    TexCoord::new(1.0 / 3.0, 1.0, 0.0),
    TexCoord::new(0.0 / 3.0, 1.0, 0.0),
    TexCoord::new(1.0 / 3.0, 0.5, 0.0),
    TexCoord::new(0.0 / 3.0, 0.5, 0.0),
];

// Square descriptors
pub const THE_SQUARE_POS: [Position; THE_SQUARE_NUM_PTS] = [
    Position::new(-1.0, -1.0, 0.0),
    Position::new(-1.0, 1.0, 0.0),
    Position::new(1.0, -1.0, 0.0),
    Position::new(1.0, 1.0, 0.0),
];

pub const THE_SQUARE_NORMALS: [Vec3; THE_SQUARE_NUM_PTS] = [
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
];

pub const THE_SQUARE_VERTICES: [u32; THE_SQUARE_NUM_PRIM * 3] = [0, 1, 2, 3, 2, 1];

pub const THE_SQUARE_TEXTURE: [TexCoord; THE_SQUARE_NUM_PTS] = [
    TexCoord::new(0.0, 0.0, 0.0),
    TexCoord::new(0.0, 1.0, 0.0),
    TexCoord::new(1.0, 0.0, 0.0),
    TexCoord::new(1.0, 1.0, 0.0),
];

// Line descriptors
pub const THE_LINE_POS: [Position; THE_LINE_NUM_PTS] = [
    Position::new(-1.0, -1.0, -1.0),
    Position::new(1.0, 1.0, 1.0),
];

pub const THE_LINE_NORMALS: [Vec3; THE_LINE_NUM_PTS] =
    [Vec3::new(-1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, 1.0)];

pub const THE_LINE_VERTICES: [u32; THE_LINE_NUM_PTS] = [0, 1];

pub const THE_LINE_TEXTURE: [TexCoord; THE_LINE_NUM_PTS] =
    [TexCoord::new(0.0, 0.0, 0.0), TexCoord::new(1.0, 1.0, 0.0)];

// Point descriptors
pub const THE_POINT_POS: Position = Position::new(0.0, 0.0, 0.0);
pub const THE_POINT_NORMAL: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const THE_POINT_TEXTURE: TexCoord = TexCoord::new(0.0, 0.0, 0.0);

pub(crate) struct ShapeGenerator {}

impl ShapeGenerator {
    pub fn output_dot(&self, output: &mut SopOutput) {
        output.add_point(THE_POINT_POS);
        output.set_normal(THE_POINT_NORMAL, 0);
        output.add_particle_system(1, 0);
        // output.set_tex_coord(&THE_POINT_TEXTURE, 1, 0);
    }

    pub fn output_line(&self, output: &mut SopOutput) {
        output.add_points(&THE_LINE_POS);
        output.set_normals(&THE_LINE_NORMALS, 0);
        output.add_line(&THE_LINE_VERTICES);
        // output.set_tex_coords(&THE_LINE_TEXTURE, 1, 0);
    }

    pub fn output_square(&self, output: &mut SopOutput) {
        output.add_points(&THE_SQUARE_POS);
        output.set_normals(&THE_SQUARE_NORMALS, 0);
        output.add_triangles(&THE_SQUARE_VERTICES);
        output.set_tex_coords(&THE_SQUARE_TEXTURE, 1, 0);
    }

    pub fn output_cube(&self, output: &mut SopOutput) {
        output.add_points(&THE_CUBE_POS);
        output.set_normals(&THE_CUBE_NORMALS, 0);
        output.add_triangles(&THE_CUBE_VERTICES);
        output.set_tex_coords(&THE_CUBE_TEXTURE, 1, 0);
    }

    pub fn output_dot_vbo(&mut self, output: &mut SopVboOutput<AllocAll>) {
        output.positions()[0] = THE_POINT_POS;
        output.normals()[0] = THE_POINT_NORMAL;
        output.tex_coords()[0] = THE_POINT_TEXTURE;
    }

    pub fn output_line_vbo(&mut self, output: &mut SopVboOutput<AllocAll>) {
        output.positions().clone_from_slice(&THE_LINE_POS);
        output.normals().clone_from_slice(&THE_LINE_NORMALS);
        output.tex_coords().clone_from_slice(&THE_LINE_TEXTURE);
        output
            .add_lines(THE_LINE_NUM_PTS)
            .clone_from_slice(&THE_LINE_VERTICES);
    }

    pub fn output_square_vbo(&mut self, output: &mut SopVboOutput<AllocAll>) {
        output.positions().clone_from_slice(&THE_SQUARE_POS);
        // Here we manually invert the normals as per original C++ logic
        for (i, normal) in output.normals().iter_mut().enumerate() {
            *normal = &THE_SQUARE_NORMALS[i] * -1.0;
        }
        output.tex_coords().clone_from_slice(&THE_SQUARE_TEXTURE);
        output
            .add_triangles(THE_SQUARE_NUM_PRIM)
            .clone_from_slice(&THE_SQUARE_VERTICES);
    }

    pub fn output_cube_vbo(&mut self, output: &mut SopVboOutput<AllocAll>) {
        output.positions().clone_from_slice(&THE_CUBE_POS);
        for (i, normal) in output.normals().iter_mut().enumerate() {
            *normal = &THE_CUBE_NORMALS[i] * -1.0;
        }
        output.tex_coords().clone_from_slice(&THE_CUBE_TEXTURE);
        output
            .add_triangles(THE_CUBE_NUM_PRIM)
            .clone_from_slice(&THE_CUBE_VERTICES);
    }
}
