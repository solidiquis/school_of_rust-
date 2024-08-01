#![allow(dead_code)]

struct Col16 {
    code: u8,
}

impl Col16 {
    fn new(code: u8) -> Self {
        Self { code }
    }
}

struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

trait AnsiColor {
    fn sprint_color(&self, txt: &str) -> String;
}

impl AnsiColor for Rgb {
    fn sprint_color(&self, txt: &str) -> String {
        format!("\x1b[38;2;{};{};{}m{txt}", self.red, self.green, self.blue)
    }
}

impl AnsiColor for Col16 {
    fn sprint_color(&self, txt: &str) -> String {
        format!("\x1b[{}m{txt}", self.code)
    }
}

fn print_text_with_color(txt: &str, color: Box<dyn AnsiColor>) {
    let colorized_string = color.sprint_color(txt);
    println!("{colorized_string}");
}

fn main() {
    let rgb = Rgb::new(15, 200, 17);
    print_text_with_color("Hello World", Box::new(rgb));

    let col16 = Col16::new(31);
    print_text_with_color("Hello World", Box::new(col16));
}
