# 🎄 Rust Christmas Tree Animation

A terminal-based Christmas tree animation written in Rust with blinking ornaments and an extensible framework for future enhancements.

## ✨ Features

- **ASCII Christmas Tree**: Beautiful terminal-rendered tree
- **Blinking Ornaments**: Colorful ornaments that blink on and off
- **Modular Architecture**: Clean separation of concerns with extensible framework
- **Color Support**: Multi-colored ornaments using crossterm
- **Interactive Controls**: Press 'q' or ESC to exit

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/ryuyeonkyoung/rust-christmas-tree.git
cd rust-christmas-tree

# Build the project
cargo build

# Run the animation
cargo run
```

### Using Makefile

```bash
# Build
make build

# Run
make run

# Clean
make clean

# Format code
make fmt

# Lint code
make lint

# Show all available commands
make help
```

## 🏗️ Project Structure

```
src/
├── main.rs          # Entry point
├── animation.rs     # Animation loop with input/update/render cycle
├── tree.rs          # Christmas tree structure and ASCII art
├── ornament.rs      # Ornament struct and LightMode enum
└── colors.rs        # Color utility functions
```

## 🎨 Extensible Framework

The project is designed with extensibility in mind. Here are the key components ready for future implementation:

### LightMode Enum

```rust
pub enum LightMode {
    Blinking,  // ✅ Implemented
    Steady,    // 🚧 Ready for implementation
    Fading,    // 🚧 Ready for implementation
    Pulsing,   // 🚧 Ready for implementation
}
```

### Ornament Structure

```rust
pub struct Ornament {
    pub x: u16,
    pub y: u16,
    pub color: Color,
    pub light_mode: LightMode,
    pub is_visible: bool,
    pub blink_counter: u32,
}
```

### Animation Loop (Input/Update/Render)

- **Input Phase**: Handle user keyboard input
- **Update Phase**: Update ornament states
- **Render Phase**: Draw the tree to terminal

### Color Utilities

- `random_color()` - Get a random color
- `random_warm_color()` - Get warm colors (red, yellow, magenta)
- `random_cool_color()` - Get cool colors (blue, cyan, green)
- `rgb_to_color()` - Convert RGB values to Color

## 🎮 Controls

- **q** or **ESC**: Quit the animation
- **r**: (Placeholder) Randomize colors
- **s**: (Placeholder) Change animation speed

## 🔧 Development

### Build for Release

```bash
cargo build --release
# Binary will be in target/release/christmas-tree
```

### Run Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## 📝 Future Enhancements

The framework is ready for these enhancements:

- [ ] Multiple light modes (steady, fading, pulsing)
- [ ] Interactive color changing
- [ ] Adjustable animation speed
- [ ] Custom tree sizes
- [ ] Snow effect
- [ ] Music/sound effects
- [ ] Different tree styles
- [ ] Save/load configurations

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📄 License

This project is open source and available under the MIT License.

## 🎅 Happy Holidays!

Enjoy your animated Christmas tree! 🎄✨