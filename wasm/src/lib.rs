use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
mod color;

const MAX_ITERATIONS: u32 = 1000;

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
    zoom: f64
) -> Result<(), JsValue> {
    let c = Complex { real, imaginary };
    let mut data = get_mb_set(width, height, c, zoom);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_mb_set(width: u32, height: u32, c: Complex, zoom: f64) -> Vec<u8> {
    let mut data = Vec::new();

    for x in 0..height {
        for y in 0..width {
            let adj = if zoom <= 0.0 { 2.0 } else { 2.0 / zoom };
            let real = scale(y as f64, 0.0, width as f64, c.real - adj, c.real + adj);
            let imaginary = scale(x as f64, 0.0, height as f64, c.imaginary + adj, c.imaginary - adj);
            let c = Complex { real, imaginary };
            let iter_index = get_iter_index(c);

            // coloring
            let mut hsl = color::HSL::default();
            if (iter_index != MAX_ITERATIONS) {
                let mut hue = 255.0 * iter_index as f64 / MAX_ITERATIONS as f64;
                hue = hue.sqrt() * 25.0;
                hsl = color::HSL {
                    h: hue,
                    s: 1.0,
                    l: 0.5,
                };
            }
            let rgb = hsl.to_rgba();
            data.push(rgb.0);
            data.push(rgb.1);
            data.push(rgb.2);
            data.push(rgb.3);
        }
    }

    data
}

fn get_iter_index(c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = Complex {
        real: 0.0,
        imaginary: 0.0,
    };
    while iter_index < MAX_ITERATIONS {
        if z.norm() > 4.0 {
            // when |z| is out of a circle with circ. 2 -> cant be in set
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

fn scale(x: f64, min1: f64, max1: f64, min2: f64, max2: f64) -> f64 {
    (x - min1) / (max1 - min1) * (max2 - min2) + min2
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
