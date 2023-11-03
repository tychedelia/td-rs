use autocxx::prelude::*;

use std::ffi::CString;

use std::ops::{Index, IndexMut};
use std::pin::Pin;
use ref_cast::RefCast;

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
        let table = Vec::new();
        let mut table_out = DatTableOutput {
            output: self.output,
            table,
        };
        table_out
            .table
            .resize(table_out.table_size().iter().product(), T::default());
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

pub struct DatTableOutput<'execute, T> {
    output: Pin<&'execute mut cxx::DAT_Output>,
    table: Vec<T>,
}

impl<'execute, T> DatTableOutput<'execute, T>
where
    T: CellType<'execute> + Default,
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
            self.output.as_mut().setTableSize(rows as i32, cols as i32);
            self.table.resize(rows * cols, T::default());
        }
    }
}

/// A type which can be used as a cell in a DAT table. Should not be implemented manually or used
/// directly.
pub trait CellType<'execute>
where
    Self: Clone,
{
    /// Get a reference to the value of this cell from the table.
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self;
    /// Set the value of this cell in the table.
    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self);
}

impl<'execute> CellType<'execute> for f64 {
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self {
        let mut out = f64::default();
        let [rows, _] = table.table_size();
        let offset = row * rows + col;
        unsafe {
            table
                .output
                .as_ref()
                .getCellDouble(row as i32, col as i32, &mut out);
        }
        let ptr = table.table.as_ptr();

        unsafe {
            let y = ptr.offset(offset as isize) as *mut f64;
            *y = out;
        }

        &table.table[offset]
    }

    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self) {
        let [rows, _] = table.table_size();
        let offset = row * rows + col;
        table.table[offset] = value.clone();
        unsafe {
            table
                .output
                .as_mut()
                .setCellDouble(row as i32, col as i32, value);
        }
    }
}

impl<'execute> CellType<'execute> for i32 {
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self {
        let mut out = i32::default();
        let [rows, _] = table.table_size();
        let offset = row * rows + col;
        unsafe {
            table
                .output
                .as_ref()
                .getCellInt(row as i32, col as i32, &mut out);
        }
        let ptr = table.table.as_ptr();

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
            table
                .output
                .as_mut()
                .setCellInt(row as i32, col as i32, value);
        }
    }
}

impl<'execute> CellType<'execute> for String {
    fn get<'a>(table: &'a DatTableOutput<'execute, Self>, row: usize, col: usize) -> &'a Self {
        let rows = table.table_size()[0].clone();
        let offset = row.clone() * rows + col.clone();
        let out = unsafe {
            let out = table.output.as_ref().getCellString(row as i32, col as i32);
            std::ffi::CStr::from_ptr(out).to_str().unwrap()
        };

        let ptr = table.table.as_ptr();

        unsafe {
            let y = ptr.offset(offset.clone() as isize) as *mut &str;
            *y = out;
        }

        &table.table[offset]
    }

    fn set(table: &mut DatTableOutput<Self>, row: usize, col: usize, value: Self) {
        let rows = table.table_size()[0];
        let offset = row * rows + col;
        table.table[offset] = value.clone();
        let cstr = std::ffi::CString::new(value).unwrap();
        unsafe {
            table
                .output
                .as_mut()
                .setCellString(row as i32, col as i32, cstr.as_ptr());
        }
    }
}

impl<'execute, T> Index<[usize; 2]> for DatTableOutput<'execute, T>
where
    T: CellType<'execute> + Default,
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, col] = index;
        self.get(row, col)
    }
}

impl<'execute, T> IndexMut<[usize; 2]> for DatTableOutput<'execute, T>
where
    T: CellType<'execute> + Default,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, col] = index;
        let [rows, _] = self.table_size();
        let out = T::default();
        self.table[row * rows + col] = out;
        //self.set(row, col, out);
        &mut self.table[row * rows + col]
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
    fn general_info(&self, _input: &OperatorInputs<DatInput>) -> DatGeneralInfo {
        DatGeneralInfo::default()
    }

    fn execute(&mut self, _output: DatOutput, _input: &OperatorInputs<DatInput>) {
        // Do nothing by default.
    }
}

#[macro_export]
macro_rules! dat_plugin {
    ($plugin_ty:ty) => {
        use td_rs_dat::cxx::c_void;
        use td_rs_dat::cxx::OP_CustomOPInfo;
        use td_rs_dat::NodeInfo;

        #[no_mangle]
        pub extern "C" fn dat_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) {
            unsafe {
                td_rs_dat::op_info::<$plugin_ty>(op_info);
            }
        }

        #[no_mangle]
        pub extern "C" fn dat_new_impl(info: NodeInfo) -> Box<dyn Dat> {
            Box::new(<$plugin_ty>::new(info))
        }
    };
}
