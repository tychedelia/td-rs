use td_rs_dat::chop::ChopInput;
use td_rs_dat::*;
use td_rs_derive::{Param, Params};

#[derive(Params, Default, Clone, Debug)]
struct DynamicMenuDatParams {
    #[param(label = "Menu")]
    menu: DynamicMenuParam,
}

/// Struct representing our DAT's state
pub struct DynamicMenuDat {
    params: DynamicMenuDatParams,
}

impl OpNew for DynamicMenuDat {
    fn new(_info: NodeInfo) -> Self {
        Self {
            params: Default::default(),
        }
    }
}

impl OpInfo for DynamicMenuDat {
    const OPERATOR_TYPE: &'static str = "Dynamicmenu";
    const OPERATOR_LABEL: &'static str = "Dynamic Menu";
    const MIN_INPUTS: usize = 1;
    // This Dat takes no input
    const MAX_INPUTS: usize = 1;
}

impl Op for DynamicMenuDat {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Dat for DynamicMenuDat {
    fn general_info(&self, _inputs: &OperatorInputs<DatInput>) -> DatGeneralInfo {
        DatGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
        }
    }

    fn execute(&mut self, output: DatOutput, inputs: &OperatorInputs<DatInput>) {
        if let Some(input) = inputs.input(0) {
            match input.dat_type() {
                DatType::Text => {
                    if let Some(output_text) = &self.params.menu.0 {
                        output
                            .text()
                            .set_text(&format!("Selected: {}", output_text));
                    } else {
                        output.text().set_text("");
                    }
                }
                _ => self.set_warning("Input must be a text DAT"),
            }
        }
    }

    fn build_dynamic_menu(
        &mut self,
        inputs: &OperatorInputs<DatInput>,
        menu_info: &mut DynamicMenuInfo,
    ) {
        if menu_info.param_name() == "Menu" {
            if let Some(input) = inputs.input(0) {
                match input.dat_type() {
                    DatType::Text => {
                        let text = input.text();
                        let labels = text
                            .split('\n')
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        for label in labels {
                            let name = label.replace(" ", "");
                            menu_info.add_menu_entry(&name, &label);
                        }
                    }
                    _ => self.set_warning("Input must be a text DAT"),
                }
            }
        }
    }
}

impl DynamicMenuDat {}

dat_plugin!(DynamicMenuDat);
