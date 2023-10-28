use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use td_rs_top::{TopBuffer, TopBufferFlags, TopContext, UploadInfo};

pub struct BufferInfo {
    pub buf: Option<Arc<TopBuffer>>,
    pub upload_info: UploadInfo,
}

pub struct FrameQueue {
    context: Arc<Mutex<TopContext>>,
    updated_buffers: Mutex<VecDeque<BufferInfo>>,
}

impl FrameQueue {
    pub fn new(context: Arc<Mutex<TopContext>>) -> Self {
        Self {
            context,
            updated_buffers: Mutex::new(VecDeque::new()),
        }
    }

    pub fn get_buffer_to_update(&mut self, byte_size: usize, flags: TopBufferFlags) -> Option<Arc<TopBuffer>> {
        let mut buffers = self.updated_buffers.lock().unwrap();

        const MAX_QUEUE_SIZE: usize = 2;

        // If we've already reached the max queue size, replace the oldest buffer
        if buffers.len() >= MAX_QUEUE_SIZE {
            let old_buf = buffers.pop_front()?.buf;

            if let Some(b) = &old_buf {
                if b.size() < byte_size || b.size() > byte_size * 2 || b.flags() != flags {
                    let mut ctx = self.context.lock().unwrap();
                    return Some(Arc::new(ctx.create_output_buffer(byte_size as usize, flags)));
                }
                return old_buf;
            }
        }
        let mut ctx = self.context.lock().unwrap();
        Some(Arc::new(ctx.create_output_buffer(byte_size as usize, flags)))
    }

    pub fn update_complete(&self, buf_info: BufferInfo) {
        let mut buffers = self.updated_buffers.lock().unwrap();
        buffers.push_back(buf_info);
    }

    pub fn update_cancelled(&self, buf: Arc<TopBuffer>) {
        drop(buf);
    }

    pub fn get_buffer_to_upload(&self) -> Option<BufferInfo> {
        let mut buffers = self.updated_buffers.lock().unwrap();
        buffers.pop_front()
    }
}
