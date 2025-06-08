use crate::cxx;
#[cfg(feature = "cuda")]
use cudarc::runtime::sys;
use std::sync::Arc;

// Re-export CudaArrayInfo from td-rs-base
#[cfg(feature = "cuda")]
pub use td_rs_base::top::CudaArrayInfo;

#[derive(Debug)]
pub struct CudaAcquireInfo {
    pub stream: sys::cudaStream_t,
}

impl Default for CudaAcquireInfo {
    fn default() -> Self {
        Self {
            stream: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
pub struct CudaOutputInfo {
    pub stream: sys::cudaStream_t,
    pub texture_desc: crate::TextureDesc,
    pub color_buffer_index: u32,
}

impl Default for CudaOutputInfo {
    fn default() -> Self {
        Self {
            stream: std::ptr::null_mut(),
            texture_desc: crate::TextureDesc::default(),
            color_buffer_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct CudaSurface {
    surface: sys::cudaSurfaceObject_t,
    _marker: std::marker::PhantomData<()>,
}

impl CudaSurface {
    pub unsafe fn from_external_array(array: *mut sys::cudaArray) -> Result<Self, anyhow::Error> {
        if array.is_null() {
            return Err(anyhow::anyhow!("Invalid CUDA array pointer"));
        }

        let mut surface = 0;
        let desc = sys::cudaResourceDesc {
            resType: sys::cudaResourceType::cudaResourceTypeArray,
            res: sys::cudaResourceDesc__bindgen_ty_1 {
                array: sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 { array },
            },
        };

        let result = sys::cudaCreateSurfaceObject(&mut surface, &desc);
        if result != sys::cudaError::cudaSuccess {
            return Err(anyhow::anyhow!(
                "CUDA surface creation failed: {:?}",
                result
            ));
        }

        Ok(Self {
            surface,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn handle(&self) -> sys::cudaSurfaceObject_t {
        self.surface
    }
}

impl Drop for CudaSurface {
    fn drop(&mut self) {
        if self.surface != 0 {
            unsafe {
                sys::cudaDestroySurfaceObject(self.surface);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct SurfaceCache {
    surfaces: std::collections::HashMap<*mut sys::cudaArray, CudaSurface>,
}

impl SurfaceCache {
    pub fn new() -> Self {
        Self {
            surfaces: std::collections::HashMap::new(),
        }
    }

    pub unsafe fn get_or_create(
        &mut self,
        array: *mut sys::cudaArray,
    ) -> Result<&CudaSurface, anyhow::Error> {
        if !self.surfaces.contains_key(&array) {
            let surface = CudaSurface::from_external_array(array)?;
            self.surfaces.insert(array, surface);
        }
        Ok(self.surfaces.get(&array).unwrap())
    }

    pub fn cleanup_invalid(&mut self, valid_arrays: &[*mut sys::cudaArray]) {
        self.surfaces.retain(|&k, _| valid_arrays.contains(&k));
    }

    pub fn clear(&mut self) {
        self.surfaces.clear();
    }
}
