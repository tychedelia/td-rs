pub mod cxx;

use std::ffi;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::pin::Pin;
use autocxx::cxx::UniquePtr;

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

impl From<NumericParameter> for cxx::OP_NumericParameter {
    fn from(param: NumericParameter) -> Self {
        cxx::OP_NumericParameter {
            name: ffi::CString::new(param.name).unwrap().into_raw(),
            label: ffi::CString::new(param.label).unwrap().into_raw(),
            page: ffi::CString::new(param.page).unwrap().into_raw(),
            defaultValues: param.default_values,
            minValues: param.min_values,
            maxValues: param.max_values,
            clampMins: param.clamp_mins,
            clampMaxes: param.clamp_maxes,
            minSliders: param.min_sliders,
            maxSliders: param.max_sliders,
            reserved: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StringParameter {
    pub name: String,
    pub label: String,
    pub page: String,
    pub default_value: String,
}

impl From<StringParameter> for cxx::OP_StringParameter {
    fn from(param: StringParameter) -> Self {
        cxx::OP_StringParameter {
            name: ffi::CString::new(param.name).unwrap().into_raw(),
            label: ffi::CString::new(param.label).unwrap().into_raw(),
            page: ffi::CString::new(param.page).unwrap().into_raw(),
            defaultValue: ffi::CString::new(param.default_value).unwrap().into_raw(),
            reserved: Default::default(),
        }
    }
}

pub struct ParameterManager<'execute> {
    manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>,
}

impl<'execute> ParameterManager<'execute> {
    pub fn new(
        mut manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>
    ) -> ParameterManager {
        Self {
            manager
        }
    }

    pub fn append_float(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendFloat(&param, 0);
    }

    pub fn append_pulse(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendPulse(&param);
    }

    pub fn append_int(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendInt(&param, 0);
    }

    pub fn append_xy(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXY(&param);
    }

    pub fn append_xyz(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXYZ(&param);
    }

    pub fn append_uv(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUV(&param);
    }

    pub fn append_uvw(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUVW(&param);
    }

    pub fn append_rgb(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGB(&param);
    }

    pub fn append_rgba(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGBA(&param);
    }

    pub fn append_toggle(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendToggle(&param);
    }

    pub fn append_string(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendString(&param);
    }

    pub fn append_file(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFile(&param);
    }

    pub fn append_folder(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFolder(&param);
    }

    pub fn append_dat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendDAT(&param);
    }

    pub fn append_chop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCHOP(&param);
    }

    pub fn append_top(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendTOP(&param);
    }

    pub fn append_object(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendObject(&param);
    }

    // pub fn append_menu(&mut self, param: StringParameter, names: &[&str], labels: &[&str]) {
    //     self.manager.as_mut().appendMenu(&param, names, labels);
    // }

    // pub fn append_string_menu(&mut self, param: StringParameter, names: &[&str], labels: &[&str]) {
    //     self.manager.as_mut().appendStringMenu(&param, names.len(), names.as_ptr(), labels.as_ptr());
    // }

    pub fn append_sop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendSOP(&param);
    }

    pub fn append_python(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPython(&param);
    }

    pub fn append_op(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendOP(&param);
    }

    pub fn append_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCOMP(&param);
    }

    pub fn append_mat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendMAT(&param);
    }

    pub fn append_panel_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPanelCOMP(&param);
    }

    pub fn append_header(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendHeader(&param);
    }

    pub fn append_momentary(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendMomentary(&param);
    }

    pub fn append_wh(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendWH(&param);
    }
}

/// Input to an operator, which can be used to get parameters, channels,
/// and other information.
pub struct OperatorInput<'execute> {
    input: Pin<&'execute crate::cxx::OP_Inputs>,
}

impl<'execute> OperatorInput<'execute> {
    /// Create a new operator input.
    pub fn new(input: Pin<&'execute crate::cxx::OP_Inputs>) -> OperatorInput<'execute> {
        Self { input }
    }

    /// Get a float parameter.
    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        unsafe { self.input.getParDouble(ffi::CString::new(name).unwrap().into_raw(), index as i32) }
    }

    /// Get an integer parameter.
    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        unsafe { self.input.getParInt(ffi::CString::new(name).unwrap().into_raw(), index as i32) }
    }

    /// Get a string parameter.
    pub fn get_string(&self, name: &str) -> &str {
        unsafe {
            let res = self.input.getParString(ffi::CString::new(name).unwrap().into_raw());
            ffi::CStr::from_ptr(res).to_str().unwrap()
        }
    }
}