use crossterm::style::{Color, Colors};
use crate::colors::random_color;

/// Represents different lighting modes for ornaments
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightMode {
    /// Ornament blinks on and off
    Blinking,
    /// Ornament stays constantly on (for future implementation)
    Steady,
    /// Ornament fades in and out (for future implementation)
    Fading,
    /// Ornament pulses (for future implementation)
    Pulsing,
}

/// Represents a single ornament on the tree
#[derive(Debug, Clone)]
pub struct Ornament {
    pub x: u16,
    pub y: u16,
    pub color: Color,
    pub light_mode: LightMode,
    pub is_visible: bool,
    pub blink_counter: u32,
}

impl Ornament {
    /// Creates a new ornament at the specified position
    // TODO: ornament 위치에 대한 검증 추가
    pub fn new(x: u16, y: u16, color: Color, light_mode: LightMode) -> Self {
        Self {
            x,
            y,
            color,
            light_mode,
            is_visible: true,
            blink_counter: 0,
        }
    }

    /// Updates the ornament state (for animation)
    pub fn update(&mut self) {
        let blink_num_limit = 5;

        match self.light_mode {
            LightMode::Blinking => {
                self.blink_counter += 1;
                // Toggle visibility every 5 frames
                if self.blink_counter >= blink_num_limit {
                    self.color= random_color();
                    self.blink_counter = 0;
                }
            }
            LightMode::Steady => {
                // Always visible (future implementation)
                self.is_visible = true;
            }
            LightMode::Fading | LightMode::Pulsing => {
                // Placeholder for future implementation
                self.is_visible = true;
            }
        }
    }

    /// Returns the character to display for this ornament
    pub fn get_char(&self) -> char {
        if self.is_visible {
            'o'
        } else {
            ' '
        }
    }
}
