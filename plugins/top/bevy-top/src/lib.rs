use anyhow::Result;
use ash::{khr, vk, Device, Instance};
use bevy::ecs::entity::EntityHashMap;
use bevy::reflect::List;
use bevy::render::camera::{
    ManualTextureView, ManualTextureViewHandle, ManualTextureViews, RenderTarget,
};
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::render_resource::{Texture, TextureDescriptor, TextureFormat, TextureView};
use bevy::render::texture::DefaultImageSampler;
use bevy::render::Extract;
use bevy::{
    core_pipeline::post_process::ChromaticAberration,
    prelude::*,
    render::{
        render_asset::RenderAssets, renderer::RenderDevice, texture::GpuImage, ExtractSchedule,
        RenderApp,
    },
    window::WindowPlugin,
};
use cudarc::driver::{sys, CudaContext};
use cudarc::runtime::sys::CUstream_st;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};
use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;
use wgpu::{Extent3d, TextureDimension, TextureUsages};

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
    #[param(
        label = "Intensity",
        page = "Chromatic Aberration ",
        default = 0.01,
        min = 0.0,
        max = 0.4
    )]
    chromatic_aberration_intensity: f64,
    #[param(label = "Color LUT", page = "Chromatic Aberration ")]
    chromatic_aberration_color_lut: TopParam,
    #[param(
        label = "Intensity",
        page = "Chromatic Aberration ",
        default = 8.0,
        min = 1.0,
        max = 16.0
    )]
    chromatic_aberration_max_samples: u32,
}

pub struct BevyTop {
    params: BevyTopParams,
    context: Arc<Mutex<TopContext>>,
    app: Option<App>,
    inputs_entities: HashMap<usize, Entity>,
    output_entities: HashMap<usize, Entity>,
}

#[derive(Component)]
struct SharedTexture {
    vulkan_memory: vk::DeviceMemory,
    vulkan_image: vk::Image,
    vulkan_device: Device,
    width: u32,
    height: u32,
    texture_desc: TextureDesc,
    pixel_format: PixelFormat,
    row_pitch: usize,
}

#[derive(Default, Deref, DerefMut)]
pub struct SharedTextureExternalMemory(EntityHashMap<ExternalMemory>);

impl Drop for SharedTexture {
    fn drop(&mut self) {
        unsafe {
            self.vulkan_device.destroy_image(self.vulkan_image, None);
        }

        unsafe {
            self.vulkan_device.free_memory(self.vulkan_memory, None);
        }
    }
}

#[derive(Component)]
struct InputTexture(usize);

#[derive(Component, Deref)]
struct InputTextureImage(Handle<Image>);

#[derive(Component)]
struct OutputTexture(usize);

#[derive(Component, Deref)]
pub struct WgpuTexture(Texture);

#[derive(Component, Deref)]
pub struct WgpuTextureView(TextureView);

#[derive(Component, PartialEq)]
pub struct TextureKey(TextureDesc);

#[derive(Component)]
struct OutputCamera(usize);

#[derive(Component)]
struct InputTexturedQuad;

#[derive(Component, Deref)]
pub struct CudaArray(CudaArrayInfo);

#[derive(Resource)]
pub struct CudaCtx(Arc<CudaContext>);

#[derive(Deref)]
pub struct CudaStream(*mut CUstream_st);

#[derive(Resource)]
struct AppSettings {
    chromatic_aberration_intensity: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            chromatic_aberration_intensity: 0.01,
        }
    }
}

#[derive(Resource)]
struct PreferredOutput {
    format: PixelFormat,
    resolution: UVec2,
}

impl TopNew for BevyTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: BevyTopParams::default(),
            context: Arc::new(Mutex::new(context)),
            app: None,
            inputs_entities: HashMap::default(),
            output_entities: HashMap::default(),
        }
    }
}

