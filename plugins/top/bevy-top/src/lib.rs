use anyhow::Result;
use ash::{khr, vk, Device, Instance};
use bevy::render::camera::{
    ManualTextureView, ManualTextureViewHandle, ManualTextureViews, RenderTarget,
};
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::render_resource::{Texture, TextureFormat};
use bevy::render::texture::DefaultImageSampler;
use bevy::render::Extract;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets, renderer::RenderDevice, texture::GpuImage, ExtractSchedule,
        RenderApp,
    },
    window::WindowPlugin,
};
use cudarc::driver::{sys, CudaContext};
use std::f32::consts::PI;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};
use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;
use wgpu::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages};

fn get_bytes_per_pixel(format: &PixelFormat) -> usize {
    match format {
        PixelFormat::Invalid => 0,

        PixelFormat::BGRA8Fixed
        | PixelFormat::RGBA8Fixed
        | PixelFormat::SBGRA8Fixed
        | PixelFormat::SRGBA8Fixed => 4,
        PixelFormat::MonoA8Fixed | PixelFormat::RG8Fixed => 2,
        PixelFormat::Mono8Fixed | PixelFormat::A8Fixed => 1,

        PixelFormat::RGBA16Fixed | PixelFormat::RGBA16Float => 8,
        PixelFormat::MonoA16Fixed
        | PixelFormat::MonoA16Float
        | PixelFormat::RG16Fixed
        | PixelFormat::RG16Float => 4,
        PixelFormat::Mono16Fixed
        | PixelFormat::Mono16Float
        | PixelFormat::A16Fixed
        | PixelFormat::A16Float => 2,

        PixelFormat::RGBA32Float => 16,
        PixelFormat::MonoA32Float | PixelFormat::RG32Float => 8,
        PixelFormat::Mono32Float | PixelFormat::A32Float => 4,

        PixelFormat::RGB10A2Fixed => 4,
        PixelFormat::RGB11Float => 4,
    }
}

fn get_preferred_output_format(input_format: &PixelFormat) -> PixelFormat {
    match input_format {
        PixelFormat::RGBA8Fixed => PixelFormat::BGRA8Fixed,
        PixelFormat::BGRA8Fixed => PixelFormat::BGRA8Fixed,
        PixelFormat::SRGBA8Fixed => PixelFormat::SBGRA8Fixed,
        PixelFormat::SBGRA8Fixed => PixelFormat::SBGRA8Fixed,

        PixelFormat::RGBA16Fixed => PixelFormat::RGBA16Fixed,
        PixelFormat::RGBA16Float => PixelFormat::RGBA16Float,

        PixelFormat::RGBA32Float => PixelFormat::RGBA32Float,

        PixelFormat::Mono8Fixed | PixelFormat::MonoA8Fixed | PixelFormat::RG8Fixed => {
            PixelFormat::BGRA8Fixed
        }
        PixelFormat::Mono16Fixed | PixelFormat::MonoA16Fixed | PixelFormat::RG16Fixed => {
            PixelFormat::RGBA16Fixed
        }
        PixelFormat::Mono32Float | PixelFormat::MonoA32Float | PixelFormat::RG32Float => {
            PixelFormat::RGBA32Float
        }

        _ => PixelFormat::BGRA8Fixed,
    }
}

fn pixel_format_to_vulkan_format(format: &PixelFormat) -> vk::Format {
    match format {
        PixelFormat::BGRA8Fixed => vk::Format::B8G8R8A8_UNORM,
        PixelFormat::RGBA8Fixed => vk::Format::R8G8B8A8_UNORM,
        PixelFormat::SBGRA8Fixed => vk::Format::B8G8R8A8_SRGB,
        PixelFormat::SRGBA8Fixed => vk::Format::R8G8B8A8_SRGB,

        PixelFormat::RGBA16Fixed => vk::Format::R16G16B16A16_UNORM,
        PixelFormat::RGBA16Float => vk::Format::R16G16B16A16_SFLOAT,

        PixelFormat::RGBA32Float => vk::Format::R32G32B32A32_SFLOAT,

        PixelFormat::Mono8Fixed => vk::Format::R8_UNORM,
        PixelFormat::Mono16Fixed => vk::Format::R16_UNORM,
        PixelFormat::Mono16Float => vk::Format::R16_SFLOAT,
        PixelFormat::Mono32Float => vk::Format::R32_SFLOAT,
        PixelFormat::RG8Fixed => vk::Format::R8G8_UNORM,
        PixelFormat::RG16Fixed => vk::Format::R16G16_UNORM,
        PixelFormat::RG16Float => vk::Format::R16G16_SFLOAT,
        PixelFormat::RG32Float => vk::Format::R32G32_SFLOAT,

        PixelFormat::A8Fixed => vk::Format::R8_UNORM,
        PixelFormat::A16Fixed => vk::Format::R16_UNORM,
        PixelFormat::A16Float => vk::Format::R16_SFLOAT,
        PixelFormat::A32Float => vk::Format::R32_SFLOAT,

        PixelFormat::MonoA8Fixed => vk::Format::R8G8_UNORM,
        PixelFormat::MonoA16Fixed => vk::Format::R16G16_UNORM,
        PixelFormat::MonoA16Float => vk::Format::R16G16_SFLOAT,
        PixelFormat::MonoA32Float => vk::Format::R32G32_SFLOAT,

        PixelFormat::RGB10A2Fixed => vk::Format::A2B10G10R10_UNORM_PACK32,
        PixelFormat::RGB11Float => vk::Format::B10G11R11_UFLOAT_PACK32,

        _ => vk::Format::B8G8R8A8_UNORM,
    }
}

