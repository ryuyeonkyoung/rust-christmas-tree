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
        let x = 0; // TODO: ornament를 상대 경로로 변경 (트리를 옮기면 따라가도록)
        let y = 0;

        // Add some blinking ornaments at predefined positions
        // These positions correspond to the ASCII tree structure
        ornaments.push(Ornament::new(x + 7, y + 3, Color::Red, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 6, y + 4, Color::Yellow, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 9, y + 4, Color::Blue, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 7, y + 6, Color::Magenta, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 13, y + 6, Color::Cyan, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 6, y + 7, Color::Green, LightMode::Blinking));
        ornaments.push(Ornament::new(x + 14, y + 7, Color::Red, LightMode::Blinking));

        Self {
            ornaments,
            width: 20,
            height: 12,
        }
    }

    /// Gets the ASCII representation of the tree
    pub fn get_ascii_tree() -> Vec<String> {
        vec![
            "                        *                        ".to_string(),
            "                       ***                       ".to_string(),
            "                      *****                      ".to_string(),
            "                     *******                     ".to_string(),
            "                    *********                    ".to_string(),
            "                   ***********                   ".to_string(),
            "                  *************                  ".to_string(),
            "                 ***************                 ".to_string(),
            "                *****************                ".to_string(),
            "               *******************               ".to_string(),
            "              *********************              ".to_string(),
            "             ***********************             ".to_string(),
            "            *************************            ".to_string(),
            "           ***************************           ".to_string(),
            "          *****************************          ".to_string(),
            "         *******************************         ".to_string(),
            "        *********************************        ".to_string(),
            "       ***********************************       ".to_string(),
            "      *************************************      ".to_string(),
            "     ***************************************     ".to_string(),
            "    *****************************************    ".to_string(),
            "   *******************************************   ".to_string(),
            "  *********************************************  ".to_string(),
            " *********************************************** ".to_string(),
            "                     |||||                       ".to_string(),
            "                     |||||                       ".to_string(),
            "                     |||||                       ".to_string(),
        ]
    }

    /// Returns list of valid ornament positions inside the canopy.
    /// Currently selects positions where the character is `'*'`.
    fn canopy_positions(tree_lines: &Vec<String>) -> Vec<(u16, u16)> {
        let mut positions = Vec::new();
        for (y, line) in tree_lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '*' {
                    positions.push((x as u16, y as u16));
                }
            }
        }
        positions
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
