#!/bin/bash
set -e

echo "🚀 Starting FinanceVault..."

# Start backend in background
echo "📦 Starting Backend on port 8000..."
cd /app
./backend &
BACKEND_PID=$!

# Wait a moment for backend to initialize
sleep 3

# Start frontend
echo "🎨 Starting Frontend on port 3000..."
cd /app/frontend
PORT=3000 node index.js &
FRONTEND_PID=$!

echo "✅ Both services started successfully!"
echo "   Backend PID: $BACKEND_PID"
echo "   Frontend PID: $FRONTEND_PID"

# Function to handle shutdown
shutdown() {
    echo "🛑 Shutting down services..."
    kill $BACKEND_PID $FRONTEND_PID 2>/dev/null
    wait $BACKEND_PID $FRONTEND_PID 2>/dev/null
    echo "✅ Services stopped"
    exit 0
}

# Trap signals
trap shutdown SIGTERM SIGINT

# Wait for both processes
wait $BACKEND_PID $FRONTEND_PID