fn pixel_format_to_wgpu_format(format: &PixelFormat) -> TextureFormat {
    match format {
        PixelFormat::BGRA8Fixed => TextureFormat::Bgra8Unorm,
        PixelFormat::RGBA8Fixed => TextureFormat::Rgba8Unorm,
        PixelFormat::SBGRA8Fixed => TextureFormat::Bgra8UnormSrgb,
        PixelFormat::SRGBA8Fixed => TextureFormat::Rgba8UnormSrgb,

        PixelFormat::RGBA16Fixed => TextureFormat::Rgba16Unorm,
        PixelFormat::RGBA16Float => TextureFormat::Rgba16Float,

        PixelFormat::RGBA32Float => TextureFormat::Rgba32Float,

        PixelFormat::Mono8Fixed => TextureFormat::R8Unorm,
        PixelFormat::Mono16Fixed => TextureFormat::R16Unorm,
        PixelFormat::Mono16Float => TextureFormat::R16Float,
        PixelFormat::Mono32Float => TextureFormat::R32Float,
        PixelFormat::RG8Fixed => TextureFormat::Rg8Unorm,
        PixelFormat::RG16Fixed => TextureFormat::Rg16Unorm,
        PixelFormat::RG16Float => TextureFormat::Rg16Float,
        PixelFormat::RG32Float => TextureFormat::Rg32Float,

        PixelFormat::A8Fixed => TextureFormat::R8Unorm,
        PixelFormat::A16Fixed => TextureFormat::R16Unorm,
        PixelFormat::A16Float => TextureFormat::R16Float,
        PixelFormat::A32Float => TextureFormat::R32Float,

        PixelFormat::MonoA8Fixed => TextureFormat::Rg8Unorm,
        PixelFormat::MonoA16Fixed => TextureFormat::Rg16Unorm,
        PixelFormat::MonoA16Float => TextureFormat::Rg16Float,
        PixelFormat::MonoA32Float => TextureFormat::Rg32Float,

        PixelFormat::RGB10A2Fixed => TextureFormat::Rgb10a2Unorm,
        PixelFormat::RGB11Float => TextureFormat::Rg11b10Ufloat,

        _ => TextureFormat::Bgra8Unorm,
    }
}

#[derive(Params, Default, Clone, Debug)]
struct BevyTopParams {
    #[param(label = "Test CUDA→Vulkan", default = 1.0)]
    test_param: f64,
}

pub struct BevyTop {
    params: BevyTopParams,
    context: Arc<Mutex<TopContext>>,
    bevy_app: Option<App>,
    textures: Option<ImportedTextures>,
    cuda_context: Option<Arc<CudaContext>>,
    cuda_stream: Option<cudarc::runtime::sys::cudaStream_t>,
}

struct SharedTexture {
    vulkan_memory: vk::DeviceMemory,
    vulkan_image: Option<vk::Image>,

    vulkan_device: Option<Device>,
    width: u32,
    height: u32,
    pixel_format: PixelFormat,
    actual_row_pitch: Option<usize>,

    cuda_external_memory: Option<ExternalMemory>,
}

struct ImportedTextures {
    input_texture: Option<SharedTexture>,
    output_texture: Option<SharedTexture>,
}

impl Drop for SharedTexture {
    fn drop(&mut self) {
        if let Some(ref device) = self.vulkan_device {
            if let Some(image) = self.vulkan_image {
                unsafe {
                    device.destroy_image(image, None);
                }
            }

            if self.vulkan_memory != vk::DeviceMemory::null() {
                unsafe {
                    device.free_memory(self.vulkan_memory, None);
                }
            }
        }
    }
}

impl Drop for ImportedTextures {
    fn drop(&mut self) {}
}

#[derive(Resource, Default)]
struct ExternalTextures {
    input_handle: Option<Handle<Image>>,
    input_manual_view_handle: Option<ManualTextureViewHandle>,
    input_texture: Option<Texture>,
    output_manual_view_handle: Option<ManualTextureViewHandle>,
    output_texture: Option<Texture>,
}

#[derive(Component)]
struct OutputCamera;

#[derive(Component)]
struct InputTexturedCube;

#[derive(Component)]
struct RotatingCube {
    speed: Vec3,
}

impl TopNew for BevyTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: BevyTopParams::default(),
            context: Arc::new(Mutex::new(context)),
            bevy_app: None,
            textures: None,
            cuda_context: None,
            cuda_stream: None,
        }
    }
}

impl OpInfo for BevyTop {
    const OPERATOR_LABEL: &'static str = "Bevy";
    const OPERATOR_TYPE: &'static str = "Bevy";
    const OPERATOR_ICON: &'static str = "BVY";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
}

impl TopInfo for BevyTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cuda;
}

impl Op for BevyTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Top for BevyTop {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame_if_asked: true,
            ..Default::default()
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        match self.execute_inner(&mut output, input) {
            Err(e) => self.set_error(&format!("Bevy TOP execution failed: {}", e)),
            _ => {}
        }
    }
}

impl BevyTop {
    fn can_reuse_texture(
        existing: Option<&SharedTexture>,
        width: u32,
        height: u32,
        format: &PixelFormat,
    ) -> bool {
        existing.map_or(false, |texture| {
            texture.width == width && texture.height == height && texture.pixel_format == *format
        })
    }
    fn execute_inner(
        &mut self,
        output: &mut TopOutput,
        input: &OperatorInputs<TopInput>,
    ) -> Result<()> {
        if self.bevy_app.is_none() {
            self.init_bevy_app()?;
        }

        let mut inputs = vec![];
        for i in 0..input.num_inputs() {
            if let Some(input_desc) = input.input(i).map(|td_input| td_input.texture_desc()) {
                inputs.push(input_desc);
            }
        }

        let input_cuda_info = input
            .input(0)
            .filter(|_| input.num_inputs() > 0)
            .map(|td_input| td_input.get_cuda_array(std::ptr::null_mut()).ok())
            .flatten();

        let params = input.params();
        let first_input_desc = input.input(0).map(|td_input| td_input.texture_desc());
        let (output_width, output_height, output_format) =
            Self::get_resolution_and_format(params, first_input_desc);

        let output_cuda_info = CudaOutputInfo {
            stream: std::ptr::null_mut(),
            texture_desc: TextureDesc {
                width: output_width,
                height: output_height,
                depth: 1,
                tex_dim: TexDim::E2D,
                pixel_format: output_format,
                aspect_x: 0.0,
                aspect_y: 0.0,
            },
            color_buffer_index: 0,
        };

        let output_array_info = output.create_cuda_array(&output_cuda_info)?;
        let began_successfully = {
            let mut ctx = self
                .context
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to lock context: {}", e))?;
            ctx.begin_cuda_operations()
        };

        if !began_successfully {
            return Err(anyhow::anyhow!("Failed to begin CUDA operations"));
        }

        if unsafe { output_array_info.cuda_array().is_null() } {
            return Ok(());
        }

        if self.textures.is_none() {
            self.textures = Some(ImportedTextures {
                input_texture: None,
                output_texture: None,
            });
        }

        self.import_input(input_cuda_info.as_ref())?;
        self.import_output(&output_array_info)?;
        self.create_manual_texture_views()?;
        self.update()?;
        self.export_output(&output_array_info)?;

        self.context.lock().unwrap().end_cuda_operations();

        Ok(())
    }

