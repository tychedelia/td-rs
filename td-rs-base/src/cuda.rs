#[derive(Debug)]
pub struct CudaContext {
    context: Option<std::sync::Arc<cudarc::driver::CudaContext>>,
}

impl Default for CudaContext {
    fn default() -> Self {
        Self { context: None }
    }
}

impl CudaContext {
    pub fn new(device_ordinal: usize) -> Result<Self, anyhow::Error> {
        let context = cudarc::driver::CudaContext::new(device_ordinal)
            .map_err(|e| anyhow::anyhow!("Failed to create CUDA context: {:?}", e))?;
        Ok(Self {
            context: Some(context),
        })
    }

    pub fn cudarc_context(&self) -> Option<&std::sync::Arc<cudarc::driver::CudaContext>> {
        self.context.as_ref()
    }

    pub fn default_stream(&self) -> Option<std::sync::Arc<cudarc::driver::CudaStream>> {
        self.context.as_ref().map(|ctx| ctx.default_stream())
    }
}
