#[cxx::bridge(namespace = "td_rs_base::ffi")]
pub mod ffi {
    #[derive(Debug, Default)]
    pub struct NumericParameter {
        pub name: String,
        pub label: String,
        pub page: String,

        pub default_values: [f64; 4],
        pub min_values: [f64; 4],
        pub max_values: [f64; 4],
        pub clamp_mins: [bool; 4],
        pub clamp_maxes: [bool; 4],

        pub min_sliders: [f64; 4],
        pub max_sliders: [f64; 4],
    }

    #[derive(Debug, Default)]
    pub struct StringParameter {
        pub name: String,
        pub label: String,
        pub page: String,
        pub default_value: String,
    }

    unsafe extern "C++" {
        include!("parameter_manager/ParameterManager.h");
        pub type ParameterManager;
        pub fn appendFloat(&self, np: NumericParameter);
        pub fn appendPulse(&self, np: NumericParameter);
        pub fn appendInt(&self, np: NumericParameter);
        pub fn appendXY(&self, np: NumericParameter);
        pub fn appendXYZ(&self, np: NumericParameter);
        pub fn appendUV(&self, np: NumericParameter);
        pub fn appendUVW(&self, np: NumericParameter);
        pub fn appendRGB(&self, np: NumericParameter);
        pub fn appendRGBA(&self, np: NumericParameter);
        pub fn appendToggle(&self, np: NumericParameter);
        pub fn appendString(&self, sp: StringParameter);
        pub fn appendFile(&self, sp: StringParameter);
        pub fn appendFolder(&self, sp: StringParameter);
        pub fn appendDAT(&self, sp: StringParameter);
        pub fn appendCHOP(&self, sp: StringParameter);
        pub fn appendTOP(&self, sp: StringParameter);
        pub fn appendObject(&self, sp: StringParameter);
        pub fn appendMenu(&self, sp: StringParameter, names: &[&str], labels: &[&str]);
        pub fn appendStringMenu(&self, sp: StringParameter, names: &[&str], labels: &[&str]);
        pub fn appendSOP(&self, sp: StringParameter);
        pub fn appendPython(&self, sp: StringParameter);
        pub fn appendOP(&self, sp: StringParameter);
        pub fn appendCOMP(&self, sp: StringParameter);
        pub fn appendMAT(&self, sp: StringParameter);
        pub fn appendPanelCOMP(&self, sp: StringParameter);
        pub fn appendHeader(&self, sp: StringParameter);
        pub fn appendMomentary(&self, np: NumericParameter);
        pub fn appendWH(&self, np: NumericParameter);
    }

    unsafe extern "C++" {
        include!("operator_input/OperatorInput.h");
        pub type OperatorInput;
        pub fn getParDouble(&self, name: &str, index: i32) -> f64;
        pub fn getParDouble2(&self, name: &str) -> &[f64];
        pub fn getParDouble3(&self, name: &str) -> &[f64];
        pub fn getParDouble4(&self, name: &str) -> &[f64];
        pub fn getParInt(&self, name: &str, index: i32) -> i32;
        pub fn getParInt2(&self, name: &str) -> &[i32];
        pub fn getParInt3(&self, name: &str) -> &[i32];
        pub fn getParInt4(&self, name: &str) -> &[i32];
        pub fn getParString(&self, name: &str) -> &str;
    }
}