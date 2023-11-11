use burn::module::Module;
use burn::record;
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::Backend;
use burn_tch::{TchBackend, TchDevice};
use stablediffusion::model::stablediffusion::{StableDiffusion, StableDiffusionConfig};
use stablediffusion::tokenizer::SimpleTokenizer;
use std::fmt::format;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::thread::JoinHandle;
use td_rs_derive::Params;
use td_rs_top::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

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
    sd_producer: StableDiffusionProducer,
    init: bool,
    prompt: String,
}

impl TopNew for StableDiffusionTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: Default::default(),
            execute_count: 0,
            context,
            sd_producer: StableDiffusionProducer::new(),
            init: false,
            prompt: "".to_string(),
        }
    }
}

impl OpInfo for StableDiffusionTop {
    const OPERATOR_LABEL: &'static str = "Stable Diffusion";
    const OPERATOR_TYPE: &'static str = "Stablediffusion";
    const MAX_INPUTS: usize = 0;
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
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            input_size_index: 0,
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        if !self.params.model.exists() && !self.params.model.is_file() {
            self.set_warning("A model must be loaded!");
            return;
        }
        self.set_warning("");

        if self.prompt != self.params.prompt {
            self.sd_producer.set_prompt(&self.params.prompt);
            self.prompt = self.params.prompt.clone();
        }
        if !self.init {
            self.sd_producer.init_model(&self.params.model);
            self.init = true;
        }

        if let Some(image) = self.sd_producer.get_image() {
            let mut buf = self
                .context
                .create_output_buffer(image.len(), TopBufferFlags::None);
            buf.data_mut().copy_from_slice(image.as_slice());

            let info = UploadInfo {
                buffer_offset: 0,
                texture_desc: TextureDesc {
                    tex_dim: TexDim::E2D,
                    width: WIDTH,
                    height: HEIGHT,
                    pixel_format: PixelFormat::BGRA8Fixed,
                    aspect_x: 0.0,
                    depth: 1,
                    aspect_y: 0.0,
                },
                first_pixel: FirstPixel::TopLeft,
                color_buffer_index: 0,
            };
            output.upload_buffer(&mut buf, &info);
        }
    }
}

struct StableDiffusionProducer {
    sd: Arc<RwLock<Option<StableDiffusion<TchBackend<f32>>>>>,
    prompt: Arc<RwLock<String>>,
    produce_loop: JoinHandle<()>,
    rx: Receiver<Vec<u8>>,
    trigger_tx: SyncSender<()>,
}

impl StableDiffusionProducer {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(3);
        let (trigger_tx, trigger_rx) = std::sync::mpsc::sync_channel(1);
        let tokenizer = SimpleTokenizer::new().unwrap();
        let sd = Arc::new(RwLock::new(None::<StableDiffusion<TchBackend<f32>>>));
        let prompt = Arc::new(RwLock::new(String::new()));
        let produce_loop_sd = sd.clone();
        let produce_loop_prompt = prompt.clone();
        let produce_loop = Self::produce_loop(
            tx,
            trigger_rx,
            tokenizer,
            produce_loop_sd,
            produce_loop_prompt,
        );

        StableDiffusionProducer {
            sd,
            rx,
            trigger_tx,
            prompt,
            produce_loop,
        }
    }

    fn produce_loop(
        tx: SyncSender<Vec<u8>>,
        trigger_rx: Receiver<()>,
        tokenizer: SimpleTokenizer,
        produce_loop_sd: Arc<RwLock<Option<StableDiffusion<TchBackend<f32>>>>>,
        produce_loop_prompt: Arc<RwLock<String>>,
    ) -> JoinHandle<()> {
        let produce_loop = std::thread::spawn(move || {
            loop {
                // Wait for a frame to be requested
                let _ = trigger_rx.recv().unwrap();

                let sd = produce_loop_sd.read().unwrap();
                match sd.as_ref() {
                    None => {}
                    Some(sd) => {
                        let device = TchDevice::Cuda(0);
                        let sd = sd.clone();
                        let sd = sd.to_device(&device);
                        let unconditional_context = sd.unconditional_context(&tokenizer);
                        let unconditional_guidance_scale: f64 = 7.5;
                        let n_steps: usize = 20;

                        let prompt = produce_loop_prompt.read().unwrap();
                        let context = sd.context(&tokenizer, &prompt).unsqueeze::<3>(); //.repeat(0, 2); // generate 2 samples
                        let images = sd.sample_image(
                            context,
                            unconditional_context,
                            unconditional_guidance_scale,
                            n_steps,
                        );

                        let image = &images[0];
                        let layer_bytes = (WIDTH * HEIGHT * 4 * std::mem::size_of::<u8>()) as u64;
                        let mut pixels = Vec::with_capacity(layer_bytes as usize);

                        for chunk in image.chunks(3) {
                            pixels.push(chunk[2]); // Blue
                            pixels.push(chunk[1]); // Green
                            pixels.push(chunk[0]); // Red
                            pixels.push(255); // Alpha (full opacity)
                        }

                        tx.send(pixels).unwrap();
                    }
                }
            }
        });
        produce_loop
    }

    fn init_model(&mut self, model_file: &PathBuf) {
        let self_sd = self.sd.clone();
        let model_file = model_file.clone();
        // Load the model in a separate thread
        std::thread::spawn(move || {
            let sd = Self::load_stable_diffusion_model_file(&model_file).unwrap();
            *self_sd.write().unwrap() = Some(sd);
        });
        return;
    }

    fn set_prompt(&mut self, p: &str) {
        let mut prompt = self.prompt.write().unwrap();
        *prompt = p.to_string();
    }

    fn get_image(&self) -> Option<Vec<u8>> {
        match self.trigger_tx.try_send(()) {
            Ok(_) => {}
            Err(err) => {
                match err {
                    TrySendError::Full(_) => {
                        // would block, so just return
                    }
                    TrySendError::Disconnected(_) => {
                        panic!("Stable Diffusion Producer thread disconnected!")
                    }
                }
            }
        };

        match self.rx.try_recv() {
            Ok(img) => {
                return Some(img);
            }
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    panic!("Stable Diffusion Producer thread disconnected!")
                }
            },
        };

        None
    }
    fn load_stable_diffusion_model_file<B: Backend>(
        filename: &PathBuf,
    ) -> Result<StableDiffusion<B>, record::RecorderError> {
        BinFileRecorder::<FullPrecisionSettings>::new()
            .load(filename.into())
            .map(|record| StableDiffusionConfig::new().init().load_record(record))
    }
}

top_plugin!(StableDiffusionTop);
