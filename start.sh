#!/bin/bash
# NetCtl Hot-Swap Start Script
# Backend state is preserved on rebuild

set -e

DAEMON_PID=""
FRONTEND_PID=""

PROD_MODE=false

for arg in "$@"; do
    case $arg in
        --prod)
            PROD_MODE=true
            shift
            ;;
    esac
done

echo "🚀 Starting NetCtl Network Control Engine (Hot-Swap Enabled)"
echo "============================================================"

# Ensure dependencies
command -v chokidar >/dev/null 2>&1 || npm install -g chokidar-cli
command -v concurrently >/dev/null 2>&1 || npm install -g concurrently

# Function: start backend if not running
start_backend() {
    if [ -n "$DAEMON_PID" ] && ps -p $DAEMON_PID > /dev/null; then
        echo "Backend already running (PID: $DAEMON_PID), sending SIGHUP for hot-reload..."
        kill -HUP $DAEMON_PID
    else
        cd backend
        echo "⏳ Building backend..."
        cargo build
        echo "✓ Backend build complete"

        echo "Starting backend daemon..."
        if [ "$EUID" -ne 0 ]; then
            sudo ./target/debug/netctl-daemon &
        else
            ./target/debug/netctl-daemon &
        fi
        DAEMON_PID=$!
        cd ..
        echo "✓ Backend started (PID: $DAEMON_PID)"
    fi
}

# Function: start frontend dev server
start_frontend() {
    cd frontend
    if [ ! -d "node_modules" ]; then
        echo "Installing frontend dependencies..."
        npm install --silent
    fi
    echo "Starting frontend dev server..."
    npm run dev &
    FRONTEND_PID=$!
    cd ..
}

# Initial start
start_backend
start_frontend

# Watch backend for source changes
npx chokidar "backend/src/**/*" -c "echo 'Backend source changed. Triggering hot-reload...'; bash -c 'start_backend'" &

# Watch frontend for source changes
npx chokidar "frontend/src/**/*" -c "echo 'Frontend source changed. Rebuilding...'; cd frontend && npm run build" &

# Wait for processes
wait