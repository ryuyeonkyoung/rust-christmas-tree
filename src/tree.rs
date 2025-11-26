use crate::ornament::{LightMode, Ornament};
use crate::colors::random_color;
use rand::seq::SliceRandom;

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

        // Get ASCII art and compute dimensions
        let tree_lines = Self::get_ascii_tree();
        let height = tree_lines.len() as u16;
        let width = tree_lines
            .iter()
            .map(|l| l.len())
            .max()
            .unwrap_or(0) as u16;

        // Collect valid positions inside the canopy (here: '*' positions)
        let mut positions = Self::canopy_positions(&tree_lines);

        // Shuffle and pick some positions without duplicates
        let mut rng = rand::thread_rng();
        positions.shuffle(&mut rng);

        // 원하는 오너먼트 수 (안정적으로 캔버스 크기 내에서만)
        let desired = 8usize.min(positions.len());
        for (x, y) in positions.into_iter().take(desired) {
            ornaments.push(Ornament::new(x, y, random_color(), LightMode::Blinking));
        }

        Self {
            ornaments,
            width,
            height,
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
