use crossterm::style::Color;
use rand::Rng;

/// Returns a brown color (RGB)
pub fn brown() -> Color {
    // RGB for a typical "brown" (you can adjust)
    Color::Rgb { r: 165, g: 42, b: 42 }
}

/// Utility functions for color management

/// Returns a random color from a predefined palette
pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let colors = [
        Color::Red,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    colors[rng.gen_range(0..colors.len())]
}

/// Returns a random warm color (for future implementation)
pub fn random_warm_color() -> Color {
    let mut rng = rand::thread_rng();
    let colors = [Color::Red, Color::Yellow, Color::Magenta];
    colors[rng.gen_range(0..colors.len())]
}

/// Returns a random cool color (for future implementation)
pub fn random_cool_color() -> Color {
    let mut rng = rand::thread_rng();
    let colors = [Color::Blue, Color::Cyan, Color::Green];
    colors[rng.gen_range(0..colors.len())]
}

/// Converts RGB to Color (for future implementation)
pub fn rgb_to_color(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}
