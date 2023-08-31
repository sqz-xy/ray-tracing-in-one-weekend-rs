use bmp::{Image, Pixel};

pub struct Canvas {
    pixels: Image,
    pub width: u32,
    pub height: u32
}

impl Canvas {
    pub fn new (p_width: u32, p_height: u32) -> Canvas {
        Canvas {
            pixels: Image::new(p_width, p_height),
            width: p_width,
            height: p_height
        }
    }

    // Colours are multiplied and divided by 255 to transform the colours from 0-1 scale
    // Sets pixel colour
    pub fn set_pixel(&mut self, p_x: f32, p_y: f32, p_colour: Pixel) {
        self.pixels.set_pixel(p_x as u32, p_y as u32, p_colour);
    }

    // Get pixel colour 
    pub fn get_pixel(&self, p_x: f32, p_y: f32) -> Pixel {
        return self.pixels.get_pixel(p_x as u32, p_y as u32);
    }

    // Saves the paper as bmp
    pub fn save(&self, p_file_name: &str) {
        let _ = self.pixels.save(p_file_name);
    }
}