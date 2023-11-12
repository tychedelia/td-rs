use raw_sync::locks::*;
use shared_memory::*;
use std::sync::atomic::{AtomicU8, Ordering};
use td_rs_chop::param::MenuParam;
use td_rs_chop::*;
use td_rs_derive::{Param, Params};

#[derive(Param, Default, Clone, Debug)]
enum Operation {
    #[default]
    Add,
    Multiply,
    Power,
}

#[derive(Params, Default, Clone, Debug)]
struct SharedMemoryChopParams {
    #[param(label = "Length", page = "Generator")]
    length: u32,
    #[param(label = "Number of Channels", page = "Generator", min = -10.0, max = 10.0)]
    num_channels: u32,
    #[param(label = "Apply Scale", page = "Generator")]
    apply_scale: bool,
    #[param(label = "Scale", page = "Generator")]
    scale: f32,
    #[param(label = "Operation", page = "Generator")]
    operation: Operation,
}

/// Struct representing our CHOP's state
#[derive(Debug)]
pub struct SharedMemoryChop {
    params: SharedMemoryChopParams,
}

/// Impl block providing default constructor for plugin
impl OpNew for SharedMemoryChop {
    #[tracing::instrument]
    fn new(_info: NodeInfo) -> Self {
        let instance = increment_value("shared_memory");

        tracing::info!("Instance: {}", instance);

        Self {
            params: SharedMemoryChopParams {
                length: 0,
                num_channels: 0,
                apply_scale: false,
                scale: 1.0,
                operation: Operation::Add,
            },
        }
    }
}

impl OpInfo for SharedMemoryChop {
    const OPERATOR_LABEL: &'static str = "Basic Generator";
    const OPERATOR_TYPE: &'static str = "Basicgenerator";
}

impl Op for SharedMemoryChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Chop for SharedMemoryChop {
    #[tracing::instrument]
    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        let params = inputs.params();
        params.enable_param("Scale", self.params.apply_scale);

        tracing::info!("Executing chop with params: {:?}", self.params);
        for i in 0..output.num_channels() {
            for j in 0..output.num_samples() {
                let cur_value = match self.params.operation {
                    Operation::Add => (i as f32) + (j as f32),
                    Operation::Multiply => (i as f32) * (j as f32),
                    Operation::Power => (i as f32).powf(j as f32),
                };
                let scale = if self.params.apply_scale {
                    self.params.scale
                } else {
                    1.0
                };
                let cur_value = cur_value * scale;
                output[i][j] = cur_value;
            }
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            timeslice: false,
            input_match_index: 0,
        }
    }

    fn channel_name(&self, index: usize, _inputs: &OperatorInputs<ChopInput>) -> String {
        format!("chan{}", index)
    }

    fn output_info(&self, _inputs: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        Some(ChopOutputInfo {
            num_channels: self.params.num_channels,
            num_samples: self.params.length,
            start_index: 0,
            ..Default::default()
        })
    }
}

fn increment_value(shmem_flink: &str) -> u8 {
    // Create or open the shared memory mapping
    let shmem = match ShmemConf::new().size(4096).flink(shmem_flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open().unwrap(),
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {shmem_flink} : {e}");
            return 0;
        }
    };

    let mut raw_ptr = shmem.as_ptr();
    let is_init: &mut AtomicU8;

    unsafe {
        is_init = &mut *(raw_ptr as *mut u8 as *mut AtomicU8);
        raw_ptr = raw_ptr.add(8);
    };

    // Initialize or wait for initialized mutex
    let mutex = if shmem.is_owner() {
        is_init.store(0, Ordering::Relaxed);
        // Initialize the mutex
        let (lock, _bytes_used) = unsafe {
            Mutex::new(
                raw_ptr,                                    // Base address of Mutex
                raw_ptr.add(Mutex::size_of(Some(raw_ptr))), // Address of data protected by mutex
            )
            .unwrap()
        };
        is_init.store(1, Ordering::Relaxed);
        lock
    } else {
        // wait until mutex is initialized
        while is_init.load(Ordering::Relaxed) != 1 {}
        // Load existing mutex
        let (lock, _bytes_used) = unsafe {
            Mutex::from_existing(
                raw_ptr,                                    // Base address of Mutex
                raw_ptr.add(Mutex::size_of(Some(raw_ptr))), // Address of data  protected by mutex
            )
            .unwrap()
        };
        lock
    };

    let mut guard = mutex.lock().unwrap();
    // Cast mutex data to &mut u8
    let val: &mut u8 = unsafe { &mut **guard };
    *val += 1;

    return *val
}

chop_plugin!(SharedMemoryChop);