    fn get_resolution_and_format(
        params: ParamInputs,
        first_input_desc: Option<TextureDesc>,
    ) -> (usize, usize, PixelFormat) {
        let mut output_width = 512;
        let mut output_height = 512;
        let output_resolution = params.get_string("outputresolution");
        let mut output_format = PixelFormat::BGRA8Fixed;

        match output_resolution {
            "useinput" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width;
                    output_height = input_desc.height;
                }
            }
            "eigth" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width / 8;
                    output_height = input_desc.height / 8;
                }
            }
            "quarter" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width / 4;
                    output_height = input_desc.height / 4;
                }
            }
            "half" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width / 2;
                    output_height = input_desc.height / 2;
                }
            }
            "2x" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width * 2;
                    output_height = input_desc.height * 2;
                }
            }
            "4x" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width * 4;
                    output_height = input_desc.height * 4;
                }
            }
            "8x" => {
                if let Some(input_desc) = &first_input_desc {
                    output_width = input_desc.width * 8;
                    output_height = input_desc.height * 8;
                }
            }
            "fit" => {
                if let Some(input_desc) = &first_input_desc {
                    let aspect_ratio = input_desc.width as f32 / input_desc.height as f32;
                    if aspect_ratio > 1.0 {
                        output_width = 512;
                        output_height = (512.0 / aspect_ratio) as usize;
                    } else {
                        output_height = 512;
                        output_width = (512.0 * aspect_ratio) as usize;
                    }
                }
            }
            "limit" => {
                if let Some(input_desc) = &first_input_desc {
                    let max_size = 512;
                    if input_desc.width > max_size || input_desc.height > max_size {
                        let aspect_ratio = input_desc.width as f32 / input_desc.height as f32;
                        if aspect_ratio > 1.0 {
                            output_width = max_size;
                            output_height = (max_size as f32 / aspect_ratio) as usize;
                        } else {
                            output_height = max_size;
                            output_width = (max_size as f32 * aspect_ratio) as usize;
                        }
                    } else {
                        output_width = input_desc.width;
                        output_height = input_desc.height;
                    }
                }
            }
            "custom" => {
                let custom_width = params.get_int("resolution", 0);
                let custom_height = params.get_int("resolution", 1);
                if custom_width > 0 && custom_height > 0 {
                    output_width = custom_width as usize;
                    output_height = custom_height as usize;
                }
            }
            "parpanel" => {}
            _ => {}
        };

        let format = params.get_string("format");
        match format {
            "useinput" => {
                if let Some(input_desc) = &first_input_desc {
                    output_format = input_desc.pixel_format;
                }
            }
            "rgba8fixed" => {
                output_format = PixelFormat::RGBA8Fixed;
            }
            "bgra8fixed" => {
                output_format = PixelFormat::BGRA8Fixed;
            }
            "srgba8fixed" => {
                output_format = PixelFormat::SRGBA8Fixed;
            }
            "sbgra8fixed" => {
                output_format = PixelFormat::SBGRA8Fixed;
            }
            "rgba16fixed" => {
                output_format = PixelFormat::RGBA16Fixed;
            }
            "rgba16float" => {
                output_format = PixelFormat::RGBA16Float;
            }
            "rgba32float" => {
                output_format = PixelFormat::RGBA32Float;
            }
            "mono8fixed" => {
                output_format = PixelFormat::Mono8Fixed;
            }
            "mono16fixed" => {
                output_format = PixelFormat::Mono16Fixed;
            }
            "mono16float" => {
                output_format = PixelFormat::Mono16Float;
            }
            "mono32float" => {
                output_format = PixelFormat::Mono32Float;
            }
            "rg8fixed" => {
                output_format = PixelFormat::RG8Fixed;
            }
            "rg16fixed" => {
                output_format = PixelFormat::RG16Fixed;
            }
            "rg16float" => {
                output_format = PixelFormat::RG16Float;
            }
            "rg32float" => {
                output_format = PixelFormat::RG32Float;
            }
            "a8fixed" => {
                output_format = PixelFormat::A8Fixed;
            }
            "a16fixed" => {
                output_format = PixelFormat::A16Fixed;
            }
            "a16float" => {
                output_format = PixelFormat::A16Float;
            }
            "a32float" => {
                output_format = PixelFormat::A32Float;
            }
            "monoalpha8fixed" => {
                output_format = PixelFormat::MonoA8Fixed;
            }
            "monoalpha16fixed" => {
                output_format = PixelFormat::MonoA16Fixed;
            }
            "monoalpha16float" => {
                output_format = PixelFormat::MonoA16Float;
            }
            "monoalpha32float" => {
                output_format = PixelFormat::MonoA32Float;
            }
            "rgb10a2fixed" => {
                output_format = PixelFormat::RGB10A2Fixed;
            }
            "rgba11float" => {
                output_format = PixelFormat::RGB11Float;
            }
            _ => {}
        }

        (output_width, output_height, output_format)
    }

    fn import_input(
        &mut self,
        cuda_array_info: Option<&td_rs_top::cuda::CudaArrayInfo>,
    ) -> Result<()> {
        let cuda_array_info = match cuda_array_info {
            Some(info) => info,
            None => return Ok(()),
        };

        let ctx = self.get_or_create_cuda_context()?;

        if let Some(ref app) = self.bevy_app {
            let render_device = app
                .world()
                .get_resource::<RenderDevice>()
                .ok_or_else(|| anyhow::anyhow!("RenderDevice not found"))?;

            let texture_desc = cuda_array_info.texture_desc();
            let width = texture_desc.width as u32;
            let height = texture_desc.height as u32;

            let need_new_texture = self
                .textures
                .as_ref()
                .map(|textures| {
                    !Self::can_reuse_texture(
                        textures.input_texture.as_ref(),
                        width,
                        height,
                        &texture_desc.pixel_format,
                    )
                })
                .unwrap_or(true);

            if need_new_texture {
                let input_texture_result = unsafe {
                    render_device
                        .wgpu_device()
                        .as_hal::<wgpu::hal::api::Vulkan, _, _>(|device| {
                            let Some(device) = device else {
                                return None;
                            };

                            let instance = device.shared_instance().raw_instance();
                            let physical_device = device.raw_physical_device();
                            let vulkan_device = device.raw_device();

                            match self.create_input_external_memory(
                                &ctx,
                                instance,
                                vulkan_device,
                                physical_device,
                                &cuda_array_info,
                                width,
                                height,
                            ) {
                                Ok(texture) => Some(texture),
                                Err(_) => None,
                            }
                        })
                };

                if let (Some(input_texture), Some(ref mut textures)) =
                    (input_texture_result, &mut self.textures)
                {
                    textures.input_texture = Some(input_texture);
                }
            }

            self.update_input_texture_data(&ctx, cuda_array_info)?;

            if let Ok(stream) = self.get_or_create_cuda_stream() {
                unsafe {
                    use cudarc::runtime::sys::*;
                    let sync_result = cudaStreamSynchronize(stream);
                    if sync_result == cudaError::cudaSuccess {}
                }
            }
        }

        Ok(())
    }

    fn import_output(&mut self, output_array_info: &CudaArrayInfo) -> Result<()> {
        let ctx = self.get_or_create_cuda_context()?;

        let output_desc = output_array_info.texture_desc();
        let need_new_output = self
            .textures
            .as_ref()
            .map(|textures| {
                !Self::can_reuse_texture(
                    textures.output_texture.as_ref(),
                    output_desc.width as u32,
                    output_desc.height as u32,
                    &output_desc.pixel_format,
                )
            })
            .unwrap_or(true);

        if need_new_output {
            let output_texture_result = if let Some(ref app) = self.bevy_app {
                let render_device = app
                    .world()
                    .get_resource::<RenderDevice>()
                    .ok_or_else(|| anyhow::anyhow!("RenderDevice not found"))?;

                unsafe {
                    render_device
                        .wgpu_device()
                        .as_hal::<wgpu::hal::api::Vulkan, _, _>(|device| {
                            let Some(device) = device else {
                                return None;
                            };

                            let instance = device.shared_instance().raw_instance();
                            let physical_device = device.raw_physical_device();
                            let vulkan_device = device.raw_device();

                            match self.create_output_external_memory(
                                &ctx,
                                instance,
                                vulkan_device,
                                physical_device,
                                output_array_info,
                            ) {
                                Ok(texture) => Some(texture),
                                Err(_) => None,
                            }
                        })
                }
            } else {
                None
            };

            if let (Some(output_texture), Some(ref mut textures)) =
                (output_texture_result, &mut self.textures)
            {
                textures.output_texture = Some(output_texture);
            }
        }

        Ok(())
    }

    fn create_output_external_memory(
        &self,
        ctx: &CudaContext,
        instance: &Instance,
        device: &Device,
        physical_device: vk::PhysicalDevice,
        output_array_info: &td_rs_top::cuda::CudaArrayInfo,
    ) -> Result<SharedTexture> {
        let output_desc = output_array_info.texture_desc();
        let width = output_desc.width as u32;
        let height = output_desc.height as u32;

        let vulkan_format = pixel_format_to_vulkan_format(&output_desc.pixel_format);

        let mut ext_mem_image_info = vk::ExternalMemoryImageCreateInfoKHR::default();
        ext_mem_image_info.handle_types = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let mut image_create_info = vk::ImageCreateInfo::default();
        image_create_info.p_next = &ext_mem_image_info as *const _ as *const std::ffi::c_void;
        image_create_info.image_type = vk::ImageType::TYPE_2D;
        image_create_info.format = vulkan_format;
        image_create_info.extent = vk::Extent3D {
            width,
            height,
            depth: 1,
        };
        image_create_info.mip_levels = 1;
        image_create_info.array_layers = 1;
        image_create_info.samples = vk::SampleCountFlags::TYPE_1;
        image_create_info.tiling = vk::ImageTiling::LINEAR;
        image_create_info.usage = vk::ImageUsageFlags::COLOR_ATTACHMENT
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::SAMPLED;
        image_create_info.sharing_mode = vk::SharingMode::EXCLUSIVE;
        image_create_info.initial_layout = vk::ImageLayout::UNDEFINED;

        let output_image = unsafe {
            device
                .create_image(&image_create_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to create output image: {:?}", e))?
        };

        let image_mem_reqs = unsafe { device.get_image_memory_requirements(output_image) };

        let memory_type_index = find_memory_type_for_external(
            instance,
            physical_device,
            image_mem_reqs.memory_type_bits,
        )?;

        let mut export_mem_info = vk::ExportMemoryAllocateInfo::default();
        export_mem_info.handle_types = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let mut alloc_info = vk::MemoryAllocateInfo::default();
        alloc_info.p_next = &export_mem_info as *const _ as *const std::ffi::c_void;
        alloc_info.allocation_size = image_mem_reqs.size;
        alloc_info.memory_type_index = memory_type_index;

        let output_memory = unsafe {
            device
                .allocate_memory(&alloc_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to allocate output memory: {:?}", e))?
        };

        unsafe {
            device
                .bind_image_memory(output_image, output_memory, 0)
                .map_err(|e| anyhow::anyhow!("Failed to bind output image memory: {:?}", e))?;
        }

        let ext_mem_win32 = khr::external_memory_win32::Device::new(instance, device);
        let mut handle_info = vk::MemoryGetWin32HandleInfoKHR::default();
        handle_info.memory = output_memory;
        handle_info.handle_type = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let handle = unsafe {
            ext_mem_win32
                .get_memory_win32_handle(&handle_info)
                .map_err(|e| anyhow::anyhow!("Failed to get Win32 handle for output: {:?}", e))?
        };

        let file = unsafe { File::from_raw_handle(handle as RawHandle) };
        let output_cuda_ext_memory =
            unsafe { import_external_memory_dedicated(&ctx, file, image_mem_reqs.size) }
                .map_err(|e| anyhow::anyhow!("Failed to import output external memory: {:?}", e))?;

        let image_subresource = vk::ImageSubresource {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            array_layer: 0,
        };
        let actual_layout =
            unsafe { device.get_image_subresource_layout(output_image, image_subresource) };
        let actual_row_pitch = actual_layout.row_pitch as usize;

        let output_texture = SharedTexture {
            vulkan_memory: output_memory,
            vulkan_image: Some(output_image),
            vulkan_device: Some(device.clone()),
            width,
            height,
            pixel_format: output_desc.pixel_format.clone(),
            actual_row_pitch: Some(actual_row_pitch),
            cuda_external_memory: Some(output_cuda_ext_memory),
        };

        Ok(output_texture)
    }

    fn create_input_external_memory(
        &self,
        ctx: &CudaContext,
        instance: &Instance,
        device: &Device,
        physical_device: vk::PhysicalDevice,
        cuda_array_info: &td_rs_top::cuda::CudaArrayInfo,
        width: u32,
        height: u32,
    ) -> Result<SharedTexture> {
        let mut ext_mem_image_info = vk::ExternalMemoryImageCreateInfoKHR::default();
        ext_mem_image_info.handle_types = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let mut image_create_info = vk::ImageCreateInfo::default();
        image_create_info.p_next = &ext_mem_image_info as *const _ as *const std::ffi::c_void;
        image_create_info.image_type = vk::ImageType::TYPE_2D;
        image_create_info.format =
            pixel_format_to_vulkan_format(&cuda_array_info.texture_desc().pixel_format);
        image_create_info.extent = vk::Extent3D {
            width,
            height,
            depth: 1,
        };
        image_create_info.mip_levels = 1;
        image_create_info.array_layers = 1;
        image_create_info.samples = vk::SampleCountFlags::TYPE_1;
        image_create_info.tiling = vk::ImageTiling::LINEAR;
        image_create_info.usage = vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::SAMPLED;
        image_create_info.sharing_mode = vk::SharingMode::EXCLUSIVE;
        image_create_info.initial_layout = vk::ImageLayout::UNDEFINED;

        let input_image = unsafe {
            device
                .create_image(&image_create_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to create input image: {:?}", e))?
        };

        let image_mem_reqs = unsafe { device.get_image_memory_requirements(input_image) };

        let memory_type_index = find_memory_type_for_external(
            instance,
            physical_device,
            image_mem_reqs.memory_type_bits,
        )?;

        let mut export_mem_info = vk::ExportMemoryAllocateInfo::default();
        export_mem_info.handle_types = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let mut alloc_info = vk::MemoryAllocateInfo::default();
        alloc_info.p_next = &export_mem_info as *const _ as *const std::ffi::c_void;
        alloc_info.allocation_size = image_mem_reqs.size;
        alloc_info.memory_type_index = memory_type_index;

        let input_memory = unsafe {
            device
                .allocate_memory(&alloc_info, None)
                .map_err(|e| anyhow::anyhow!("Failed to allocate input memory: {:?}", e))?
        };

        unsafe {
            device
                .bind_image_memory(input_image, input_memory, 0)
                .map_err(|e| anyhow::anyhow!("Failed to bind input image memory: {:?}", e))?;
        }

        let ext_mem_win32 = khr::external_memory_win32::Device::new(instance, device);
        let mut handle_info = vk::MemoryGetWin32HandleInfoKHR::default();
        handle_info.memory = input_memory;
        handle_info.handle_type = vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

        let handle = unsafe {
            ext_mem_win32
                .get_memory_win32_handle(&handle_info)
                .map_err(|e| anyhow::anyhow!("Failed to get Win32 handle for input: {:?}", e))?
        };

        let file =
            unsafe { std::fs::File::from_raw_handle(handle as std::os::windows::io::RawHandle) };
        let cuda_ext_memory =
            unsafe { import_external_memory_dedicated(ctx, file, image_mem_reqs.size) }
                .map_err(|e| anyhow::anyhow!("Failed to import external memory: {:?}", e))?;

        let input_subresource = vk::ImageSubresource {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            array_layer: 0,
        };
        let input_layout =
            unsafe { device.get_image_subresource_layout(input_image, input_subresource) };
        let input_row_pitch = input_layout.row_pitch as usize;

        let input_texture = SharedTexture {
            vulkan_memory: input_memory,
            vulkan_image: Some(input_image),
            vulkan_device: Some(device.clone()),
            width,
            height,
            pixel_format: cuda_array_info.texture_desc().pixel_format.clone(),
            actual_row_pitch: Some(input_row_pitch),
            cuda_external_memory: Some(cuda_ext_memory),
        };

        Ok(input_texture)
    }

    fn update_input_texture_data(
        &mut self,
        _ctx: &CudaContext,
        cuda_array_info: &td_rs_top::cuda::CudaArrayInfo,
    ) -> Result<()> {
        let textures = self
            .textures
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No textures available"))?;

        let input_texture = textures
            .input_texture
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No input texture available"))?;

        let cuda_external_memory = input_texture
            .cuda_external_memory
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Input texture has no external memory!"))?;

        let device_ptr = cuda_external_memory.map_all_ref().map_err(|e| {
            anyhow::anyhow!("Failed to map input external memory for update: {:?}", e)
        })?;

        let (width, height, actual_row_pitch) = (
            input_texture.width,
            input_texture.height,
            input_texture.actual_row_pitch,
        );

        let td_cuda_array = unsafe { cuda_array_info.cuda_array() };
        let input_desc = cuda_array_info.texture_desc();

        let stream = self.get_or_create_cuda_stream()?;

        Self::copy_from_array(
            td_cuda_array as *mut cudarc::runtime::sys::cudaArray,
            device_ptr,
            width,
            height,
            &input_desc.pixel_format,
            actual_row_pitch,
            stream,
        )?;

        Ok(())
    }

    fn create_manual_texture_views(&mut self) -> Result<()> {
        if let Some(ref mut app) = self.bevy_app {
            let srgb_view_formats = vec![TextureFormat::Bgra8UnormSrgb];
            let rgba_srgb_view_formats = vec![TextureFormat::Rgba8UnormSrgb];
            let empty_view_formats: Vec<TextureFormat> = vec![];

            let (input, output) = {
                let mut input: Option<(wgpu::hal::vulkan::Texture, wgpu::TextureDescriptor)> = None;
                let mut output: Option<(wgpu::hal::vulkan::Texture, wgpu::TextureDescriptor)> =
                    None;

                if let Some(ref textures) = self.textures {
                    if let Some(ref input_texture) = textures.input_texture {
                        if let Some(vulkan_image) = input_texture.vulkan_image {
                            let input_format = input_texture.pixel_format.clone();
                            let wgpu_format = pixel_format_to_wgpu_format(&input_format);

                            match Self::image_as_hal(
                                vulkan_image,
                                input_texture.width,
                                input_texture.height,
                                wgpu_format,
                            ) {
                                Ok(texture) => {
                                    input = Some((
                                        texture,
                                        TextureDescriptor {
                                            label: Some("input_texture"),
                                            size: Extent3d {
                                                width: input_texture.width,
                                                height: input_texture.height,
                                                depth_or_array_layers: 1,
                                            },
                                            mip_level_count: 1,
                                            sample_count: 1,
                                            dimension: TextureDimension::D2,
                                            format: wgpu_format,
                                            usage: TextureUsages::COPY_SRC
                                                | TextureUsages::TEXTURE_BINDING,
                                            view_formats: &[],
                                        },
                                    ));
                                }
                                Err(_) => {}
                            }
                        } else {
                        }
                    } else {
                    }

                    if let Some(ref output_texture) = textures.output_texture {
                        if let Some(vulkan_image) = output_texture.vulkan_image {
                            let output_format = output_texture.pixel_format.clone();
                            let wgpu_format = pixel_format_to_wgpu_format(&output_format);

                            let view_formats_slice = match wgpu_format {
                                TextureFormat::Bgra8Unorm => &srgb_view_formats,
                                TextureFormat::Rgba8Unorm => &rgba_srgb_view_formats,
                                _ => &empty_view_formats,
                            };

                            match Self::image_as_hal(
                                vulkan_image,
                                output_texture.width,
                                output_texture.height,
                                wgpu_format,
                            ) {
                                Ok(texture) => {
                                    output = Some((
                                        texture,
                                        TextureDescriptor {
                                            label: Some("output_texture"),
                                            size: Extent3d {
                                                width: output_texture.width,
                                                height: output_texture.height,
                                                depth_or_array_layers: 1,
                                            },
                                            mip_level_count: 1,
                                            sample_count: 1,
                                            dimension: TextureDimension::D2,
                                            format: wgpu_format,
                                            usage: TextureUsages::COPY_DST
                                                | TextureUsages::RENDER_ATTACHMENT,
                                            view_formats: view_formats_slice,
                                        },
                                    ));
                                }
                                Err(_e) => {}
                            }
                        }
                    } else {
                    }
                } else {
                }

                (input, output)
            };

            unsafe {
                let render_device = app.world().resource::<RenderDevice>();

                let mut input_data: Option<(
                    wgpu::Texture,
                    wgpu::TextureView,
                    UVec2,
                    TextureFormat,
                )> = None;
                let mut output_data: Option<(
                    wgpu::Texture,
                    wgpu::TextureView,
                    UVec2,
                    TextureFormat,
                )> = None;

                if let Some((input_texture, input_descriptor)) = input {
                    let input_size = input_descriptor.size.clone();
                    let input_format = input_descriptor.format;
                    let input_texture = render_device
                        .wgpu_device()
                        .create_texture_from_hal::<wgpu::hal::api::Vulkan>(
                            input_texture,
                            &input_descriptor,
                        );
                    let input_texture_view =
                        input_texture.create_view(&wgpu::TextureViewDescriptor::default());

                    input_data = Some((
                        input_texture,
                        input_texture_view,
                        UVec2::new(input_size.width, input_size.height),
                        input_format,
                    ));
                }

                if let Some((output_texture, output_descriptor)) = output {
                    let output_size = output_descriptor.size.clone();
                    let output_format = output_descriptor.format;
                    let output_texture = render_device
                        .wgpu_device()
                        .create_texture_from_hal::<wgpu::hal::api::Vulkan>(
                            output_texture,
                            &output_descriptor,
                        );

                    let output_view_format = match output_format {
                        TextureFormat::Bgra8Unorm => TextureFormat::Bgra8UnormSrgb,
                        TextureFormat::Rgba8Unorm => TextureFormat::Rgba8UnormSrgb,
                        _ => output_format,
                    };
                    let output_texture_view =
                        output_texture.create_view(&wgpu::TextureViewDescriptor {
                            label: Some("output_texture_view"),
                            format: Some(output_view_format),
                            ..Default::default()
                        });

                    output_data = Some((
                        output_texture,
                        output_texture_view,
                        UVec2::new(output_size.width, output_size.height),
                        output_view_format,
                    ));
                }

                if let Some((input_texture, input_texture_view, input_size, input_format)) =
                    input_data
                {
                    {
                        let mut manual_texture_views =
                            app.world_mut().resource_mut::<ManualTextureViews>();
                        manual_texture_views.insert(
                            ManualTextureViewHandle(1),
                            ManualTextureView {
                                texture_view: input_texture_view.into(),
                                size: input_size,
                                format: input_format,
                            },
                        );
                    }

                    {
                        let mut external_textures =
                            app.world_mut().resource_mut::<ExternalTextures>();
                        external_textures.input_texture = Some(input_texture.into());
                        external_textures.input_manual_view_handle =
                            Some(ManualTextureViewHandle(1));
                    }
                }

                if let Some((output_texture, output_texture_view, output_size, output_format)) =
                    output_data
                {
                    {
                        let mut manual_texture_views =
                            app.world_mut().resource_mut::<ManualTextureViews>();
                        manual_texture_views.insert(
                            ManualTextureViewHandle(2),
                            ManualTextureView {
                                texture_view: output_texture_view.into(),
                                size: output_size,
                                format: output_format,
                            },
                        );
                    }

                    {
                        let mut external_textures =
                            app.world_mut().resource_mut::<ExternalTextures>();
                        external_textures.output_texture = Some(output_texture.into());
                        external_textures.output_manual_view_handle =
                            Some(ManualTextureViewHandle(2));
                    }
                }
            }
        }

        Ok(())
    }

    fn image_as_hal(
        vulkan_image: vk::Image,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Result<wgpu::hal::vulkan::Texture> {
        use wgpu::hal::TextureUses;

        let hal_desc = wgpu::hal::TextureDescriptor {
            label: Some("external_texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: TextureUses::COLOR_TARGET | TextureUses::RESOURCE,
            memory_flags: wgpu::hal::MemoryFlags::empty(),
            view_formats: vec![],
        };

        let hal_texture = unsafe {
            wgpu::hal::vulkan::Device::texture_from_raw(
                vulkan_image,
                &hal_desc,
                Some(Box::new(|| {})),
            )
        };

        Ok(hal_texture)
    }

    fn export_output(&mut self, output_array_info: &CudaArrayInfo) -> Result<()> {
        let td_output_array = unsafe { output_array_info.cuda_array() };

        if td_output_array.is_null() {
            return Ok(());
        }

        let export_info = self
            .textures
            .as_ref()
            .and_then(|textures| textures.output_texture.as_ref())
            .and_then(|output_texture| {
                output_texture
                    .cuda_external_memory
                    .as_ref()
                    .map(|cuda_external_memory| {
                        (
                            cuda_external_memory,
                            output_texture.width,
                            output_texture.height,
                        )
                    })
            });

        if let Some((cuda_external_memory, width, height)) = export_info {
            let device_ptr = cuda_external_memory
                .map_all_ref()
                .map_err(|e| anyhow::anyhow!("Failed to map output external memory: {:?}", e))?;

            let output_desc = output_array_info.texture_desc();

            let output_row_pitch = self
                .textures
                .as_ref()
                .and_then(|textures| textures.output_texture.as_ref())
                .and_then(|output_texture| output_texture.actual_row_pitch);

            let stream = self
                .get_or_create_cuda_stream()
                .unwrap_or(std::ptr::null_mut());

            Self::copy_buffer_to_td_texture_static_ptr_async(
                device_ptr,
                td_output_array as *mut cudarc::runtime::sys::cudaArray,
                width,
                height,
                &output_desc.pixel_format,
                output_row_pitch,
                stream,
            )?;

            if !stream.is_null() {
                unsafe {
                    use cudarc::runtime::sys::*;
                    let sync_result = cudaStreamSynchronize(stream);
                    if sync_result == cudaError::cudaSuccess {}
                }
            }
        }

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        if let Some(ref mut app) = self.bevy_app {
            app.update();

            let render_device = app.world().resource::<RenderDevice>();
            render_device.wgpu_device().poll(wgpu::Maintain::Wait);
        }

        Ok(())
    }

    fn copy_from_array(
        td_cuda_array: *mut cudarc::runtime::sys::cudaArray,
        device_ptr: sys::CUdeviceptr,
        width: u32,
        height: u32,
        input_format: &PixelFormat,
        vulkan_row_pitch: Option<usize>,
        stream: cudarc::runtime::sys::cudaStream_t,
    ) -> Result<()> {
        unsafe {
            use cudarc::runtime::sys::*;

            if td_cuda_array.is_null() {
                return Ok(());
            }

            let bytes_per_pixel = get_bytes_per_pixel(input_format);
            if bytes_per_pixel == 0 {
                return Err(anyhow::anyhow!("Invalid input format: {:?}", input_format));
            }

            let calculated_pitch = width * bytes_per_pixel as u32;
            let dst_pitch = vulkan_row_pitch.unwrap_or(calculated_pitch as usize);

            let width_in_bytes = calculated_pitch;

            let copy_result = cudaMemcpy2DFromArrayAsync(
                device_ptr as *mut std::ffi::c_void,
                dst_pitch,
                td_cuda_array as cudaArray_const_t,
                0,
                0,
                width_in_bytes as usize,
                height as usize,
                cudaMemcpyKind::cudaMemcpyDeviceToDevice,
                stream,
            );

            if copy_result != cudaError::cudaSuccess {
                return Err(anyhow::anyhow!(
                    "CUDA async input copy failed: {:?}",
                    copy_result
                ));
            }
        }

        Ok(())
    }

    fn copy_buffer_to_td_texture_static_ptr_async(
        device_ptr: sys::CUdeviceptr,
        td_cuda_array: *mut cudarc::runtime::sys::cudaArray,
        width: u32,
        height: u32,
        output_format: &PixelFormat,
        vulkan_row_pitch: Option<usize>,
        stream: cudarc::runtime::sys::cudaStream_t,
    ) -> Result<()> {
        unsafe {
            use cudarc::runtime::sys::*;

            if td_cuda_array.is_null() {
                return Ok(());
            }

            let bytes_per_pixel = get_bytes_per_pixel(output_format);
            if bytes_per_pixel == 0 {
                return Err(anyhow::anyhow!(
                    "Invalid output format: {:?}",
                    output_format
                ));
            }

            let calculated_pitch = width * bytes_per_pixel as u32;
            let vulkan_pitch = vulkan_row_pitch.unwrap_or(calculated_pitch as usize);

            let cuda_aligned_pitch = if vulkan_pitch % 512 != 0 {
                ((vulkan_pitch + 511) / 512) * 512
            } else {
                vulkan_pitch
            };

            let src_pitch = cuda_aligned_pitch;

            let width_in_bytes = calculated_pitch;

            let copy_result = cudaMemcpy2DToArrayAsync(
                td_cuda_array as cudaArray_t,
                0,
                0,
                device_ptr as *const std::ffi::c_void,
                src_pitch,
                width_in_bytes as usize,
                height as usize,
                cudaMemcpyKind::cudaMemcpyDeviceToDevice,
                stream,
            );

            if copy_result != cudaError::cudaSuccess {
                return Err(anyhow::anyhow!(
                    "CUDA async output copy failed: {:?}",
                    copy_result
                ));
            }
        }

        Ok(())
    }

    fn init_bevy_app(&mut self) -> Result<()> {
        let mut app = App::new();

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                })
                .disable::<PipelinedRenderingPlugin>(),
        );

        app.init_resource::<ExternalTextures>();
        app.add_systems(Startup, setup_scene);
        app.add_systems(
            Update,
            (update_camera_target, rotate_cubes, update_input_texture),
        );

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(ExtractSchedule, extract_external_textures);

        app.finish();
        app.cleanup();

        self.bevy_app = Some(app);
        Ok(())
    }

    fn get_or_create_cuda_context(&mut self) -> Result<Arc<CudaContext>> {
        if self.cuda_context.is_none() {
            let context = CudaContext::new(0)
                .map_err(|e| anyhow::anyhow!("Failed to create CUDA context: {:?}", e))?;
            self.cuda_context = Some(context);
        }
        self.cuda_context
            .clone()
            .ok_or_else(|| anyhow::anyhow!("CUDA context is not initialized, but it should be"))
    }

    fn get_or_create_cuda_stream(&mut self) -> Result<cudarc::runtime::sys::cudaStream_t> {
        if self.cuda_stream.is_none() {
            unsafe {
                use cudarc::runtime::sys::*;
                let mut stream = std::ptr::null_mut();
                let result = cudaStreamCreate(&mut stream);
                if result == cudaError::cudaSuccess {
                    self.cuda_stream = Some(stream);
                } else {
                    return Err(anyhow::anyhow!(
                        "Failed to create CUDA stream: {:?}",
                        result
                    ));
                }
            }
        }
        Ok(self.cuda_stream.unwrap())
    }
}

fn find_memory_type_for_external(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
) -> Result<u32> {
    let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };

    for i in 0..mem_properties.memory_type_count {
        let memory_type = mem_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0
            && (memory_type.property_flags
                & (vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL))
                == (vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL)
        {
            return Ok(i);
        }
    }

    for i in 0..mem_properties.memory_type_count {
        let memory_type = mem_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0
            && (memory_type.property_flags
                & (vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE))
                == (vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Ok(i);
        }
    }

    for i in 0..mem_properties.memory_type_count {
        let memory_type = mem_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0
            && (memory_type.property_flags & vk::MemoryPropertyFlags::HOST_VISIBLE)
                == vk::MemoryPropertyFlags::HOST_VISIBLE
        {
            return Ok(i);
        }
    }

    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0 {
            return Ok(i);
        }
    }

    Err(anyhow::anyhow!(
        "Failed to find suitable memory type for external memory"
    ))
}

