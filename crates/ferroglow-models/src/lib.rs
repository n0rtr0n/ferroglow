use serde::Serialize;

#[derive(Serialize)]
pub struct Pattern {
    name: String,
}


impl Pattern {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize)]
pub struct Pixel {
    x: u16,
    y: u16,
    color: Color,
    universe: u16,
    channel_position: u16,
}

impl Pixel {
    pub fn new(
        x:u16, 
        y:u16, 
        color: Color, 
        universe: u16,
        channel_position: u16,
    ) -> Self {
        Self { x, y, color, universe, channel_position }
    }
}

#[derive(Serialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
}

#[derive(Serialize)]
pub struct PixelMap {
    pixels: Vec<Pixel>,
}

impl PixelMap {
    pub fn new(pixels: Vec<Pixel>) -> Self {
        Self { pixels }
    }
}