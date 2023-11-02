mod shapes;

use td_rs_derive::{Param, Params};
use td_rs_sop::*;


#[derive(Param, Default)]
enum Shape {
    #[default]
    Point,
    Line,
    Square,
    Cube,
}

#[derive(Params)]
struct GeneratorSopParams {
    #[param(label = "Shape")]
    shape: Shape,
    #[param(label = "Color")]
    color: Color,
    #[param(label = "GPU Direct")]
    gpu_direct: bool,
}

struct GeneratorSop {
    params: GeneratorSopParams,
    shape_gen: shapes::ShapeGenerator,
}

impl OpNew for GeneratorSop {
    fn new(info: NodeInfo) -> Self {
        Self {
            params: GeneratorSopParams {
                shape: Shape::default(),
                color: (0,0,0,0).into(),
                gpu_direct: false,
            },
            shape_gen: shapes::ShapeGenerator {
                last_vbo_alloc_vertices: 0,
            },
        }
    }
}

impl OpInfo for GeneratorSop {
    const OPERATOR_TYPE: &'static str = "Generator";
    const OPERATOR_LABEL: &'static str = "Generator";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 0;
}

impl Op for GeneratorSop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Sop for GeneratorSop {
    fn general_info(&self, _input: &OperatorInputs<SopInput>) -> SopGeneralInfo {
        SopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            direct_to_gpu: self.params.gpu_direct,
        }
    }

    fn execute(&mut self, output: &mut SopOutput, inputs: &OperatorInputs<SopInput>) {
        match self.params.shape {
            Shape::Point => self.shape_gen.output_dot(output),
            Shape::Line => self.shape_gen.output_line(output),
            Shape::Square => self.shape_gen.output_square(output),
            Shape::Cube => self.shape_gen.output_cube(output)
        }

        for i in 0..output.num_points() {
            output.set_color(&self.params.color, i);
        }

        output.set_bounding_box((-1, -1, -1, 1, 1, 1));
    }

    fn execute_vbo(&mut self, output: &mut SopVboOutput, _inputs: &OperatorInputs<SopInput>) {
        output.enable_color();
        output.enable_color();
        output.enable_normal();
        output.enable_tex_coord(1);

        match self.params.shape {
            Shape::Point => self.shape_gen.output_dot_vbo(output),
            Shape::Line => self.shape_gen.output_line_vbo(output),
            Shape::Square => self.shape_gen.output_square_vbo(output),
            Shape::Cube => self.shape_gen.output_cube_vbo(output)
        }


        let colors = output.get_colors();
        let num_vertices = self.shape_gen.last_vbo_alloc_vertices;
        for i in 0..num_vertices {
            colors[i] = self.params.color.clone();
        }
        output.set_bounding_box((-1, -1, -1, 1, 1, 1));
        output.update_complete();
    }
}

sop_plugin!(GeneratorSop);