unsafe fn import_external_memory_dedicated(
    cuda_context: &CudaContext,
    file: File,
    size: u64,
) -> Result<ExternalMemory, anyhow::Error> {
    cuda_context
        .bind_to_thread()
        .map_err(|e| anyhow::anyhow!("Failed to bind CUDA context to thread: {:?}", e))?;

    #[cfg(windows)]
    let external_memory = unsafe {
        let raw_handle = file.as_raw_handle();

        let mut external_memory = std::mem::MaybeUninit::uninit();
        let handle_description = sys::CUDA_EXTERNAL_MEMORY_HANDLE_DESC {
            type_: sys::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32,
            handle: sys::CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1 {
                win32: sys::CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
                    handle: raw_handle,
                    name: std::ptr::null(),
                },
            },
            size,
            flags: 1,
            reserved: [0; 16],
        };

        let result = cudarc::driver::sys::cuImportExternalMemory(
            external_memory.as_mut_ptr(),
            &handle_description,
        );
        if result != cudarc::driver::sys::CUresult::CUDA_SUCCESS {
            return Err(anyhow::anyhow!(
                "CUDA external memory import failed: {:?}",
                result
            ));
        }

        external_memory.assume_init()
    };

    Ok(ExternalMemory {
        external_memory,
        size,
        _file: ManuallyDrop::new(file),
    })
}

