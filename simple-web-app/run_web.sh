#!/bin/bash
# Script to reliably start the Vite development server

cd "$(dirname "$0")/web"

# Stop any existing vite processes
pkill -f "vite"

# Start Vite in a detached session
# setsid: detach from current terminal
# nohup: ignore hangup signals
# --host: expose to network (fixes some localhost binding issues)
echo "Starting Vite server..."
setsid nohup ./node_modules/.bin/vite --host > vite.log 2>&1 &

PID=$!
echo "Vite server started with PID: $PID"
echo "Logs are being written to: web/vite.log"
echo "Access at: http://localhost:5173"
