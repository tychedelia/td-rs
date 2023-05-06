use std::sync::{RwLock};
use std::{thread, time};
use monome::{Monome, MonomeEvent};
use monome::MonomeDeviceType::Arc;
use crate::Chop;
use crate::ffi::{ChopGeneralInfo, ChopOperatorInputs, ChopOutput, ChopOutputInfo};

// Last delta for each encoder
#[derive(Debug)]
struct ArcState(i32, i32, i32, i32);

struct ArcListener {
    state: std::sync::Arc<RwLock<ArcState>>
}

impl ArcListener {
    fn new(state: std::sync::Arc<RwLock<ArcState>>) -> Self {
        Self { state }
    }

    fn listen(self) {
        let mut monome = Monome::new("/prefix".to_string()).unwrap();
        let mut led = [0.; 4];

        for i in 0..4 {
            monome.ring_all(i, 0);
            monome.ring_set(i, led[i] as u32, 15);
        }

        loop {
            loop {
                let e = monome.poll();

                match e {
                    Some(MonomeEvent::EncoderDelta { n, delta }) => {
                        let mut s = self.state.write().unwrap();
                        match n {
                            0 => s.0 += delta,
                            1 => s.1 += delta,
                            2 => s.2 += delta,
                            3 => s.3 += delta,
                            _ => panic!("unknown encoder")
                        }

                        let n = n as usize;
                        monome.ring_set(n, led[n] as u32, 0);
                        led[n] = led[n] + (delta as f32 / 4.);
                        if led[n] < 0. {
                            led[n] += 64.;
                        }
                        monome.ring_set(n, led[n] as u32, 15);
                    }
                    _ => {
                        break;
                    }
                }
            }

            let refresh = time::Duration::from_millis(10);
            thread::sleep(refresh);
        }
    }
}


pub struct ArcChop {
    state: std::sync::Arc<RwLock<ArcState>>
}

impl ArcChop {
    pub(crate) fn new() -> Self {
        let state = std::sync::Arc::new(RwLock::new(ArcState(0, 0,0, 0)));

        let listener_state = state.clone();
        std::thread::spawn(move || {
            ArcListener::new(listener_state).listen();
        });

        Self {
            state
        }
    }
}

impl Chop for ArcChop {
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        info.sample_rate = 60.0;
        info.num_samples = 1;
        info.num_channels = 4;
        true
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
        let s = self.state.read().unwrap();

        output.channels[0].data.push(s.0 as f32);
        output.channels[1].data.push(s.1 as f32);
        output.channels[2].data.push(s.2 as f32);
        output.channels[3].data.push(s.3 as f32);
    }

    fn get_general_info(&self) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: true,
            timeslice: false,
            ..Default::default()
        }
    }
}