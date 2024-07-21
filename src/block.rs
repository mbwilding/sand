use coolor::{Color, Hsl};

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub character: char,
    pub color: Color,
}

impl Block {
    pub fn new() -> Block {
        let hue = rand::random::<f32>() * 360.0;
        let color = coolor::Color::Hsl(Hsl::new(hue, 0.8, 0.5));
        Block {
            character: '█',
            color,
        }
    }
}