#[derive(Debug)]
pub struct ExternalMemory {
    external_memory: sys::CUexternalMemory,
    size: u64,
    _file: ManuallyDrop<File>,
}

impl ExternalMemory {
    pub fn map_all(self) -> Result<MappedBuffer, anyhow::Error> {
        let size = self.size as usize;
        self.map_range(0..size)
    }

    pub fn map_all_ref(&self) -> Result<sys::CUdeviceptr, anyhow::Error> {
        let size = self.size as usize;
        self.map_range_ref(0..size)
    }

    pub fn map_range_ref(
        &self,
        range: std::ops::Range<usize>,
    ) -> Result<sys::CUdeviceptr, anyhow::Error> {
        assert!(range.start as u64 <= self.size);
        assert!(range.end as u64 <= self.size);

        let device_ptr = unsafe {
            let buffer_desc = sys::CUDA_EXTERNAL_MEMORY_BUFFER_DESC {
                offset: range.start as u64,
                size: range.len() as u64,
                flags: 0,
                reserved: [0; 16],
            };

            let mut device_ptr = std::mem::MaybeUninit::uninit();
            let result = cudarc::driver::sys::cuExternalMemoryGetMappedBuffer(
                device_ptr.as_mut_ptr(),
                self.external_memory,
                &buffer_desc,
            );

            if result != cudarc::driver::sys::CUresult::CUDA_SUCCESS {
                return Err(anyhow::anyhow!(
                    "CUDA external memory mapping failed: {:?}",
                    result
                ));
            }

            device_ptr.assume_init()
        };

        Ok(device_ptr)
    }

