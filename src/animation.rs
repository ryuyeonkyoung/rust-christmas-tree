use crate::tree::ChristmasTree;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

/// Main animation loop handler
pub struct AnimationLoop {
    pub tree: ChristmasTree,
    pub running: bool,
    pub frame_count: u64,
}

impl AnimationLoop {
    /// Creates a new animation loop
    pub fn new() -> Self {
        Self {
            tree: ChristmasTree::new(),
            running: false,
            frame_count: 0,
        }
    }

    /// Initializes the terminal for animation
    pub fn init_terminal(&self) -> io::Result<()> {
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    /// Cleans up the terminal after animation
    pub fn cleanup_terminal(&self) -> io::Result<()> {
        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Main animation loop - runs the input/update/render cycle
    pub fn run(&mut self) -> io::Result<()> {
        self.init_terminal()?;
        self.running = true;

        while self.running {
            // Input phase
            self.handle_input()?;

            // Update phase
            self.update();

            // Render phase
            self.render()?;

            // Frame timing (approximately 10 FPS)
            std::thread::sleep(Duration::from_millis(100));
            self.frame_count += 1;
        }

        self.cleanup_terminal()?;
        Ok(())
    }

    /// Handle user input (for future implementation: add interactivity)
    pub fn handle_input(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.running = false;
                    }
                    // Placeholder for future input handling
                    KeyCode::Char('r') => {
                        // Future: randomize colors
                    }
                    KeyCode::Char('s') => {
                        // Future: change speed
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Update the animation state
    pub fn update(&mut self) {
        self.tree.update();
    }

    /// Render the tree to the terminal
    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Move cursor to top
        execute!(stdout, cursor::MoveTo(0, 0))?;

        // Get the tree ASCII art
        let tree_lines = ChristmasTree::get_ascii_tree();

        // Render each line
        for (y, line) in tree_lines.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, y as u16))?;

            // Render the line with ornaments
            for (x, ch) in line.chars().enumerate() {
                let x_pos = x as u16;
                let y_pos = y as u16;

                // Check if there's an ornament at this position
                let ornament = self
                    .tree
                    .ornaments
                    .iter()
                    .find(|o| o.x == x_pos && o.y == y_pos);

                if let Some(orn) = ornament {
                    if orn.is_visible {
                        execute!(
                            stdout,
                            SetForegroundColor(orn.color),
                            Print(orn.get_char()),
                            ResetColor
                        )?;
                    } else {
                        execute!(stdout, Print(' '))?;
                    }
                } else {
                    // Render tree structure with color
                    let color = match ch {
                        '*' => Color::Yellow,
                        '/' | '\\' => Color::Green,
                        '|' => Color::DarkRed,
                        'o' => Color::Red,
                        _ => Color::White,
                    };
                    execute!(stdout, SetForegroundColor(color), Print(ch), ResetColor)?;
                }
            }
        }

        // Add instructions at the bottom
        execute!(
            stdout,
            cursor::MoveTo(0, (tree_lines.len() + 2) as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("Press 'q' or ESC to quit"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }
}

impl Default for AnimationLoop {
    fn default() -> Self {
        Self::new()
    }
}
