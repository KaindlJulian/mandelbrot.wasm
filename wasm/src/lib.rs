use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 0.0;
    let param_r = 0.0;
    let scale = 1.0;

    for x in 0..width {
        for y in 0..height {

            let real = map_pixel(x as i16, 0, width as i16, -2, 2);
            let imaginary = map_pixel(y as i16, 0, height as i16, -2, 2);
            let c = Complex { real, imaginary};
            let iter_index = get_iter_index(c);

            data.push((iter_index / 4) as u8);  //R
            data.push((iter_index / 2) as u8);  //G
            data.push(iter_index as u8);        //B
            data.push(255);                     //A
        }
    }

    data
}

fn get_iter_index(c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let real = 0.0;
    let imaginary = 0.0;
    let mut z = Complex { real, imaginary };
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

fn map_pixel(x: i16, min1: i16, max1: i16, min2: i16, max2: i16) -> f64 {
    let a = ((x as f64) - (min1 as f64)) / ((max1 as f64) - (min1 as f64)) * ((max2 as f64) - (min2 as f64)) + (min2 as f64);
    a
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