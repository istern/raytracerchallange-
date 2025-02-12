
use crate::color::Color;

#[derive(Debug, PartialEq)]
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