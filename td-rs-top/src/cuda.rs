pub mod surface {
    use cudarc::driver::{sys, DriverError};
    use std::mem::MaybeUninit;

    pub fn create() -> Result<sys::CUsurfObject, DriverError> {
        let mut surface = MaybeUninit::uninit();
        unsafe {
            sys::cuSurfObjectCreate(surface.as_mut_ptr(), std::ptr::null_mut()).result()?;
            Ok(surface.assume_init())
        }
    }

    pub fn destroy(surface: sys::CUsurfObject) -> Result<(), DriverError> {
        unsafe {
            sys::cuSurfObjectDestroy(surface).result()?;
            Ok(())
        }
    }

    pub fn get_resource_desc(surface: sys::CUsurfObject) -> Result<sys::CUDA_RESOURCE_DESC, DriverError> {
        let mut desc = MaybeUninit::uninit();
        unsafe {
            sys::cuSurfObjectGetResourceDesc(desc.as_mut_ptr(), surface).result()?;
            Ok(desc.assume_init())
        }
    }
}
