
use crate::color::Color;

#[derive(Debug, PartialEq, Default)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>, // A 2D grid of colors
}


impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let color = Color{
            red:0.0,
            green:0.0,
            blue:0.0
        };
        Self {
            width,
            height,
            pixels: vec![vec![color; width]; height], // Default: Black color
        }
    }
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    if x < canvas.width && y < canvas.height {
        canvas.pixels[y][x] = color;
    }
}


pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut ppm: String = String::new();

    // PPM Header
    ppm.push_str("P3\n");
    ppm.push_str(&format!("{} {}\n", canvas.width, canvas.height));
    ppm.push_str("255\n");

   
    for row in &canvas.pixels {
        let mut line = String::new();

        for color in row {
            let r = (color.red * 255.0).round().clamp(0.0, 255.0) as u8;
            let g = (color.green * 255.0).round().clamp(0.0, 255.0) as u8;
            let b = (color.blue * 255.0).round().clamp(0.0, 255.0) as u8;

            let pixel_data = format!("{} {} {} ", r, g, b);

            // Ensure lines in the PPM file do not exceed 70 characters
            if line.len() + pixel_data.len() > 70 {
                ppm.push_str(line.trim_end());
                ppm.push('\n');
                line.clear();
            }

            line.push_str(&pixel_data);
        }

        if !line.is_empty() {
            ppm.push_str(line.trim_end());
            ppm.push('\n');
        }
    }

    ppm
}
