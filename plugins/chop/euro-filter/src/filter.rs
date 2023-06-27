use std::f64::consts::PI;

pub struct LowPassFilter {
    first_filtering: bool,
    hat_x_prev: f64,
    hat_x: f64,
}

impl LowPassFilter {
    pub fn new() -> Self {
        Self {
            first_filtering: true,
            hat_x_prev: 0.0,
            hat_x: 0.0,
        }
    }

    pub fn filter(&mut self, value: f64, alpha: f64) -> f64 {
        if self.first_filtering {
            self.first_filtering = false;
            self.hat_x_prev = value;
        }
        self.hat_x = alpha * value + (1.0 - alpha) * self.hat_x_prev;
        self.hat_x_prev = self.hat_x;
        self.hat_x
    }

    pub fn hat_x_prev(&self) -> f64 {
        self.hat_x_prev
    }
}

pub struct OneEuroImpl {
    first_filtering: bool,
    rate: f64,
    min_cut_off: f64,
    beta: f64,
    x_filt: LowPassFilter,
    d_cut_off: f64,
    dx_filt: LowPassFilter,
}

impl OneEuroImpl {
    pub fn new(rate: f64, min_cut_off: f64, beta: f64, d_cut_off: f64) -> Self {
        Self {
            first_filtering: true,
            rate,
            min_cut_off,
            beta,
            x_filt: LowPassFilter::new(),
            d_cut_off,
            dx_filt: LowPassFilter::new(),
        }
    }

    pub fn change_input(&mut self, rate: f64, min_cut_off: f64, beta: f64, d_cut_off: f64) {
        self.rate = rate;
        self.min_cut_off = min_cut_off;
        self.beta = beta;
        self.d_cut_off = d_cut_off;
    }

    pub fn filter(&mut self, x: f64) -> f64 {
        let dx = if self.first_filtering {
            0.0
        } else {
            (x - self.x_filt.hat_x_prev()) * self.rate
        };
        self.first_filtering = false;
        let edx = self.dx_filt.filter(dx, Self::alpha(self.d_cut_off));
        let cut_off = self.min_cut_off + self.beta * edx.abs();
        self.x_filt.filter(x, Self::alpha(cut_off))
    }

    fn alpha(cutoff: f64) -> f64 {
        let tau = 1.0 / (2.0 * PI * cutoff);
        let te = 1.0 / cutoff;
        1.0 / (1.0 + tau / te)
    }
}
