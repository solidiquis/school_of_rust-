#![allow(dead_code)]

// Rust enum is the same as an ADT, which is a Algebraic Date-type.
// Implemented as a tagged C-Union
// This enum color has two variants: Rgb, Col16
enum Color {
    Rgb(u8, u8, u8),
    Col16(u8),
}

impl Color {
    // Associated method
    fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::Rgb(red, green, blue)
    }

    fn new_col16(code: u8) -> Self {
        match code {
            30..=37 => Self::Col16(code),
            _ => panic!("not valid code"),
        }
    }

    // method.. technically also an associated method
    fn print_colorized(&self, message: &str) {
        let color = match self {
            Self::Col16(code) => format!("{code}"),
            Self::Rgb(red, green, blue) => format!("{red}, {green}, {blue}")
        };

        println!("{message} -> {color}");
    }
}

fn main() {
    let color = Color::new_rgb(100, 100, 100);

    // Syntactic sugar
    color.print_colorized("Hello World");

    // Equivalent to above
    Color::print_colorized(&color, "Hello World");
}
