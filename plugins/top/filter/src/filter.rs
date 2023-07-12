fn diffuse_error(pixel: &mut u32, error: u32, coeff: f64) {
    let px_chan: &mut [u8; 4] = unsafe { std::mem::transmute(pixel) };
    let e_chan: &[u8; 4] = unsafe { std::mem::transmute(&error) };

    for i in 0..3 {
        let temp = px_chan[i] as f64 + coeff * e_chan[i] as f64;
        if temp > u8::MAX as f64 {
            px_chan[i] = u8::MAX;
        } else {
            px_chan[i] = temp as u8;
        }
    }
}

fn resize_image(
    in_pixel: &[u32],
    in_width: usize,
    in_height: usize,
    out_width: usize,
    out_height: usize,
) -> Vec<u32> {
    let mut ret = vec![0u32; out_width * out_height];
    let scale_y = in_width as f64 / out_width as f64;
    let scale_x = in_height as f64 / out_height as f64;

    for y in 0..out_height {
        for x in 0..out_width {
            let in_y = (y as f64 * scale_y) as usize;
            let in_x = (x as f64 * scale_x) as usize;
            ret[y * out_width + x] = in_pixel[in_y * in_width + in_x];
        }
    }
    ret
}

fn closest_palette_color(color: u32, color_bits: u8) -> u32 {
    let shift = 8 - color_bits;
    let [red, green, blue, alpha] = color.to_le_bytes();
    let red = (red >> shift) << shift;
    let green = (green >> shift) << shift;
    let blue = (blue >> shift) << shift;
    u32::from_le_bytes([red, green, blue, alpha])
}

fn do_dithering(in_pixel: &[u32], out_pixel: &mut [u32], w: usize, h: usize, color_bits: u8) {
    // for i in 0..h {
    //     for j in 0..w {
    //         let old_px = in_pixel[i * w + j];
    //         let new_px = closest_palette_color(old_px, color_bits);
    //         let error = old_px.wrapping_sub(new_px);
    //         out_pixel[i * w + j] = new_px;
    //
    //         if j != w - 1 {
    //             diffuse_error(&mut in_pixel[i * w + j + 1], error, 7.0 / 16.0);
    //         }
    //         if j != 0 && i != h - 1 {
    //             diffuse_error(&mut in_pixel[(i + 1) * w + j - 1], error, 3.0 / 16.0);
    //         }
    //         if i != h - 1 {
    //             diffuse_error(&mut in_pixel[(i + 1) * w + 1], error, 5.0 / 16.0);
    //         }
    //         if j != w - 1 && i != h - 1 {
    //             diffuse_error(&mut in_pixel[(i + 1) * w + j + 1], error, 1.0 / 16.0);
    //         }
    //     }
    // }
}

fn limit_colors(in_pixel: &[u32], out_pixel: &mut [u32], w: usize, h: usize, color_bits: u8) {
    for i in 0..w * h {
        out_pixel[i] = closest_palette_color(in_pixel[i], color_bits);
    }
}

pub struct Filter;

impl Filter {
    pub(crate) fn do_filter_work(
        in_buffer: &[u32],
        in_width: usize,
        in_height: usize,
        out_buffer: &mut [u32],
        out_width: usize,
        out_height: usize,
        do_dither: bool,
        bits_per_color: u8,
    ) {
        let needs_resize = in_height != out_height || in_width != out_width;

        let mut buf = Vec::new();
        if needs_resize {
            buf = resize_image(in_buffer, in_width, in_height, out_width, out_height);
        }

        if do_dither {
            do_dithering(
                &in_buffer,
                out_buffer,
                out_width,
                out_height,
                bits_per_color,
            );
        } else {
            limit_colors(
                &in_buffer,
                out_buffer,
                out_width,
                out_height,
                bits_per_color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let test_cases = [];

        for &(color, color_bits, expected) in &test_cases {
            let result = closest_palette_color(color, color_bits);
            assert_eq!(
                result,
                expected,
                "Expected output for input color {:#32b} and {} bits to be {:#32b}, but got {:#32b}",
                color,
                color_bits,
                expected,
                result
            );
        }
    }
}
