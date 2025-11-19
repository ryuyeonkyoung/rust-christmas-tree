#!/bin/bash
# Quick run script for Christmas Tree Animation

echo "🎄 Rust Christmas Tree Animation 🎄"
echo ""
echo "Building project..."
cargo build --quiet

if [ $? -eq 0 ]; then
    echo "Starting animation..."
    echo "Press 'q' or ESC to exit"
    echo ""
    cargo run --quiet
else
    echo "❌ Build failed!"
    exit 1
fi
