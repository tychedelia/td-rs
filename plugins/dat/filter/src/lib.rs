use td_rs_dat::*;
use td_rs_derive::{Param, Params};

#[derive(Param, Default, Clone, Debug)]
enum FilterType {
    UpperCamelCase,
    #[default]
    LowerCase,
    UpperCase,
}

#[derive(Params, Default, Clone, Debug)]
struct FilterDatParams {
    #[param(label = "Case", page = "Filter")]
    case: FilterType,
    #[param(label = "Keep Spaces", page = "Filter")]
    keep_spaces: bool,
}

/// Struct representing our DAT's state
pub struct FilterDat {
    params: FilterDatParams,
}

/// Impl block providing default constructor for plugin
impl FilterDat {
    pub(crate) fn new(info: NodeInfo) -> Self {
        Self {
            params: Default::default(),
        }
    }
}

impl OpInfo for FilterDat {
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    // This Dat takes no input
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 1;
}

impl Op for FilterDat {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Dat for FilterDat {
    fn execute(&mut self, output: DatOutput, inputs: &OperatorInputs<DatInput>) {
        if let Some(input) = inputs.input(0) {
            if input.is_table() {
                let mut output = output.table::<String>();
                let [rows, cols] = input.table_size();
                output.set_table_size(rows, cols);
                for row in 0..rows {
                    for col in 0..cols {
                        if let Some(cell) = input.cell(row.clone(), col) {
                            match self.params.case {
                                FilterType::UpperCamelCase => {
                                    let formatted =
                                        to_camel_case(cell, self.params.keep_spaces.clone());
                                    output[[row, col]] = formatted;
                                }
                                FilterType::LowerCase => {
                                    let formatted =
                                        to_lower_case(cell, self.params.keep_spaces.clone());
                                    output[[row, col]] = formatted;
                                }
                                FilterType::UpperCase => {
                                    let formatted =
                                        to_upper_case(cell, self.params.keep_spaces.clone());
                                    output[[row, col]] = formatted;
                                }
                            }
                        }
                    }
                }
            } else {
                let mut output = output.text();
                match self.params.case {
                    FilterType::UpperCamelCase => {
                        let formatted =
                            to_camel_case(input.text(), self.params.keep_spaces.clone());
                        output.set_text(&formatted);
                    }
                    FilterType::LowerCase => {
                        let formatted =
                            to_lower_case(input.text(), self.params.keep_spaces.clone());
                        output.set_text(&formatted);
                    }
                    FilterType::UpperCase => {
                        let formatted =
                            to_upper_case(input.text(), self.params.keep_spaces.clone());
                        output.set_text(&formatted);
                    }
                }
            }
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<DatInput>) -> DatGeneralInfo {
        DatGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
        }
    }
}

pub fn to_camel_case(s: &str, keep_spaces: bool) -> String {
    let mut out = String::new();
    let mut next_upper = true;

    for c in s.chars() {
        if c.is_whitespace() {
            next_upper = true;
            if keep_spaces {
                out.push(c);
            }
        } else if next_upper {
            out.push(c.to_ascii_uppercase());
            next_upper = false;
        } else {
            out.push(c.to_ascii_lowercase());
        }
    }

    out
}

pub fn change_case(s: &str, keep_spaces: bool, upper: bool) -> String {
    let mut out = String::new();

    for c in s.chars() {
        if keep_spaces || !c.is_whitespace() {
            out.push(if upper {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            });
        }
    }

    out
}

pub fn to_upper_case(s: &str, keep_spaces: bool) -> String {
    change_case(s, keep_spaces, true)
}

pub fn to_lower_case(s: &str, keep_spaces: bool) -> String {
    change_case(s, keep_spaces, false)
}

dat_plugin!(FilterDat);
