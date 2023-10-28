use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params, Default, Clone)]
struct CpuMemoryTopParams {
    #[param(label = "Brightness", min = 0.0, max = 1.0)]
    brightness: f64,
    #[param(label = "Speed", min = -10.0, max = 10.0, default = 1.0)]
    speed: f64,
    #[param(label = "Reset")]
    reset: bool,
}

/// Struct representing our SOP's state
#[derive(Default)]
pub struct CpuMemoryTop {
    step: f64,
    speed: f64,
    brightness: f64,
    params: CpuMemoryTopParams,
}

/// Impl block providing default constructor for plugin
impl CpuMemoryTop {
    pub(crate) fn new() -> Self {
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

impl Top for CpuMemoryTop {}

top_plugin!(CpuMemoryTop);
