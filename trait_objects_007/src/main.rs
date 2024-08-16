use std::ops::Deref;

struct Color {
    metadata: String,
    col: Rgb,
}

impl Deref for Color {
    type Target = Rgb;

    fn deref(&self) -> &Self::Target {
        &self.col
    }
}

#[derive(Debug)]
struct Rgb(u8, u8, u8);

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    fn red(&self) -> u8 {
        self.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color = Color {
        metadata: "This is a color metadata".to_string(),
        col: Rgb::new(255, 255, 255),
    };

    println!("{}", color.red());

    Ok(())
}
