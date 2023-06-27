#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]

use autocxx::prelude::*;
use ref_cast::RefCast;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::{pin, Pin};
use std::rc::Rc;
use std::sync::Arc;
pub use td_rs_base::dat::*;
pub use td_rs_base::param::OperatorParams;
pub use td_rs_base::*;

pub mod cxx;

#[derive(Debug, Default)]
pub struct DatGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
}

pub struct DatOutput<'execute> {
    output: Pin<&'execute mut cxx::DAT_Output>,
}

impl<'execute> DatOutput<'execute> {
    pub fn new(output: Pin<&'execute mut cxx::DAT_Output>) -> Self {
        Self { output }
    }

    pub fn table(mut self) -> DatTableOutput<'execute> {
        self.output
            .as_mut()
            .setOutputDataType(cxx::DAT_OutDataType::Table);
        DatTableOutput {
            output: self.output,
        }
    }

    pub fn text(mut self) -> DatTextOutput<'execute> {
        self.output
            .as_mut()
            .setOutputDataType(cxx::DAT_OutDataType::Text);
        DatTextOutput {
            output: self.output,
        }
    }
}

pub struct DatTableOutput<'execute> {
    output: Pin<&'execute mut cxx::DAT_Output>,
}

impl<'execute> DatTableOutput<'execute> {}

pub trait Cell : Copy {
    fn get(table: &DatTableOutput, row: usize, col: usize) -> Self;
    fn set(table: &mut DatTableOutput, row: usize, col: usize, value: Self);
}

impl Cell for &str {
    fn get(table: &DatTableOutput, row: usize, col: usize) -> Self {
        // let chars = table.output.as_ref().getCellString(row as i32, col as i32);
        // let c_str = unsafe { CStr::from_ptr(chars) };
        // c_str.to_str().expect("Failed to convert CStr to str")
        ""
    }

    fn set(table: &mut DatTableOutput, row: usize, col: usize, value: Self) {
        unsafe {
            // let c_str = CString::new(value).unwrap();
            // if !table
            //     .output
            //     .as_mut()
            //     .setCellString(row as i32, col as i32, c_str.as_ptr())
            // {
            //     panic!("Failed to set cell");
            // }
        }
    }
}
impl Cell for i32 {
    fn get(table: &DatTableOutput, row: usize, col: usize) -> Self {
        unsafe {
            let mut value = 0;
            // if !table
            //     .output
            //     .as_ref()
            //     .getCellInt(row as i32, col as i32, &mut value)
            // {
            //     panic!("Failed to get cell");
            // }
            value
        }
    }

    fn set(table: &mut DatTableOutput, row: usize, col: usize, value: Self) {
        // if !table
        //     .output
        //     .as_mut()
        //     .setCellInt(row as i32, col as i32, value as i32)
        // {
        //     panic!("Failed to set cell");
        // }
    }
}

impl <T> From<[usize; 2]> for CellIdx<T> {
    fn from(idx: [usize; 2]) -> Self {
        Self(idx[0], idx[1], std::marker::PhantomData)
    }
}

impl Cell for f64 {
    fn get(table: &DatTableOutput, row: usize, col: usize) -> Self {
        unsafe {
            let mut value = 0.0;
            // if !table
            //     .output
            //     .as_ref()
            //     .getCellDouble(row as i32, col as i32, &mut value)
            // {
            //     panic!("Failed to get cell");
            // }
            value
        }
    }

    fn set(table: &mut DatTableOutput, row: usize, col: usize, value: Self) {
        // if !table
        //     .output
        //     .as_mut()
        //     .setCellDouble(row as i32, col as i32, value)
        // {
        //     panic!("Failed to set cell");
        // }
    }
}

pub struct CellIdx<T>(usize, usize, std::marker::PhantomData<T>);

impl<T> CellIdx<T> {
    pub fn of(row: usize, col: usize) -> Self {
        Self(row, col, std::marker::PhantomData)
    }
}

impl<'execute, T: Cell + Copy> Index<CellIdx<T>> for DatTableOutput<'execute> {
    type Output = T;

    fn index(&self, index: CellIdx<T>) -> &Self::Output {
        &self.get(&self, index.0, index.1)
    }
}

pub struct DatTextOutput<'execute> {
    output: Pin<&'execute mut cxx::DAT_Output>,
}

impl<'execute> DatTextOutput<'execute> {
    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let c_str = CString::new(text).unwrap();
            self.output.as_mut().setText(c_str.as_ptr());
        }
    }
}

pub trait Dat: Op {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn general_info(&self, input: &OperatorInputs<DatInput>) -> DatGeneralInfo {
        DatGeneralInfo::default()
    }

    fn execute(&mut self, output: &mut DatOutput, input: &OperatorInputs<DatInput>) {
        // Do nothing by default.
    }
}

#[macro_export]
macro_rules! dat_plugin {
    ($plugin_ty:ty) => {
        use td_rs_dat::cxx::OP_CustomOPInfo;

        #[no_mangle]
        pub extern "C" fn dat_get_plugin_info_impl(mut op_info: Pin<&mut OP_CustomOPInfo>) {
            unsafe {
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_TYPE).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.opType, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_LABEL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.opLabel, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_ICON).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.opIcon, new_string_ptr);
                op_info.minInputs = <$plugin_ty>::MIN_INPUTS as i32;
                op_info.maxInputs = <$plugin_ty>::MAX_INPUTS as i32;
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_NAME).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.authorName, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_EMAIL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.authorEmail, new_string_ptr);
                op_info.majorVersion = <$plugin_ty>::MAJOR_VERSION;
                op_info.minorVersion = <$plugin_ty>::MINOR_VERSION;
                let new_string = std::ffi::CString::new(<$plugin_ty>::PYTHON_VERSION).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_dat::cxx::setString(op_info.pythonVersion, new_string_ptr);
                op_info.cookOnStart = <$plugin_ty>::COOK_ON_START;
            }
        }

        #[no_mangle]
        pub extern "C" fn dat_new_impl() -> Box<dyn Dat> {
            Box::new(<$plugin_ty>::new())
        }
    };
}
