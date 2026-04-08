#!/bin/bash
# Quick start script for NetCtl

set -e

echo "🚀 Starting NetCtl Network Control Engine"
echo "=========================================="

# Check if build exists
if [ ! -f "backend/target/release/netctl-daemon" ]; then
    echo "Backend not built. Running build.sh first..."
    bash build.sh
fi

if [ ! -d "frontend/dist" ]; then
    echo "Frontend not built. Building..."
    cd frontend
    npm run build
    cd ..
fi

echo ""
echo "Starting backend daemon..."
cd backend

# Check if running with root
if [ "$EUID" -ne 0 ]; then
    echo "⚠ Backend requires root for network operations"
    echo "  Run with: sudo ./target/release/netctl-daemon"
    echo ""
    echo "Running with sudo..."
    sudo ./target/release/netctl-daemon &
    DAEMON_PID=$!
else
    ./target/release/netctl-daemon &
    DAEMON_PID=$!
fi

cd ..

sleep 2

echo "✓ Backend started (PID: $DAEMON_PID)"
echo "✓ API running at http://localhost:3001"

echo ""
echo "Starting frontend development server..."
cd frontend
npm run dev

echo ""
echo "🎉 NetCtl is ready!"
echo "   Dashboard: http://localhost:5173"
echo "   API:       http://localhost:3001"