    pub fn map_range(self, range: std::ops::Range<usize>) -> Result<MappedBuffer, anyhow::Error> {
        assert!(range.start as u64 <= self.size);
        assert!(range.end as u64 <= self.size);

        let device_ptr = unsafe {
            let buffer_desc = sys::CUDA_EXTERNAL_MEMORY_BUFFER_DESC {
                offset: range.start as u64,
                size: range.len() as u64,
                flags: 0,
                reserved: [0; 16],
            };

            let mut device_ptr = std::mem::MaybeUninit::uninit();
            let result = cudarc::driver::sys::cuExternalMemoryGetMappedBuffer(
                device_ptr.as_mut_ptr(),
                self.external_memory,
                &buffer_desc,
            );

            if result != cudarc::driver::sys::CUresult::CUDA_SUCCESS {
                return Err(anyhow::anyhow!(
                    "CUDA external memory mapping failed: {:?}",
                    result
                ));
            }

            device_ptr.assume_init()
        };

        Ok(MappedBuffer {
            device_ptr,
            len: range.len(),
            external_memory: self,
        })
    }
}

impl Drop for ExternalMemory {
    fn drop(&mut self) {
        unsafe {
            cudarc::driver::sys::cuDestroyExternalMemory(self.external_memory);
            ManuallyDrop::<File>::drop(&mut self._file);
        }
    }
}

