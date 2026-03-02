#!/bin/bash

echo "🔨 Building Discord Security Bot..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Copying from .env.example..."
    cp .env.example .env
    echo "📝 Please edit .env with your configuration before running the bot"
    echo ""
fi

# Build the project
echo "🔧 Compiling project (this may take a few minutes)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "📦 Binary location: ./target/release/discord-security-bot"
    echo ""
    echo "To run the bot:"
    echo "  ./target/release/discord-security-bot"
    echo ""
    echo "Or use:"
    echo "  cargo run --release"
else
    echo ""
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi
