#!/bin/bash
# NetCtl Build Setup Script

set -e

echo "🚀 NetCtl Build Environment Setup"
echo "===================================="

# Check for Rust
echo ""
echo "Checking Rust installation..."
if command -v rustc &> /dev/null; then
    echo "✓ Rust is installed: $(rustc --version)"
else
    echo "✗ Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✓ Rust installed: $(rustc --version)"
fi

# Check for Node.js
echo ""
echo "Checking Node.js installation..."
if command -v node &> /dev/null; then
    echo "✓ Node.js is installed: $(node --version)"
else
    echo "✗ Node.js not found."
    echo " Install from https://nodejs.org/ or use:"
    echo " macOS: brew install node"
    echo " Linux: sudo apt-get install nodejs npm"
    exit 1
fi

# Check for LLVM (for eBPF compilation)
echo ""
echo "Checking LLVM installation..."
if command -v llvm-config &> /dev/null; then
    echo "✓ LLVM is installed: $(llvm-config --version)"
else
    echo "⚠ LLVM not found. eBPF compilation may fail."
    echo " Installation instructions:"
    echo " macOS: brew install llvm"
    echo " Linux: sudo apt-get install llvm llvm-dev"
fi

# Build backend
echo ""
echo "Building backend..."
cd backend
cargo build --release
echo "✓ Backend build complete"

# Install frontend dependencies
echo ""
echo "Installing frontend dependencies..."
cd ../frontend
npm install
echo "✓ Frontend dependencies installed"

# Build frontend
echo ""
echo "Building frontend..."
npm run build
echo "✓ Frontend build complete"

echo ""
echo "✅ NetCtl build completed successfully!"
echo ""
echo "Next steps:"
echo " 1. Run backend: sudo ./backend/target/release/netctl-daemon"
echo " 2. Run frontend: cd frontend && npm run dev"
echo " 3. Visit: http://localhost:5173"