impl OpInfo for BevyTop {
    const OPERATOR_LABEL: &'static str = "Bevy";
    const OPERATOR_TYPE: &'static str = "Bevy";
    const OPERATOR_ICON: &'static str = "BVY";
    const MIN_INPUTS: usize = 1;
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
    fn execute_inner(
        &mut self,
        output: &mut TopOutput,
        top_input: &OperatorInputs<TopInput>,
    ) -> Result<()> {
        if self.app.is_none() {
            self.app = Some(Self::init_bevy_app());
        }

        let Some(app) = self.app.as_mut() else {
            return Err(anyhow::anyhow!("Bevy app is not initialized"));
        };

        Self::update_app_settings(&self.params, app);

        // Setup cuda context and stream
        if let None = app.world().get_resource::<CudaCtx>() {
            let ctx = CudaContext::new(0)
                .map_err(|e| anyhow::anyhow!("Failed to create CUDA context: {:?}", e))?;
            app.world_mut().insert_resource(CudaCtx(ctx));
        }
        if let None = app.world().get_non_send_resource::<CudaStream>() {
            unsafe {
                use cudarc::runtime::sys::*;
                let mut stream = std::ptr::null_mut();
                let result = cudaStreamCreate(&mut stream);
                if result != cudaError::cudaSuccess {
                    return Err(anyhow::anyhow!(
                        "Failed to create CUDA stream: {:?}",
                        result
                    ));
                }
                app.world_mut().insert_non_send_resource(CudaStream(stream));
            }
        }

        let (output_width, output_height, output_format) = Self::get_resolution_and_format(
            top_input.params(),
            top_input.input(0).map(|x| x.texture_desc()),
        );
        app.world_mut().insert_resource(PreferredOutput {
            format: output_format,
            resolution: UVec2::new(output_width as u32, output_height as u32),
        });

        for idx in 0..top_input.num_inputs() {
            let input = top_input
                .input(idx)
                .ok_or_else(|| anyhow::anyhow!("Input {} not found in OperatorInputs", idx))?;
            let cuda_array =
                input.get_cuda_array(app.world().non_send_resource::<CudaStream>().0)?;
            let texture_desc = cuda_array.texture_desc();
            let cuda_array = CudaArray(cuda_array);
            let entity = self.inputs_entities.entry(idx).or_insert_with(|| {
                let image_handle = app
                    .world_mut()
                    .resource_mut::<Assets<Image>>()
                    .reserve_handle();
                app.world_mut()
                    .spawn((
                        InputTexture(idx),
                        InputTextureImage(image_handle),
                        TextureKey(texture_desc),
                    ))
                    .id()
            });
            app.world_mut().entity_mut(*entity).insert(cuda_array);
        }

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

        self.output_entities.entry(0).or_insert_with(|| {
            app.world_mut()
                .spawn((
                    OutputTexture(0),
                    TextureKey(output_array_info.texture_desc()),
                    CudaArray(output_array_info),
                ))
                .id()
        });

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

        Self::update(app);
        Self::export_output(app.world_mut())
            .map_err(|e| anyhow::anyhow!("Failed to export output: {}", e))?;

        self.context
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock context: {}", e))?
            .end_cuda_operations();

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

    fn import_inputs(
        mut commands: Commands,
        arrays: Query<(Entity, &CudaArray, &TextureKey), With<InputTexture>>,
        render_device: Res<RenderDevice>,
        cuda_ctx: Res<CudaCtx>,
        mut shared_texture_external_memory: NonSendMut<SharedTextureExternalMemory>,
    ) -> bevy::prelude::Result {
        for (entity, array, key) in arrays.iter() {
            let texture_desc = array.texture_desc();
            let width = texture_desc.width as u32;
            let height = texture_desc.height as u32;

            if *key != TextureKey(texture_desc) {
                unsafe {
                    render_device
                        .wgpu_device()
                        .as_hal::<wgpu::hal::api::Vulkan, _, _>(|device| {
                            let Some(device) = device else {
                                return Err(anyhow::anyhow!("Failed to get Vulkan device"));
                            };

                            let instance = device.shared_instance().raw_instance();
                            let physical_device = device.raw_physical_device();
                            let vulkan_device = device.raw_device();
                            let (texture, external_memory) = Self::create_input_external_memory(
                                &cuda_ctx.0,
                                instance,
                                vulkan_device,
                                physical_device,
                                &array,
                                width,
                                height,
                            )?;
                            commands
                                .entity(entity)
                                .remove::<WgpuTexture>()
                                .insert(texture);
                            shared_texture_external_memory.insert(entity, external_memory);
                            Ok(())
                        })?;
                }
            }
        }

        Ok(())
    }

    fn import_outputs(
        mut commands: Commands,
        arrays: Query<(Entity, &CudaArray, &TextureKey), With<OutputTexture>>,
        render_device: Res<RenderDevice>,
        cuda_ctx: Res<CudaCtx>,
        mut shared_texture_external_memory: NonSendMut<SharedTextureExternalMemory>,
    ) -> bevy::prelude::Result {
        for (entity, array, key) in arrays.iter() {
            let texture_desc = array.texture_desc();

            if *key != TextureKey(texture_desc) {
                let output_desc = array.texture_desc();
                if *key != TextureKey(output_desc) {
                    unsafe {
                        render_device
                            .wgpu_device()
                            .as_hal::<wgpu::hal::api::Vulkan, _, _>(|device| {
                                let Some(device) = device else {
                                    return Err(anyhow::anyhow!("Failed to get Vulkan device"));
                                };

                                let instance = device.shared_instance().raw_instance();
                                let physical_device = device.raw_physical_device();
                                let vulkan_device = device.raw_device();

                                let (texture, external_memory) =
                                    Self::create_output_external_memory(
                                        &cuda_ctx.0,
                                        instance,
                                        vulkan_device,
                                        physical_device,
                                        array,
                                    )?;

                                commands
                                    .entity(entity)
                                    .remove::<WgpuTexture>()
                                    .insert(texture);
                                shared_texture_external_memory.insert(entity, external_memory);
                                Ok(())
                            })?;
                    };
                }
            }
        }

        Ok(())
    }

    fn create_output_external_memory(
        ctx: &CudaContext,
        instance: &Instance,
        device: &Device,
        physical_device: vk::PhysicalDevice,
        output_array_info: &CudaArrayInfo,
    ) -> Result<(SharedTexture, ExternalMemory)> {
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
        let cuda_external_memory =
            unsafe { import_external_memory_dedicated(&ctx, file, image_mem_reqs.size) }
                .map_err(|e| anyhow::anyhow!("Failed to import output external memory: {:?}", e))?;

        let image_subresource = vk::ImageSubresource {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            array_layer: 0,
        };
        let actual_layout =
            unsafe { device.get_image_subresource_layout(output_image, image_subresource) };
        let row_pitch = actual_layout.row_pitch as usize;

        let output_texture = SharedTexture {
            vulkan_memory: output_memory,
            vulkan_image: output_image,
            vulkan_device: device.clone(),
            width,
            height,
            texture_desc: output_desc.clone(),
            pixel_format: output_desc.pixel_format,
            row_pitch,
        };

        Ok((output_texture, cuda_external_memory))
    }

    fn create_input_external_memory(
        ctx: &CudaContext,
        instance: &Instance,
        device: &Device,
        physical_device: vk::PhysicalDevice,
        cuda_array_info: &CudaArrayInfo,
        width: u32,
        height: u32,
    ) -> Result<(SharedTexture, ExternalMemory)> {
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

        let file = unsafe { File::from_raw_handle(handle as RawHandle) };
        let cuda_external_memory =
            unsafe { import_external_memory_dedicated(ctx, file, image_mem_reqs.size) }
                .map_err(|e| anyhow::anyhow!("Failed to import external memory: {:?}", e))?;

        let input_subresource = vk::ImageSubresource {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            array_layer: 0,
        };
        let input_layout =
            unsafe { device.get_image_subresource_layout(input_image, input_subresource) };
        let row_pitch = input_layout.row_pitch as usize;

        let input_texture = SharedTexture {
            vulkan_memory: input_memory,
            vulkan_image: input_image,
            vulkan_device: device.clone(),
            width,
            height,
            texture_desc: cuda_array_info.texture_desc(),
            pixel_format: cuda_array_info.texture_desc().pixel_format,
            row_pitch,
        };

        Ok((input_texture, cuda_external_memory))
    }

    fn update_input_texture_data(
        input_textures: Query<(Entity, &SharedTexture, &CudaArray), With<InputTexture>>,
        cuda_stream: NonSend<CudaStream>,
        shared_texture_external_memory: NonSend<SharedTextureExternalMemory>,
    ) -> bevy::prelude::Result {
        for (entity, input_texture, cuda_array) in input_textures {
            let cuda_external_memory =
                shared_texture_external_memory.get(&entity).ok_or_else(|| {
                    anyhow::anyhow!(
                        "External memory for input texture {:?} not found",
                        input_texture.vulkan_image
                    )
                })?;
            let device_ptr = cuda_external_memory.map_all_ref().map_err(|e| {
                anyhow::anyhow!("Failed to map input external memory for update: {:?}", e)
            })?;

            let (width, height, row_pitch) = (
                input_texture.width,
                input_texture.height,
                input_texture.row_pitch,
            );

            let input_desc = cuda_array.texture_desc();
            let cuda_array = unsafe { cuda_array.cuda_array() };

            Self::copy_from_array(
                cuda_array,
                device_ptr,
                width,
                height,
                &input_desc.pixel_format,
                row_pitch,
                **cuda_stream,
            )?;
        }

        unsafe {
            use cudarc::runtime::sys::*;
            let sync_result = cudaStreamSynchronize(**cuda_stream);
            if sync_result != cudaError::cudaSuccess {
                return Err(anyhow::anyhow!(
                    "Failed to synchronize CUDA stream after updating input textures: {:?}",
                    sync_result
                ))?;
            }
        }

        Ok(())
    }

    fn sync_textures(
        mut commands: Commands,
        input_textures: Query<
            (Entity, &SharedTexture, &InputTexture),
            (With<InputTexture>, Without<WgpuTexture>),
        >,
        output_textures: Query<
            (Entity, &SharedTexture, &OutputTexture),
            (With<OutputTexture>, Without<WgpuTexture>),
        >,
        render_device: Res<RenderDevice>,
        mut manual_texture_views: ResMut<ManualTextureViews>,
    ) -> bevy::prelude::Result {
        let srgb_view_formats = vec![TextureFormat::Bgra8UnormSrgb];
        let rgba_srgb_view_formats = vec![TextureFormat::Rgba8UnormSrgb];
        let empty_view_formats: Vec<TextureFormat> = vec![];

        for (entity, shared_texture, input_tag) in input_textures {
            let input_format = shared_texture.pixel_format.clone();
            let wgpu_format = pixel_format_to_wgpu_format(&input_format);
            let hal_texture = Self::image_as_hal(
                shared_texture.vulkan_image,
                shared_texture.width,
                shared_texture.height,
                wgpu_format,
            )?;
            let texture_descriptor = TextureDescriptor {
                label: Some("input_texture"),
                size: Extent3d {
                    width: shared_texture.width,
                    height: shared_texture.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: wgpu_format,
                usage: TextureUsages::COPY_SRC | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            };

            let input_texture = unsafe {
                render_device
                    .wgpu_device()
                    .create_texture_from_hal::<wgpu::hal::api::Vulkan>(
                        hal_texture,
                        &texture_descriptor,
                    )
            };
            let input_texture_view =
                input_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let handle = ManualTextureViewHandle(input_tag.0 as u32);
            manual_texture_views.insert(
                handle,
                ManualTextureView {
                    texture_view: input_texture_view.clone().into(),
                    size: UVec2::new(
                        texture_descriptor.size.width,
                        texture_descriptor.size.height,
                    ),
                    format: wgpu_format,
                },
            );

            commands.entity(entity).insert((
                WgpuTexture(Texture::from(input_texture)),
                WgpuTextureView(TextureView::from(input_texture_view)),
                handle,
            ));
        }

        for (entity, output_texture, output_tag) in output_textures {
            let output_format = output_texture.pixel_format.clone();
            let wgpu_format = pixel_format_to_wgpu_format(&output_format);

            let view_formats_slice = match wgpu_format {
                TextureFormat::Bgra8Unorm => &srgb_view_formats,
                TextureFormat::Rgba8Unorm => &rgba_srgb_view_formats,
                _ => &empty_view_formats,
            };

            let hal_texture = Self::image_as_hal(
                output_texture.vulkan_image,
                output_texture.width,
                output_texture.height,
                wgpu_format,
            )?;
            let texture_descriptor = TextureDescriptor {
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
                usage: TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
                view_formats: view_formats_slice,
            };
            let output_texture = unsafe {
                render_device
                    .wgpu_device()
                    .create_texture_from_hal::<wgpu::hal::api::Vulkan>(
                        hal_texture,
                        &texture_descriptor,
                    )
            };

            let output_view_format = match wgpu_format {
                TextureFormat::Bgra8Unorm => TextureFormat::Bgra8UnormSrgb,
                TextureFormat::Rgba8Unorm => TextureFormat::Rgba8UnormSrgb,
                _ => wgpu_format,
            };
            let output_texture_view = output_texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("output_texture_view"),
                format: Some(output_view_format),
                ..Default::default()
            });

            let handle = ManualTextureViewHandle((output_tag.0 * 10) as u32);
            manual_texture_views.insert(
                handle,
                ManualTextureView {
                    texture_view: output_texture_view.clone().into(),
                    size: UVec2::new(
                        texture_descriptor.size.width,
                        texture_descriptor.size.height,
                    ),
                    format: wgpu_format,
                },
            );

            commands.entity(entity).insert((
                WgpuTexture(Texture::from(output_texture)),
                WgpuTextureView(TextureView::from(output_texture_view)),
                handle,
            ));
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

    fn export_output(world: &mut World) -> bevy::prelude::Result {
        let mut outputs = world.query::<(Entity, &CudaArray, &SharedTexture, &OutputTexture)>();
        let cuda_stream = world.non_send_resource::<CudaStream>();
        let shared_texture_external_memory =
            world.non_send_resource::<SharedTextureExternalMemory>();

        let stream = **cuda_stream;

        for (entity, cuda_array, shared_texture, output_tag) in outputs.iter(world) {
            let cuda_array = unsafe { cuda_array.cuda_array() };

            if cuda_array.is_null() {
                return Err(anyhow::anyhow!("Output CUDA array is null"))?;
            }

            let cuda_external_memory =
                shared_texture_external_memory.get(&entity).ok_or_else(|| {
                    anyhow::anyhow!("External memory for output {} not found", output_tag.0)
                })?;
            let device_ptr = cuda_external_memory
                .map_all_ref()
                .map_err(|e| anyhow::anyhow!("Failed to map output external memory: {:?}", e))?;

            Self::copy_to_array(
                device_ptr,
                cuda_array,
                shared_texture.texture_desc.width as u32,
                shared_texture.texture_desc.height as u32,
                &shared_texture.pixel_format,
                shared_texture.row_pitch,
                stream,
            )?;
        }

        if !stream.is_null() {
            unsafe {
                use cudarc::runtime::sys::*;
                let sync_result = cudaStreamSynchronize(stream);
                if sync_result == cudaError::cudaSuccess {}
            }
        }

        Ok(())
    }

    fn update(app: &mut App) {
        app.update();

        let render_device = app.world().resource::<RenderDevice>();
        render_device.wgpu_device().poll(wgpu::Maintain::Wait);
    }

    fn copy_from_array(
        cuda_array: *mut cudarc::runtime::sys::cudaArray,
        device_ptr: sys::CUdeviceptr,
        width: u32,
        height: u32,
        input_format: &PixelFormat,
        vulkan_row_pitch: usize,
        stream: cudarc::runtime::sys::cudaStream_t,
    ) -> Result<()> {
        unsafe {
            use cudarc::runtime::sys::*;

            if cuda_array.is_null() {
                return Ok(());
            }

            let bytes_per_pixel = get_bytes_per_pixel(input_format);
            if bytes_per_pixel == 0 {
                return Err(anyhow::anyhow!("Invalid input format: {:?}", input_format));
            }

            let calculated_pitch = width * bytes_per_pixel as u32;

            let width_in_bytes = calculated_pitch;

            let copy_result = cudaMemcpy2DFromArrayAsync(
                device_ptr as *mut std::ffi::c_void,
                vulkan_row_pitch,
                cuda_array as cudaArray_const_t,
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

    fn copy_to_array(
        device_ptr: sys::CUdeviceptr,
        td_cuda_array: *mut cudarc::runtime::sys::cudaArray,
        width: u32,
        height: u32,
        output_format: &PixelFormat,
        vulkan_row_pitch: usize,
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
            let cuda_aligned_pitch = if vulkan_row_pitch % 512 != 0 {
                ((vulkan_row_pitch + 511) / 512) * 512
            } else {
                vulkan_row_pitch
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

    fn init_bevy_app() -> App {
        let mut app = App::new();

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                })
        );

        app.init_resource::<AppSettings>();
        app.init_non_send_resource::<SharedTextureExternalMemory>();
        app.add_systems(Startup, setup_scene);
        app.add_systems(
            First,
            (
                Self::import_inputs,
                Self::import_outputs,
                Self::update_input_texture_data,
                Self::sync_textures,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                update_camera_target,
                update_input_texture,
                update_chromatic_aberration_settings,
            ),
        );

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(ExtractSchedule, extract_external_textures);

        app.finish();
        app.cleanup();

        app
    }

    fn update_app_settings(params: &BevyTopParams, app: &mut App) {
        let mut app_settings = app.world_mut().resource_mut::<AppSettings>();
        app_settings.chromatic_aberration_intensity = params.chromatic_aberration_intensity as f32;
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
    inputs: Query<&InputTextureImage>,
) {
    let Some(input_handle) = inputs.iter().next() else {
        return;
    };
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some((**input_handle).clone()),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        InputTexturedQuad,
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::new(1.0, -1.0, 1.0)),
        ChromaticAberration::default(),
        OutputCamera(0),
    ));
}

fn update_camera_target(
    mut camera_query: Query<&mut Camera, With<OutputCamera>>,
    output_textures: Query<&ManualTextureViewHandle, With<OutputTexture>>,
) {
    let Some(output_manual_view_handle) = output_textures.iter().next() else {
        warn!("No output texture found for camera target update.");
        return;
    };

    for mut camera in camera_query.iter_mut() {
        camera.target = RenderTarget::TextureView(*output_manual_view_handle);
    }
}

fn update_chromatic_aberration_settings(
    mut chromatic_aberration: Query<&mut ChromaticAberration>,
    app_settings: Res<AppSettings>,
) {
    if app_settings.is_changed() {
        let intensity = app_settings.chromatic_aberration_intensity;

        // Pick a reasonable maximum sample size for the intensity to avoid an
        // artifact whereby the individual samples appear instead of producing
        // smooth streaks of color.
        let max_samples = ((intensity - 0.02) / (0.20 - 0.02) * 56.0 + 8.0)
            .clamp(8.0, 64.0)
            .round() as u32;

        for mut chromatic_aberration in &mut chromatic_aberration {
            chromatic_aberration.intensity = intensity;
            chromatic_aberration.max_samples = max_samples;
        }
    }
}

fn update_input_texture(
    inputs: Query<&InputTextureImage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut quad_query: Query<&MeshMaterial3d<StandardMaterial>, With<InputTexturedQuad>>,
) {
    let Some(input_handle) = inputs.iter().next() else {
        warn!("No input texture found for updating materials.");
        return;
    };

    for material_handle in quad_query.iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color_texture = Some((**input_handle).clone());
        }
    }
}

fn extract_external_textures(
    inputs: Extract<Query<(&InputTextureImage, &WgpuTexture, &WgpuTextureView)>>,
    default_sampler: Res<DefaultImageSampler>,
    mut gpu_images: ResMut<RenderAssets<GpuImage>>,
) {
    for (input_handle, input_texture, input_texture_view) in &inputs {
        let input_gpu_image = GpuImage {
            texture: input_texture.0.clone(),
            texture_view: input_texture_view.0.clone(),
            texture_format: input_texture.0.format(),
            sampler: (**default_sampler).clone(),
            size: Extent3d {
                width: input_texture.0.size().width,
                height: input_texture.0.size().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 0,
        };
        gpu_images.insert((**input_handle).id(), input_gpu_image);
    }
}

top_plugin!(BevyTop);
