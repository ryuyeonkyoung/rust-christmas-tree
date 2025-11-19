.PHONY: build run clean test install help

# Default target
all: build

# Build the project
build:
@echo "🔨 Building Christmas Tree..."
@cargo build

# Build release version
release:
@echo "🔨 Building release version..."
@cargo build --release

# Run the animation
run:
@echo "🎄 Starting Christmas Tree Animation..."
@cargo run

# Clean build artifacts
clean:
@echo "🧹 Cleaning build artifacts..."
@cargo clean

# Run tests (placeholder for future tests)
test:
@echo "🧪 Running tests..."
@cargo test

# Check code without building
check:
@echo "✅ Checking code..."
@cargo check

# Format code
fmt:
@echo "📝 Formatting code..."
@cargo fmt

# Lint code
lint:
@echo "🔍 Linting code..."
@cargo clippy -- -D warnings

# Install the binary
install:
@echo "📦 Installing Christmas Tree..."
@cargo install --path .

# Show help
help:
@echo "🎄 Rust Christmas Tree - Available Commands 🎄"
@echo ""
@echo "  make build    - Build the project"
@echo "  make release  - Build release version"
@echo "  make run      - Run the animation"
@echo "  make clean    - Clean build artifacts"
@echo "  make test     - Run tests"
@echo "  make check    - Check code without building"
@echo "  make fmt      - Format code with rustfmt"
@echo "  make lint     - Lint code with clippy"
@echo "  make install  - Install the binary"
@echo "  make help     - Show this help message"
