use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params, Default, Clone)]
struct CpuMemoryTopParams {
    #[param(label = "Brightness", min = 0.0, max = 1.0)]
    brightness: f64,
    #[param(label = "Speed", min = -10.0, max = 10.0, default = 1.0)]
    speed: f64,
    #[param(label = "Reset")]
    reset: Pulse,
}

/// Struct representing our SOP's state
#[derive(Params, Default)]
pub struct CpuMemoryTop {
    execute_count: u32,
    params: CpuMemoryTopParams,
}

/// Impl block providing default constructor for plugin
impl CpuMemoryTop {
    pub(crate) fn new(info: NodeInfo) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl OpInfo for CpuMemoryTop {
    const OPERATOR_LABEL: &'static str = "CPU Mem Sample";
    const OPERATOR_TYPE: &'static str = "Cpumemsample";
    const OPERATOR_ICON: &'static str = "CPM";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 0;
}

impl TopInfo for CpuMemoryTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cpu;
}

impl Op for CpuMemoryTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Top for CpuMemoryTop {
    fn execute(&mut self, _output: TopOutput, _input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;
        let mut upload = UploadInfo::default();
        upload.texture_desc.width = 256;
        upload.texture_desc.height = 256;
        upload.texture_desc.tex_dim = TexDim::E2D;
        upload.texture_desc.pixel_format = PixelFormat::RGBA32Float;
        let size = upload.texture_desc.width * upload.texture_desc.height * 4;

    }
}

top_plugin!(CpuMemoryTop);
