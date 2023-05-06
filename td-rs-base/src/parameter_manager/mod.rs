use crate::cxx::ffi::{NumericParameter, StringParameter};
use std::pin::Pin;

pub struct ParameterManager<'execute> {
    manager: Pin<&'execute mut crate::cxx::ffi::ParameterManager>,
}

impl<'execute> ParameterManager<'execute> {
    pub fn new(
        manager: Pin<&'execute mut crate::cxx::ffi::ParameterManager>,
    ) -> ParameterManager<'execute> {
        Self { manager }
    }

    pub fn append_float(&self, param: NumericParameter) {
        println!("append_float: {:?}", param);
        self.manager.appendFloat(param);
    }

    pub fn append_pulse(&self, param: NumericParameter) {
        self.manager.appendPulse(param);
    }

    pub fn append_int(&self, param: NumericParameter) {
        self.manager.appendInt(param);
    }

    pub fn append_xy(&self, param: NumericParameter) {
        self.manager.appendXY(param);
    }

    pub fn append_xyz(&self, param: NumericParameter) {
        self.manager.appendXYZ(param);
    }

    pub fn append_uv(&self, param: NumericParameter) {
        self.manager.appendUV(param);
    }

    pub fn append_uvw(&self, param: NumericParameter) {
        self.manager.appendUVW(param);
    }

    pub fn append_rgb(&self, param: NumericParameter) {
        self.manager.appendRGB(param);
    }

    pub fn append_rgba(&self, param: NumericParameter) {
        self.manager.appendRGBA(param);
    }

    pub fn append_toggle(&self, param: NumericParameter) {
        self.manager.appendToggle(param);
    }

    pub fn append_string(&self, param: StringParameter) {
        self.manager.appendString(param);
    }

    pub fn append_file(&self, param: StringParameter) {
        self.manager.appendFile(param);
    }

    pub fn append_folder(&self, param: StringParameter) {
        self.manager.appendFolder(param);
    }

    pub fn append_dat(&self, param: StringParameter) {
        self.manager.appendDAT(param);
    }

    pub fn append_chop(&self, param: StringParameter) {
        self.manager.appendCHOP(param);
    }

    pub fn append_top(&self, param: StringParameter) {
        self.manager.appendTOP(param);
    }

    pub fn append_object(&self, param: StringParameter) {
        self.manager.appendObject(param);
    }

    pub fn append_menu(&self, param: StringParameter, names: &[&str], labels: &[&str]) {
        self.manager.appendMenu(param, names, labels);
    }

    pub fn append_string_menu(&self, param: StringParameter, names: &[&str], labels: &[&str]) {
        self.manager.appendStringMenu(param, names, labels);
    }

    pub fn append_sop(&self, param: StringParameter) {
        self.manager.appendSOP(param);
    }

    pub fn append_python(&self, param: StringParameter) {
        self.manager.appendPython(param);
    }

    pub fn append_op(&self, param: StringParameter) {
        self.manager.appendOP(param);
    }

    pub fn append_comp(&self, param: StringParameter) {
        self.manager.appendCOMP(param);
    }

    pub fn append_mat(&self, param: StringParameter) {
        self.manager.appendMAT(param);
    }

    pub fn append_panel_comp(&self, param: StringParameter) {
        self.manager.appendPanelCOMP(param);
    }

    pub fn append_header(&self, param: StringParameter) {
        self.manager.appendHeader(param);
    }

    pub fn append_momentary(&self, param: NumericParameter) {
        self.manager.appendMomentary(param);
    }

    pub fn append_wh(&self, param: NumericParameter) {
        self.manager.appendWH(param);
    }
}
