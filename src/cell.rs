use coolor::{Color, Hsl};

/// The cell struct
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub glyph: char,
    pub color: Color,
}

impl Cell {
    /// Creates a new cell
    pub fn new(lower: bool, upper: bool, number: bool, symbol: bool) -> Cell {
        let hue = rand::random::<f32>() * 360.0;
        let saturation = 0.8;
        let lightness = 0.5;
        let color = coolor::Color::Hsl(Hsl::new(hue, saturation, lightness));

        let character = if lower && upper && number && symbol {
            let choice = rand::random::<u8>() % 4;
            match choice {
                0 => (rand::random::<u8>() % 26 + b'a') as char,
                1 => (rand::random::<u8>() % 26 + b'A') as char,
                2 => (rand::random::<u8>() % 10 + b'0') as char,
                _ => {
                    let symbols = b"!@#$%^&*()_+-=[]{}|;:',.<>?/";
                    symbols[rand::random::<usize>() % symbols.len()] as char
                }
            }
        } else if lower && upper && number {
            let choice = rand::random::<u8>() % 3;
            match choice {
                0 => (rand::random::<u8>() % 26 + b'a') as char,
                1 => (rand::random::<u8>() % 26 + b'A') as char,
                _ => (rand::random::<u8>() % 10 + b'0') as char,
            }
        } else if lower && upper {
            let choice = rand::random::<u8>() % 2;
            match choice {
                0 => (rand::random::<u8>() % 26 + b'a') as char,
                _ => (rand::random::<u8>() % 26 + b'A') as char,
            }
        } else if lower {
            (rand::random::<u8>() % 26 + b'a') as char
        } else if upper {
            (rand::random::<u8>() % 26 + b'A') as char
        } else if number {
            (rand::random::<u8>() % 10 + b'0') as char
        } else if symbol {
            let symbols = b"!@#$%^&*()_+-=[]{}|;:',.<>?/`~\"\\";
            symbols[rand::random::<usize>() % symbols.len()] as char
        } else {
            '�'
        };

        Cell {
            glyph: character,
            color,
        }
    }
}