#[derive(Debug)]
pub struct MappedBuffer {
    pub device_ptr: sys::CUdeviceptr,
    pub len: usize,
    external_memory: ExternalMemory,
}

impl Drop for MappedBuffer {
    fn drop(&mut self) {}
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut external_textures: ResMut<ExternalTextures>,
) {
    let input_handle: Handle<Image> = Handle::default();
    external_textures.input_handle = Some(input_handle.clone());

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.0, 3.0, 3.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(input_handle.clone()),
            metallic: 0.1,
            perceptual_roughness: 0.3,
            reflectance: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        InputTexturedCube,
        RotatingCube {
            speed: Vec3::new(0.5, 0.8, 0.3),
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.3),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            reflectance: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -2.5, 0.0),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 25000.0,
            color: Color::srgb(1.0, 0.98, 0.9),
            ..default()
        },
        Transform {
            translation: Vec3::new(4.0, 6.0, 4.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 8000.0,
            color: Color::srgb(0.8, 0.9, 1.0),
            ..default()
        },
        Transform {
            translation: Vec3::new(-3.0, 2.0, -2.0),
            rotation: Quat::from_rotation_y(PI / 6.),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.4, 0.6),
        brightness: 800.0,
        affects_lightmapped_meshes: false,
    });

    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.15)),
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 6.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
            .with_scale(Vec3::new(1.0, -1.0, 1.0)),
        OutputCamera,
    ));
}

