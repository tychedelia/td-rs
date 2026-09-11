//! CPU readback path for platforms without CUDA interop (e.g. macOS).

use anyhow::{anyhow, Result};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::camera::{ManualTextureView, ManualTextureViewHandle, ManualTextureViews};
use bevy::render::render_resource::{Buffer, Texture, TextureDescriptor, TextureFormat};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use td_rs_top::*;
use wgpu::{Extent3d, TextureDimension, TextureUsages};

use crate::{
    get_bytes_per_pixel, pixel_format_to_wgpu_format, BevyTop, InputTexture, InputTextureImage,
    OutputTexture, PreferredOutput,
};

const OUTPUT_VIEW_HANDLE: ManualTextureViewHandle = ManualTextureViewHandle(1000);

pub struct CpuOutputTarget {
    texture: Texture,
    desc: TextureDesc,
    padded_bytes_per_row: usize,
    readback: Buffer,
}

impl BevyTop {
    pub(crate) fn execute_cpu(
        &mut self,
        output: &mut TopOutput,
        top_input: &OperatorInputs<TopInput>,
    ) -> Result<()> {
        if self.app.is_none() {
            self.app = Some(Self::init_bevy_app());
        }

        let Some(app) = self.app.as_mut() else {
            return Err(anyhow!("Bevy app is not initialized"));
        };

        Self::update_app_settings(&self.params, app);

        let (output_width, output_height, output_format) = Self::resolve_output_desc(output);
        app.world_mut().insert_resource(PreferredOutput {
            format: output_format,
            resolution: UVec2::new(output_width as u32, output_height as u32),
        });

        Self::upload_inputs(app, &mut self.inputs_entities, top_input)?;
        Self::ensure_output_target(
            app,
            &mut self.cpu_output,
            &mut self.output_entities,
            output_width,
            output_height,
            output_format,
        )?;

        Self::update(app);

        Self::readback_and_upload(app, &self.cpu_output, &self.context, output)
    }

    fn upload_inputs(
        app: &mut App,
        inputs_entities: &mut HashMap<usize, Entity>,
        top_input: &OperatorInputs<TopInput>,
    ) -> Result<()> {
        for idx in 0..top_input.num_inputs() {
            let Some(input) = top_input.input(idx) else {
                continue;
            };

            let mut downloaded = input.download_texture(DownloadOptions {
                vertical_flip: false,
                pixel_format: PixelFormat::BGRA8Fixed,
                ..Default::default()
            });
            let desc = downloaded.texture_desc();
            let expected = desc.width * desc.height * get_bytes_per_pixel(&desc.pixel_format);
            let data = downloaded.data::<u8>();
            if data.len() < expected {
                return Err(anyhow!(
                    "Downloaded input {} is {} bytes, expected {}",
                    idx,
                    data.len(),
                    expected
                ));
            }

            let image = Image::new(
                Extent3d {
                    width: desc.width as u32,
                    height: desc.height as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                data[..expected].to_vec(),
                TextureFormat::Bgra8UnormSrgb,
                RenderAssetUsages::RENDER_WORLD,
            );

            let entity = *inputs_entities.entry(idx).or_insert_with(|| {
                let image_handle = app
                    .world_mut()
                    .resource_mut::<Assets<Image>>()
                    .reserve_handle();
                app.world_mut()
                    .spawn((InputTexture(idx), InputTextureImage(image_handle)))
                    .id()
            });
            let handle = app
                .world()
                .entity(entity)
                .get::<InputTextureImage>()
                .ok_or_else(|| anyhow!("Input entity {} has no image handle", idx))?
                .0
                .clone();
            app.world_mut()
                .resource_mut::<Assets<Image>>()
                .insert(handle.id(), image);
        }

        Ok(())
    }

    fn ensure_output_target(
        app: &mut App,
        cpu_output: &mut Option<CpuOutputTarget>,
        output_entities: &mut HashMap<usize, Entity>,
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
    ) -> Result<()> {
        let desc = TextureDesc {
            width,
            height,
            depth: 1,
            tex_dim: TexDim::E2D,
            pixel_format,
            aspect_x: 0.0,
            aspect_y: 0.0,
        };

        if cpu_output.as_ref().is_some_and(|t| t.desc == desc) {
            return Ok(());
        }

        let bytes_per_pixel = get_bytes_per_pixel(&pixel_format);
        if bytes_per_pixel == 0 {
            return Err(anyhow!("Invalid output format: {:?}", pixel_format));
        }

        let wgpu_format = pixel_format_to_wgpu_format(&pixel_format);
        let srgb_view_formats = [TextureFormat::Bgra8UnormSrgb];
        let rgba_srgb_view_formats = [TextureFormat::Rgba8UnormSrgb];
        let view_formats: &[TextureFormat] = match wgpu_format {
            TextureFormat::Bgra8Unorm => &srgb_view_formats,
            TextureFormat::Rgba8Unorm => &rgba_srgb_view_formats,
            _ => &[],
        };

        let render_device = app.world().resource::<RenderDevice>().clone();
        let texture = render_device.create_texture(&TextureDescriptor {
            label: Some("bevy_top_cpu_output"),
            size: Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: wgpu_format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_SRC
                | TextureUsages::TEXTURE_BINDING,
            view_formats,
        });

        let view_format = match wgpu_format {
            TextureFormat::Bgra8Unorm => TextureFormat::Bgra8UnormSrgb,
            TextureFormat::Rgba8Unorm => TextureFormat::Rgba8UnormSrgb,
            _ => wgpu_format,
        };
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("bevy_top_cpu_output_view"),
            format: Some(view_format),
            ..Default::default()
        });

        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
        let readback = render_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bevy_top_cpu_readback"),
            size: (padded_bytes_per_row * height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        app.world_mut()
            .resource_mut::<ManualTextureViews>()
            .insert(
                OUTPUT_VIEW_HANDLE,
                ManualTextureView {
                    texture_view,
                    size: UVec2::new(width as u32, height as u32),
                    format: wgpu_format,
                },
            );

        output_entities.entry(0).or_insert_with(|| {
            app.world_mut()
                .spawn((OutputTexture(0), OUTPUT_VIEW_HANDLE))
                .id()
        });

        *cpu_output = Some(CpuOutputTarget {
            texture,
            desc,
            padded_bytes_per_row,
            readback,
        });

        Ok(())
    }

