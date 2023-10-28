#![feature(min_specialization)]
use wasmtime::{Engine, Linker, Module, Store};

use td_rs_chop::*;
use td_rs_derive::Params;

#[derive(Params, Default, Clone)]
struct WasmChopParams {
    #[param(label = "Apply Scale", page = "Filter")]
    apply_scale: bool,
    #[param(label = "Scale", page = "Filter", min = - 10.0, max = 10.0)]
    scale: f32,
    #[param(label = "Wasm", page = "Wasm")]
    wasm: FileParam,
}

pub struct WasmChop {
    params: WasmChopParams,
    engine: Engine,
    module: Option<Module>,
}

impl OpNew for WasmChop {
    fn new(info: NodeInfo) -> Self {
        Self {
            params: WasmChopParams {
                ..Default::default()
            },
            engine: Engine::default(),
            module: None,
        }
    }
}

impl OpInfo for WasmChop {
    const OPERATOR_TYPE: &'static str = "Wasm";
    const OPERATOR_LABEL: &'static str = "Wasm";
    const MIN_INPUTS: usize = 1;
    const MAX_INPUTS: usize = 1;
}

impl Op for WasmChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn info_dat(&self) -> Option<Box<&dyn InfoDat>> {
        Some(Box::new(self))
    }
}

impl InfoDat for WasmChop {
    fn entry(&self, _index: usize, _entry_index: usize) -> String {
        let wasm_file = &self.params.wasm;
        if wasm_file.exists() && wasm_file.is_file() {
            wasmprinter::print_file(wasm_file.as_path()).expect("Failed to print wasm file")
        } else {
            "".to_string()
        }
    }

    fn size(&self) -> (u32, u32) {
        (1, 1)
    }
}

impl Chop for WasmChop {
    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        let params = inputs.params();
        params.enable_param("Wasm", false);
        params.enable_param("Scale", self.params.apply_scale);

        if let Some(input) = &inputs.input(0) {
            params.enable_param("Wasm", true);

            let wasm_file = &self.params.wasm;
            if wasm_file.exists() && wasm_file.is_file() {
                let module = Module::from_file(&self.engine.clone(), wasm_file.as_path())
                    .expect("Failed to load wasm file");
                self.module = Some(module);
            }

            if let Some(module) = &self.module {
                let mut linker = Linker::new(&self.engine.clone());

                let scale = self.params.scale;
                linker.func_wrap("env", "scale", move || scale).unwrap();

                let mut store = Store::new(&self.engine.clone(), ());
                let instance = match linker.instantiate(&mut store, module) {
                    Ok(instance) => instance,
                    Err(e) => {
                        self.set_error(&format!("Failed to instantiate module: {}", e));
                        return;
                    }
                };
                let execute = instance
                    .get_typed_func::<(u32, u32, f32), f32>(&mut store, "execute")
                    .expect("Failed to get execute function");

                for i in 0..output.num_channels() {
                    for j in 0..output.num_samples() {
                        let res = execute
                            .call(&mut store, (i as u32, j as u32, input[i][j]))
                            .expect("Failed to call execute function");
                        output[i][j] = res;
                    }
                }
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
}

chop_plugin!(WasmChop);
