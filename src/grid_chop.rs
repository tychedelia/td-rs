use std::sync::{Arc, RwLock};
use monome::{Monome, MonomeEvent};
use crate::{Chop, ChopGeneralInfo, ChopOutputInfo};
use crate::ffi::{ChopOperatorInputs, ChopOutput};

struct Grid {
    grid: Vec<u8>,
    state: Arc<RwLock<Vec<u8>>>,
}

impl Grid {
    pub fn new(state: Arc<RwLock<Vec<u8>>>) -> Self {
        Self {
            grid: vec![0; 128],
            state,
        }
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.grid[y * 16 + x] = val;
    }

    fn listen(mut self) {
        let mut monome = Monome::new("/prefix".to_string()).unwrap();
        monome.set_all_intensity(&self.grid);

        loop {
            loop {
                let e = monome.poll();

                match e {
                    Some(MonomeEvent::GridKey { x, y, direction }) => {
                        let x = x as usize;
                        let y = y as usize;

                        {
                            let mut s = self.state.write().unwrap();
                            s[x] = (255 - y * 32) as u8;
                        }

                        for k in 0..y {
                            println!("{}", k);
                            self.set(x, k, 0);
                        }
                        for k in y..8 {
                            println!("{}", k);
                            self.set(x, k, (16 - (k + 1) * 2) as u8);
                        }

                        monome.set_all_intensity(&self.grid);
                    }
                    _ => {
                        break;
                    }
                }
            }

            let refresh = std::time::Duration::from_millis(10);
            std::thread::sleep(refresh);
        }
    }
}

pub struct GridChop {
    state: Arc<RwLock<Vec<u8>>>
}

impl GridChop {
    pub fn new() -> Self {
        let state = Arc::new(RwLock::new(vec![0; 16]));

        let grid = Grid::new(state.clone());
        std::thread::spawn(move || {
            grid.listen();
        });

        Self {
            state
        }
    }

}

impl Chop for GridChop {
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        info.sample_rate = 60.0;
        info.num_samples = 1;
        info.num_channels = 16;
        true
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
        let s = self.state.read().unwrap();
        for (i, chan) in output.channels.iter_mut().enumerate() {
            chan.data.push(s[i] as f32);
        }
    }

    fn get_general_info(&self) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: true,
            timeslice: false,
            ..Default::default()
        }
    }
}