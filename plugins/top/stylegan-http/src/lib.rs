use futures::task::noop_waker_ref;
use std::collections::VecDeque;
use std::future;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::Poll;
use td_rs_derive::Params;
use td_rs_top::*;
use tokio::task::JoinHandle;

const URL: &str = "http://localhost:5000";
const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const MAX_TASKS: usize = 10;

#[derive(Params, Default, Clone, Debug)]
struct StyleganHttpTopParams {
    #[param(label = "Seed", page = "Network")]
    seed: u16,
    #[param(label = "X Feature", page = "Network")]
    x_feature: u16,
    #[param(label = "X Range", page = "Network")]
    x_range: u16,
    #[param(label = "Y Feature", page = "Network")]
    y_feature: u16,
    #[param(label = "Y Range", page = "Network")]
    y_range: u16,
    #[param(label = "Z Feature", page = "Network")]
    z_feature: u16,
    #[param(label = "Z Range", page = "Network")]
    z_range: u16,
    #[param(label = "Blocking")]
    blocking: bool,
}

pub type Task = JoinHandle<anyhow::Result<Vec<u8>>>;

/// Struct representing our SOP's state
pub struct StyleganHttpTop {
    params: StyleganHttpTopParams,
    execute_count: u32,
    context: TopContext,
    tasks: VecDeque<Task>,
    last_req: Option<ImageReq>,
}

impl Future for StyleganHttpTop {
    type Output = Option<Vec<u8>>;

    // Poll internal tasks,s
    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let req = self.params_as_req();
        if self.last_req.as_ref() != Some(&req) && self.tasks.len() < MAX_TASKS {
            self.tasks.push_back(tokio::spawn(Self::request_image(req.clone())));
            self.last_req = Some(req);
        };

        // While we have tasks, poll them
        // If they're ready, return the image
        // If they're failed, throw them away
        // If they're not ready, reinsert them at the beginning
        while let Some(mut task) = self.tasks.pop_front() {
            // Pin'n'poll
            match Pin::new(&mut task).poll(cx) {
                Poll::Ready(Ok(Ok(image))) => {
                    return Poll::Ready(Some(image));
                }
                Poll::Ready(Ok(Err(_))) | Poll::Ready(Err(_)) => {
                    continue;
                }
                Poll::Pending => {
                    self.tasks.insert(0, task);
                    return Poll::Ready(None);
                }
            }
        }

        return Poll::Ready(None);
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
struct ImageReq {
    seed: u16,
    x: u16,
    x_range: u16,
    y: u16,
    y_range: u16,
    z: u16,
    z_range: u16,
}

impl StyleganHttpTop {
    fn get_image(&mut self) -> Option<Vec<u8>> {
        RUNTIME.block_on(self)
    }

    fn params_as_req(&self) -> ImageReq {
        ImageReq {
            seed: self.params.seed,
            x: self.params.x_feature,
            x_range: self.params.x_range,
            y: self.params.y_feature,
            y_range: self.params.y_range,
            z: self.params.z_feature,
            z_range: self.params.z_range,
        }
    }

    async fn request_image(image_req: ImageReq) -> anyhow::Result<Vec<u8>> {
        let ImageReq {
            seed,
            x,
            x_range,
            y,
            y_range,
            z,
            z_range,
        } = image_req;
        let mut bytes = reqwest::get(format!(
            "{URL}?seed={seed}&x={x}&x_range={x_range}&y={y}&y_range={y_range}&z={z}&z_range={z_range}"
        ))
        .await?
        .bytes()
        .await?
        .to_vec();

        let image = bytes
            .chunks_exact_mut(3)
            .map(|c| {
                let mut c = c.to_vec();
                c.reverse();
                c.push(255);
                c
            })
            .flatten()
            .collect::<Vec<u8>>();

        Ok(image)
    }
}

impl TopNew for StyleganHttpTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: Default::default(),
            execute_count: 0,
            context,
            tasks: VecDeque::with_capacity(MAX_TASKS),
            last_req: None,
        }
    }
}

impl OpInfo for StyleganHttpTop {
    const OPERATOR_LABEL: &'static str = "Stylegan Http";
    const OPERATOR_TYPE: &'static str = "Styleganhttpn";
    const MAX_INPUTS: usize = 0;
    const MIN_INPUTS: usize = 0;
}

impl TopInfo for StyleganHttpTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cpu;
}

impl Op for StyleganHttpTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reset" {}
    }
}

impl Top for StyleganHttpTop {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            input_size_index: 0,
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

        if let Some(mut image) = self.get_image() {
            // kick off another request optimistically
            self.get_image();
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

top_plugin!(StyleganHttpTop);
