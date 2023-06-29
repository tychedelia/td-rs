#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]

use autocxx::prelude::*;
use ref_cast::RefCast;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::{pin, Pin};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
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

    pub fn table<T: CellType<'execute> + Default>(mut self) -> DatTableOutput<'execute, T> {
        self.output
            .as_mut()
            .setOutputDataType(cxx::DAT_OutDataType::Table);
        let mut table = Vec::new();
        let mut table_out = DatTableOutput {
            output: self.output,
            table,
        };
        table_out.table.resize(table_out.table_size().iter().product(), T::default());
        table_out
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

/// A type which can be used as a cell in a DAT table. Should not be implemented manually or used
/// directly.
pub trait CellType<'execute>
    where Self: Copy
{
    /// Get a reference to the value of this cell from the table.
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self;
    /// Set the value of this cell in the table.
    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self);
}


impl<'execute> CellType<'execute> for f64 {
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self {
        let mut out = f64::default();
        let rows = table.table_size()[0].clone();
        let offset = row.clone() * rows + col.clone();
        unsafe {
            table.output
                .as_ref()
                .getCellDouble(row as i32, col as i32, &mut out);
        }
        let ptr = table.table.as_ptr();


        /// SAFETY:
        /// 1. The size of the table is set whenever the table is created, and is reset when
        ///    the table is resized. The size of the table is always equal to the number of
        ///    rows times the number of columns.
        /// 2. The value read by reading the cell is valid for the lifetime `'execute`.
        unsafe {
            let y = ptr.offset(offset.clone() as isize) as *mut f64;
            *y = out;
        }

        &table.table[offset]
    }

    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self) {
        let rows = table.table_size()[0].clone();
        let offset = row.clone() * rows + col.clone();
        table.table[offset] = value.clone();
        unsafe {
            table.output
                .as_mut()
                .setCellDouble(row as i32, col as i32, value);
        }
    }
}

impl<'execute> CellType<'execute> for i32 {
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self {
        let mut out = i32::default();
        let rows = table.table_size()[0].clone();
        let offset = row.clone() * rows + col.clone();
        unsafe {
            table.output
                .as_ref()
                .getCellInt(row as i32, col as i32, &mut out);
        }
        let ptr = table.table.as_ptr();


        /// SAFETY:
        /// 1. The size of the table is set whenever the table is created, and is reset when
        ///    the table is resized. The size of the table is always equal to the number of
        ///    rows times the number of columns.
        /// 2. The value read by reading the cell is valid for the lifetime `'execute`.
        unsafe {
            let y = ptr.offset(offset.clone() as isize) as *mut i32;
            *y = out;
        }

        &table.table[offset]
    }

    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self) {
        let rows = table.table_size()[0].clone();
        let offset = row.clone() * rows + col.clone();
        table.table[offset] = value.clone();
        unsafe {
            table.output
                .as_mut()
                .setCellInt(row as i32, col as i32, value);
        }
    }
}

pub struct DatTableOutput<'execute, T> {
    output: Pin<&'execute mut cxx::DAT_Output>,
    table: Vec<T>,
}

impl <'execute, T, > Index<[usize; 2]> for DatTableOutput<'execute, T>
    where T: CellType<'execute> + Copy + Default
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = index[0].clone();
        let col = index[1].clone();
        self.get(row, col)
    }
}

impl<'execute, T> DatTableOutput<'execute, T>
    where T: CellType<'execute> + Default
{
    pub fn get(&self, row: usize, col: usize) -> &T {
        T::get(self, row, col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        T::set(self, row, col, value)
    }

    pub fn table_size(&self) -> [usize; 2] {
        let mut rows = 0;
        let mut cols = 0;
        unsafe {
            self.output.as_ref().getTableSize(&mut rows, &mut cols);
        }
        [rows as usize, cols as usize]
    }

    pub fn set_table_size(&mut self, rows: usize, cols: usize) {
        unsafe {
            self.output.as_mut().setTableSize(rows.clone() as i32, cols.clone() as i32);
            self.table.resize(rows * cols, T::default());
        }
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