fn update_camera_target(
    mut camera_query: Query<&mut Camera, With<OutputCamera>>,
    external_textures: Res<ExternalTextures>,
) {
    for mut camera in camera_query.iter_mut() {
        if let Some(output_handle) = external_textures.output_manual_view_handle {
            camera.target = RenderTarget::TextureView(output_handle);
        }
    }
}

fn rotate_cubes(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingCube)>) {
    for (mut transform, rotating) in query.iter_mut() {
        let rotation_delta = rotating.speed * time.delta_secs();
        transform.rotate_x(rotation_delta.x);
        transform.rotate_y(rotation_delta.y);
        transform.rotate_z(rotation_delta.z);
    }
}

fn update_input_texture(
    external_textures: Res<ExternalTextures>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cube_query: Query<&MeshMaterial3d<StandardMaterial>, With<InputTexturedCube>>,
) {
    if let Some(input_handle) = &external_textures.input_handle {
        for material_handle in cube_query.iter_mut() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.base_color_texture = Some(input_handle.clone());
            }
        }
    }
}

fn extract_external_textures(
    external_textures: Extract<Res<ExternalTextures>>,
    manual_texture_views: Extract<Res<ManualTextureViews>>,
    default_sampler: Res<DefaultImageSampler>,
    mut gpu_images: ResMut<RenderAssets<GpuImage>>,
) {
    match (
        external_textures.input_texture.as_ref(),
        external_textures.input_handle.as_ref(),
        external_textures.input_manual_view_handle.as_ref(),
    ) {
        (Some(input_texture), Some(input_handle), Some(input_manual_view_handle)) => {
            let texture_view = manual_texture_views
                .get(input_manual_view_handle)
                .expect("Input ManualTextureViewHandle not found");
            let input_gpu_image = GpuImage {
                texture: input_texture.clone(),
                texture_view: texture_view.texture_view.clone(),
                texture_format: texture_view.format,
                sampler: (**default_sampler).clone(),
                size: Extent3d {
                    width: texture_view.size.x,
                    height: texture_view.size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 0,
            };
            gpu_images.insert(input_handle.id(), input_gpu_image);
        }
        _ => {}
    }
}

top_plugin!(BevyTop);
