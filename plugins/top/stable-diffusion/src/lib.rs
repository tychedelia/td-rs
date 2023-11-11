use burn::module::Module;
use burn::record;
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::Backend;
use burn_tch::{TchBackend, TchDevice};
use stablediffusion::model::stablediffusion::{StableDiffusion, StableDiffusionConfig};
use stablediffusion::tokenizer::SimpleTokenizer;
use std::fmt::format;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use td_rs_derive::Params;
use td_rs_top::*;

#[derive(Params, Default, Clone, Debug)]
struct StableDiffusionTopParams {
    #[param(label = "Reset")]
    reset: Pulse,
    #[param(label = "Prompt")]
    prompt: String,
    #[param(label = "Model")]
    model: FileParam,
}

/// Struct representing our SOP's state
pub struct StableDiffusionTop {
    params: StableDiffusionTopParams,
    execute_count: u32,
    context: TopContext,
    sd: Option<StableDiffusion<TchBackend<f32>>>,
    pub tokenizer: SimpleTokenizer,
}

impl StableDiffusionTop {
    fn load_stable_diffusion_model_file<B: Backend>(
        self: &Self,
        filename: &PathBuf,
    ) -> Result<StableDiffusion<B>, record::RecorderError> {
        BinFileRecorder::<FullPrecisionSettings>::new()
            .load(filename.into())
            .map(|record| StableDiffusionConfig::new().init().load_record(record))
    }
}

impl TopNew for StableDiffusionTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        let tokenizer = SimpleTokenizer::new().unwrap();
        Self {
            params: Default::default(),
            execute_count: 0,
            context,
            sd: None,
            tokenizer,
        }
    }
}

impl OpInfo for StableDiffusionTop {
    const OPERATOR_LABEL: &'static str = "Stable Diffusion";
    const OPERATOR_TYPE: &'static str = "Stable Diffusion";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 0;
}

impl TopInfo for StableDiffusionTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cpu;
}

impl Op for StableDiffusionTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reset" {}
    }
}

impl Top for StableDiffusionTop {
    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        if !self.params.model.exists() && !self.params.model.is_file() {
            return;
        }

        type Backend = TchBackend<f32>;
        let device = TchDevice::Cuda(0);

        let unconditional_guidance_scale: f64 = 7.5;
        let n_steps: usize = 20;
        let prompt = self.params.prompt.as_str();
        if self.sd.is_none() {
            let mut sd = self.load_stable_diffusion_model_file(&self.params.model);
            match sd {
                Err(err) => self.set_error(format!("Error loading model: {}", err).as_str()),
                Ok(sd) => {
                    self.sd = Some(sd.clone());
                }
            }
            return;
        }

        if let Some(sd) = &self.sd {
            let sd = sd.clone().to_device(&device);
            let unconditional_context = sd.unconditional_context(&self.tokenizer);
            let context = sd.context(&self.tokenizer, prompt).unsqueeze::<3>(); //.repeat(0, 2); // generate 2 samples

            let images = sd.sample_image(
                context,
                unconditional_context,
                unconditional_guidance_scale,
                n_steps,
            );

            let image = &images[0];
            let mut buf = self
                .context
                .create_output_buffer(image.len(), TopBufferFlags::None);
            buf.data_mut().copy_from_slice(image.as_slice());
            let height = 512;
            let width = 512;

            let info = UploadInfo {
                buffer_offset: 0,
                texture_desc: TextureDesc {
                    tex_dim: TexDim::E2D,
                    width,
                    height,
                    pixel_format: PixelFormat::BGRA8Fixed,
                    aspect_x: 0.0,
                    depth: 1,
                    aspect_y: 0.0,
                },
                first_pixel: FirstPixel::BottomLeft,
                color_buffer_index: 0,
            };
            output.upload_buffer(&mut buf, &info);
        }
    }
}

top_plugin!(StableDiffusionTop);
