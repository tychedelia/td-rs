use crate::cxx;
use ref_cast::RefCast;
use std::pin::Pin;
use std::thread::JoinHandle;
use td_rs_base::{GetInput, OperatorInputs};

pub const NUM_CPU_PIXEL_BUFFERS: usize = 3;

#[derive(Debug, Default)]
pub struct DownloadOptions {
    pub vertical_flip: bool,
    pub cpu_mem_pixel_type: CpuMemPixelType,
}

#[derive(Debug, Default)]
pub enum CpuMemPixelType {
    // 8-bit per color, BGRA pixels. This is preferred for 4 channel 8-bit data
    #[default]
    BGRA8Fixed = 0,
    // 8-bit per color, RGBA pixels. Only use this one if absolutely nesseary.
    RGBA8Fixed,
    // 32-bit float per color, RGBA pixels
    RGBA32Float,

    // A few single and two channel versions of the above
    R8Fixed,
    RG8Fixed,
    R32Float,
    RG32Float,

    R16Fixed = 100,
    RG16Fixed,
    RGBA16Fixed,

    R16Float = 200,
    RG16Float,
    RGBA16Float,
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct TopCpuInput {
    input: cxx::OP_TOPInput,
}

impl TopCpuInput {
    pub fn width(&self) -> usize {
        self.input.width as usize
    }

    pub fn height(&self) -> usize {
        self.input.height as usize
    }
}

trait DownloadTexture {
    fn texture<T>(&self, top: &TopCpuInput, opts: DownloadOptions) -> Option<&[T]>;
    fn texture_blocking<T>(&self, top: &TopCpuInput, opts: DownloadOptions) -> &[T];
}

impl<'execute> DownloadTexture for OperatorInputs<'execute, TopCpuInput> {
    fn texture<T>(&self, top: &TopCpuInput, opts: DownloadOptions) -> Option<&[T]> {
        let opts = cxx::OP_TOPInputDownloadOptions {
            downloadType: cxx::OP_TOPInputDownloadType::Delayed,
            verticalFlip: opts.vertical_flip,
            cpuMemPixelType: match opts.cpu_mem_pixel_type {
                CpuMemPixelType::BGRA8Fixed => cxx::OP_CPUMemPixelType::BGRA8Fixed,
                CpuMemPixelType::RGBA8Fixed => cxx::OP_CPUMemPixelType::RGBA8Fixed,
                CpuMemPixelType::RGBA32Float => cxx::OP_CPUMemPixelType::RGBA32Float,
                CpuMemPixelType::R8Fixed => cxx::OP_CPUMemPixelType::R8Fixed,
                CpuMemPixelType::RG8Fixed => cxx::OP_CPUMemPixelType::RG8Fixed,
                CpuMemPixelType::R32Float => cxx::OP_CPUMemPixelType::R32Float,
                CpuMemPixelType::RG32Float => cxx::OP_CPUMemPixelType::RG32Float,
                CpuMemPixelType::R16Fixed => cxx::OP_CPUMemPixelType::R16Fixed,
                CpuMemPixelType::RG16Fixed => cxx::OP_CPUMemPixelType::RG16Fixed,
                CpuMemPixelType::RGBA16Fixed => cxx::OP_CPUMemPixelType::RGBA16Fixed,
                CpuMemPixelType::R16Float => cxx::OP_CPUMemPixelType::R16Float,
                CpuMemPixelType::RG16Float => cxx::OP_CPUMemPixelType::RG16Float,
                CpuMemPixelType::RGBA16Float => cxx::OP_CPUMemPixelType::RGBA16Float,
            },
        };

        unsafe {
            let buf = self.inputs.getTOPDataInCPUMemory(&top.input, &opts);
            if buf.is_null() {
                return None;
            }
            let buf = buf as *const _ as *const T;
            let buf = std::slice::from_raw_parts(buf, top.width() * top.height());
            Some(buf)
        }
    }

    fn texture_blocking<T>(&self, top: &TopCpuInput, opts: DownloadOptions) -> &[T] {
        let opts = cxx::OP_TOPInputDownloadOptions {
            downloadType: cxx::OP_TOPInputDownloadType::Delayed,
            verticalFlip: opts.vertical_flip,
            cpuMemPixelType: match opts.cpu_mem_pixel_type {
                CpuMemPixelType::BGRA8Fixed => cxx::OP_CPUMemPixelType::BGRA8Fixed,
                CpuMemPixelType::RGBA8Fixed => cxx::OP_CPUMemPixelType::RGBA8Fixed,
                CpuMemPixelType::RGBA32Float => cxx::OP_CPUMemPixelType::RGBA32Float,
                CpuMemPixelType::R8Fixed => cxx::OP_CPUMemPixelType::R8Fixed,
                CpuMemPixelType::RG8Fixed => cxx::OP_CPUMemPixelType::RG8Fixed,
                CpuMemPixelType::R32Float => cxx::OP_CPUMemPixelType::R32Float,
                CpuMemPixelType::RG32Float => cxx::OP_CPUMemPixelType::RG32Float,
                CpuMemPixelType::R16Fixed => cxx::OP_CPUMemPixelType::R16Fixed,
                CpuMemPixelType::RG16Fixed => cxx::OP_CPUMemPixelType::RG16Fixed,
                CpuMemPixelType::RGBA16Fixed => cxx::OP_CPUMemPixelType::RGBA16Fixed,
                CpuMemPixelType::R16Float => cxx::OP_CPUMemPixelType::R16Float,
                CpuMemPixelType::RG16Float => cxx::OP_CPUMemPixelType::RG16Float,
                CpuMemPixelType::RGBA16Float => cxx::OP_CPUMemPixelType::RGBA16Float,
            },
        };

        unsafe {
            let buf = self.inputs.getTOPDataInCPUMemory(&top.input, &opts);
            if buf.is_null() {
                return panic!("Failed to download texture");
            }
            let buf = buf as *const _ as *const T;
            let buf = std::slice::from_raw_parts(buf, top.width() * top.height());
            buf
        }
    }
}

impl<'execute> GetInput<'execute, TopCpuInput> for OperatorInputs<'execute, TopCpuInput> {
    fn num_inputs(&self) -> usize {
        self.inputs.getNumInputs() as usize
    }

    fn input(&self, index: usize) -> Option<&'execute TopCpuInput> {
        let input = self.inputs.getInputTOP(index as i32);
        if input.is_null() {
            None
        } else {
            Some(TopCpuInput::ref_cast(unsafe { &*input }))
        }
    }
}

pub struct TopCpuOutput<'execute> {
    tasks: Vec<JoinHandle<()>>,
    specs: Pin<&'execute mut cxx::TOP_OutputFormatSpecs>,
}

impl<'execute> TopCpuOutput<'execute> {
    pub fn new(specs: Pin<&'execute mut cxx::TOP_OutputFormatSpecs>) -> Self {
        Self {
            tasks: Vec::new(),
            specs,
        }
    }

    pub fn width(&self) -> usize {
        self.specs.width as usize
    }

    pub fn height(&self) -> usize {
        self.specs.height as usize
    }

    // pub fn exec(&mut self, buf: &mut [u32], task: T) {
    //     if self.tasks.len() < NUM_CPU_PIXEL_BUFFERS {
    //         self.tasks.push(std::thread::spawn(move || task(buf)));
    //         return ();
    //     }

    //     let task = self.tasks.first().unwrap();
    //     task.join().expect("thread panicked");
    //     self.tasks.remove(0);
    // }
}
