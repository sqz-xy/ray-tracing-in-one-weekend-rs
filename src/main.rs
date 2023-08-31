use bmp::{Pixel};

use canvas::Canvas;

pub mod canvas;

fn main() {
    let mut canvas = Canvas::new(256, 256);

    for i in 0..canvas.height {
        for j in 0..canvas.width {
            let r = i as f64 / (canvas.width as f64 - 1.0);
            let g = j as f64 / (canvas.height as f64 - 1.0);
            let b = 0.0;

            let ir = r * 255.999;
            let ig = g * 255.999;
            let ib = b * 255.999;
            
            //println!("{}, {}", r, g);
            canvas.set_pixel(i as f32, j as f32, Pixel::new(ir as u8, ig as u8, ib as u8));
        }
    }

    canvas.save("img.bmp");
}