    fn readback_and_upload(
        app: &mut App,
        cpu_output: &Option<CpuOutputTarget>,
        context: &Arc<Mutex<TopContext>>,
        output: &mut TopOutput,
    ) -> Result<()> {
        let Some(target) = cpu_output.as_ref() else {
            return Err(anyhow!("No CPU output target"));
        };

        let render_device = app.world().resource::<RenderDevice>().clone();
        let render_queue = app.world().resource::<RenderQueue>().clone();

        let mut encoder =
            render_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("bevy_top_readback"),
            });
        encoder.copy_texture_to_buffer(
            target.texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &target.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(target.padded_bytes_per_row as u32),
                    rows_per_image: None,
                },
            },
            Extent3d {
                width: target.desc.width as u32,
                height: target.desc.height as u32,
                depth_or_array_layers: 1,
            },
        );
        render_queue.submit([encoder.finish()]);

        let slice = target.readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
        render_device.wgpu_device().poll(wgpu::Maintain::Wait);
        rx.recv()
            .map_err(|e| anyhow!("Readback callback dropped: {}", e))?
            .map_err(|e| anyhow!("Failed to map readback buffer: {:?}", e))?;

        let bytes_per_pixel = get_bytes_per_pixel(&target.desc.pixel_format);
        let unpadded_bytes_per_row = target.desc.width * bytes_per_pixel;
        let size = unpadded_bytes_per_row * target.desc.height;

        let mut ctx = context
            .lock()
            .map_err(|e| anyhow!("Failed to lock context: {}", e))?;
        let mut buf = ctx.create_output_buffer(size, TopBufferFlags::None);
        {
            let mapped = slice.get_mapped_range();
            let out = buf.data_mut::<u8>();
            for row in 0..target.desc.height {
                let src = row * target.padded_bytes_per_row;
                let dst = row * unpadded_bytes_per_row;
                out[dst..dst + unpadded_bytes_per_row]
                    .copy_from_slice(&mapped[src..src + unpadded_bytes_per_row]);
            }
        }
        target.readback.unmap();
        drop(ctx);

        let info = UploadInfo {
            buffer_offset: 0,
            texture_desc: target.desc.clone(),
            first_pixel: FirstPixel::TopLeft,
            color_buffer_index: 0,
            ..Default::default()
        };
        output.upload_buffer(&mut buf, &info);

        Ok(())
    }
}
