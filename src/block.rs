use coolor::{Color, Hsl};

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub character: char,
    pub color: Color,
}

impl Block {
    pub fn new() -> Block {
        let hue = rand::random::<f32>() * 360.0;
        let saturation = 0.8;
        let lightness = 0.5;
        let color = coolor::Color::Hsl(Hsl::new(hue, saturation, lightness));

        let character = rand::random::<u8>() % 52;
        let character = if character < 26 {
            (character + b'a') as char
        } else {
            (character - 26 + b'A') as char
        };

        Block { character, color }
    }
}
