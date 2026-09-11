use td_rs_derive::Params;
use td_rs_pop::*;

#[derive(Params, Default, Clone, Debug)]
struct SimpleShapesPopParams {
    #[param(label = "Scale", default = 1.0, min = 0.0, max = 10.0)]
    scale: f64,
}

struct SimpleShapesPop {
    params: SimpleShapesPopParams,
    context: PopContext,
}

impl PopNew for SimpleShapesPop {
    fn new(_info: NodeInfo, context: PopContext) -> Self {
        Self {
            params: SimpleShapesPopParams::default(),
            context,
        }
    }
}

impl OpInfo for SimpleShapesPop {
    const OPERATOR_TYPE: &'static str = "Simpleshapesrs";
    const OPERATOR_LABEL: &'static str = "Simple Shapes RS";
    const OPERATOR_ICON: &'static str = "SSR";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 0;
}

impl Op for SimpleShapesPop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Pop for SimpleShapesPop {
    fn execute(&mut self, output: &mut PopOutput, _inputs: &OperatorInputs<PopInput>) {
        let scale = self.params.scale as f32;
        if !self.cube(output, scale) {
            self.set_error("Could not allocate POP buffers");
        }
    }
}

impl SimpleShapesPop {
    fn cube(&mut self, output: &mut PopOutput, scale: f32) -> bool {
        let s = scale;
        #[rustfmt::skip]
        let positions: [f32; 24] = [
            // front
            -s, -s,  s,
             s, -s,  s,
             s,  s,  s,
            -s,  s,  s,
            // back
            -s, -s, -s,
             s, -s, -s,
             s,  s, -s,
            -s,  s, -s,
        ];

        let Some(mut pos_buf) = self.context.create_buffer(
            std::mem::size_of_val(&positions),
            BufferMode::SequentialWrite,
            BufferUsage::Attribute,
        ) else {
            return false;
        };
        pos_buf.data_mut::<f32>().copy_from_slice(&positions);
        output.set_attribute(
            pos_buf,
            &AttributeInfo {
                name: "P".to_string(),
                num_components: 3,
                class: AttributeClass::Point,
                ..Default::default()
            },
        );

        #[rustfmt::skip]
        let face_normals: [[f32; 3]; 6] = [
            [0.0, 0.0, 1.0],   // front
            [1.0, 0.0, 0.0],   // right
            [0.0, 0.0, -1.0],  // back
            [-1.0, 0.0, 0.0],  // left
            [0.0, -1.0, 0.0],  // bottom
            [0.0, 1.0, 0.0],   // top
        ];
        let mut normals = [0.0f32; 6 * 6 * 3];
        for (face, normal) in face_normals.iter().enumerate() {
            for vertex in 0..6 {
                let base = (face * 6 + vertex) * 3;
                normals[base..base + 3].copy_from_slice(normal);
            }
        }

        let Some(mut normal_buf) = self.context.create_buffer(
            std::mem::size_of_val(&normals),
            BufferMode::SequentialWrite,
            BufferUsage::Attribute,
        ) else {
            return false;
        };
        normal_buf.data_mut::<f32>().copy_from_slice(&normals);
        output.set_attribute(
            normal_buf,
            &AttributeInfo {
                name: "N".to_string(),
                num_components: 3,
                qualifier: AttributeQualifier::Direction,
                class: AttributeClass::Vertex,
                ..Default::default()
            },
        );

        #[rustfmt::skip]
        let face_colors: [[f32; 4]; 6] = [
            [1.0, 0.0, 0.0, 1.0], // front
            [0.0, 1.0, 0.0, 1.0], // right
            [0.0, 0.0, 1.0, 1.0], // back
            [1.0, 1.0, 1.0, 1.0], // left
            [1.0, 1.0, 0.0, 1.0], // bottom
            [1.0, 0.0, 1.0, 1.0], // top
        ];
        let mut colors = [0.0f32; 6 * 2 * 4];
        for (face, color) in face_colors.iter().enumerate() {
            for triangle in 0..2 {
                let base = (face * 2 + triangle) * 4;
                colors[base..base + 4].copy_from_slice(color);
            }
        }

        let Some(mut color_buf) = self.context.create_buffer(
            std::mem::size_of_val(&colors),
            BufferMode::SequentialWrite,
            BufferUsage::Attribute,
        ) else {
            return false;
        };
        color_buf.data_mut::<f32>().copy_from_slice(&colors);
        output.set_attribute(
            color_buf,
            &AttributeInfo {
                name: "Color".to_string(),
                num_components: 4,
                qualifier: AttributeQualifier::Color,
                class: AttributeClass::Primitive,
                ..Default::default()
            },
        );

        #[rustfmt::skip]
        let indices: [u32; 36] = [
            // front
            0, 1, 2, 2, 3, 0,
            // right
            1, 5, 6, 6, 2, 1,
            // back
            7, 6, 5, 5, 4, 7,
            // left
            4, 0, 3, 3, 7, 4,
            // bottom
            4, 5, 1, 1, 0, 4,
            // top
            3, 2, 6, 6, 7, 3,
        ];

        let Some(mut index_buf) = self.context.create_buffer(
            std::mem::size_of_val(&indices),
            BufferMode::SequentialWrite,
            BufferUsage::IndexBuffer,
        ) else {
            return false;
        };
        index_buf.data_mut::<u32>().copy_from_slice(&indices);
        output.set_index_buffer(index_buf);

        let point_info = self.context.create_point_info_buffer(8);
        let topology_info = self.context.create_topology_info_buffer(&TopologyInfo {
            triangles_count: indices.len() as u32 / 3,
            ..Default::default()
        });
        if point_info.is_none() || topology_info.is_none() {
            return false;
        }
        output.set_info_buffers(InfoBuffers {
            point_info,
            topology_info,
            ..Default::default()
        });

        true
    }
}

pop_plugin!(SimpleShapesPop);
