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
    fn new(_info: NodeInfo) -> Self {
        Self {
            params: GeneratorSopParams {
                shape: Shape::default(),
                color: (0, 0, 0, 0).into(),
                gpu_direct: false,
            },
            shape_gen: shapes::ShapeGenerator {},
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

    fn execute(&mut self, output: &mut SopOutput, _inputs: &OperatorInputs<SopInput>) {
        match self.params.shape {
            Shape::Point => self.shape_gen.output_dot(output),
            Shape::Line => self.shape_gen.output_line(output),
            Shape::Square => self.shape_gen.output_square(output),
            Shape::Cube => self.shape_gen.output_cube(output),
        }

        for i in 0..output.num_points() {
            output.set_color(&self.params.color, i);
        }

        output.set_bounding_box((-1, -1, -1, 1, 1, 1));
    }

    fn execute_vbo(&mut self, output: SopVboOutput<Unalloc>, _inputs: &OperatorInputs<SopInput>) {
        let mut output = match self.params.shape {
            Shape::Point => {
                let mut output = output.alloc_all(1, 1, 1, BufferMode::Static);
                self.shape_gen.output_dot_vbo(&mut output);
                output
            }
            Shape::Line => {
                let mut output = output.alloc_all(
                    shapes::THE_LINE_NUM_PTS,
                    shapes::THE_LINE_NUM_PTS,
                    1,
                    BufferMode::Static,
                );
                self.shape_gen.output_line_vbo(&mut output);
                output
            }
            Shape::Square => {
                let mut output = output.alloc_all(
                    shapes::THE_SQUARE_NUM_PTS,
                    shapes::THE_SQUARE_NUM_PRIM * 3,
                    1,
                    BufferMode::Static,
                );
                self.shape_gen.output_square_vbo(&mut output);
                output
            }
            Shape::Cube => {
                let mut output = output.alloc_all(
                    shapes::THE_CUBE_NUM_PTS,
                    shapes::THE_CUBE_NUM_PRIM * 3,
                    1,
                    BufferMode::Static,
                );
                self.shape_gen.output_cube_vbo(&mut output);
                output
            }
        };

        let colors = output.colors();
        let num_vertices = output.state.vertices;
        for i in 0..num_vertices {
            colors[i] = self.params.color.clone();
        }
        output.set_bounding_box((-1, -1, -1, 1, 1, 1));
        let _output = output.update_complete();
    }
}

sop_plugin!(GeneratorSop);
