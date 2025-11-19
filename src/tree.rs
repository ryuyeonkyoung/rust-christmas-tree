use crate::ornament::{LightMode, Ornament};
use crossterm::style::Color;

/// Represents the Christmas tree with its ornaments
pub struct ChristmasTree {
    pub ornaments: Vec<Ornament>,
    pub width: u16,
    pub height: u16,
}

impl ChristmasTree {
    /// Creates a new Christmas tree with default ornaments
    pub fn new() -> Self {
        let mut ornaments = Vec::new();

        // Add some blinking ornaments at predefined positions
        // These positions correspond to the ASCII tree structure
        ornaments.push(Ornament::new(10, 2, Color::Red, LightMode::Blinking));
        ornaments.push(Ornament::new(8, 4, Color::Yellow, LightMode::Blinking));
        ornaments.push(Ornament::new(12, 4, Color::Blue, LightMode::Blinking));
        ornaments.push(Ornament::new(7, 6, Color::Magenta, LightMode::Blinking));
        ornaments.push(Ornament::new(13, 6, Color::Cyan, LightMode::Blinking));
        ornaments.push(Ornament::new(6, 8, Color::Green, LightMode::Blinking));
        ornaments.push(Ornament::new(14, 8, Color::Red, LightMode::Blinking));

        Self {
            ornaments,
            width: 20,
            height: 12,
        }
    }

    /// Gets the ASCII representation of the tree
    pub fn get_ascii_tree() -> Vec<String> {
        vec![
            "        *        ".to_string(),
            "       /o\\       ".to_string(),
            "      /o o\\      ".to_string(),
            "     /o   o\\     ".to_string(),
            "    /o  o  o\\    ".to_string(),
            "   /o   o   o\\   ".to_string(),
            "  /o  o   o  o\\  ".to_string(),
            " /o   o o o   o\\ ".to_string(),
            "      |||        ".to_string(),
            "      |||        ".to_string(),
        ]
    }

    /// Updates all ornaments (for animation)
    pub fn update(&mut self) {
        for ornament in &mut self.ornaments {
            ornament.update();
        }
    }

    /// Placeholder for adding custom ornaments (future implementation)
    pub fn add_ornament(&mut self, ornament: Ornament) {
        self.ornaments.push(ornament);
    }

    /// Placeholder for removing ornaments (future implementation)
    pub fn clear_ornaments(&mut self) {
        self.ornaments.clear();
    }
}

impl Default for ChristmasTree {
    fn default() -> Self {
        Self::new()
    }
}
