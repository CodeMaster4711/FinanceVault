#!/bin/bash
set -e

echo "🚀 Starting FinanceVault..."

# Ensure environment variables are set
export RUST_LOG=${RUST_LOG:-info}
export DATABASE_URL=${DATABASE_URL:-sqlite:/data/finance.db}

# Check if database directory exists
mkdir -p /data
echo "📁 Data directory: /data"
echo "💾 Database URL: $DATABASE_URL"
echo "🔧 Rust Log Level: $RUST_LOG"

# Create a log file for backend output
BACKEND_LOG=/tmp/backend.log
FRONTEND_LOG=/tmp/frontend.log

# Start backend in background with explicit logging
echo "📦 Starting Backend on port 8000..."
cd /app

# Start backend and redirect to log file
RUST_LOG=$RUST_LOG DATABASE_URL=$DATABASE_URL ./backend > $BACKEND_LOG 2>&1 &
BACKEND_PID=$!

echo "   Backend PID: $BACKEND_PID"

# Tail backend logs in background
tail -f $BACKEND_LOG 2>/dev/null | sed 's/^/[BACKEND] /' &
BACKEND_LOG_PID=$!

# Wait for backend to initialize and check if it's running
echo "⏳ Waiting for backend to initialize..."
sleep 5

if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "❌ Backend failed to start!"
    echo "📋 Backend logs:"
    cat $BACKEND_LOG
    exit 1
fi

echo "✅ Backend process is running (PID: $BACKEND_PID)"

# Check if backend is responding
MAX_RETRIES=10
RETRY=0
while [ $RETRY -lt $MAX_RETRIES ]; do
    if curl -f http://localhost:8000/ 2>/dev/null > /dev/null; then
        echo "✅ Backend health check passed"
        break
    fi
    RETRY=$((RETRY+1))
    if [ $RETRY -lt $MAX_RETRIES ]; then
        echo "⏳ Backend not ready yet, retrying ($RETRY/$MAX_RETRIES)..."
        sleep 2
    else
        echo "⚠️  Backend health check failed after $MAX_RETRIES attempts"
        echo "📋 Backend logs:"
        cat $BACKEND_LOG
    fi
done

# Start frontend (SvelteKit with adapter-node)
echo "🎨 Starting Frontend on port 3000..."
cd /app/frontend
PORT=3000 HOST=0.0.0.0 node index.js > $FRONTEND_LOG 2>&1 &
FRONTEND_PID=$!

echo "   Frontend PID: $FRONTEND_PID"

# Tail frontend logs in background
tail -f $FRONTEND_LOG 2>/dev/null | sed 's/^/[FRONTEND] /' &
FRONTEND_LOG_PID=$!

# Function to handle shutdown
shutdown() {
    echo "🛑 Shutting down services..."
    kill $BACKEND_PID $FRONTEND_PID $BACKEND_LOG_PID $FRONTEND_LOG_PID 2>/dev/null || true
    wait $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
    echo "✅ Services stopped"
    exit 0
}

# Trap signals
trap shutdown SIGTERM SIGINT

# Monitor both processes
while true; do
    if ! kill -0 $BACKEND_PID 2>/dev/null; then
        echo "❌ Backend process died!"
        echo "📋 Last backend logs:"
        tail -20 $BACKEND_LOG
        exit 1
    fi
    if ! kill -0 $FRONTEND_PID 2>/dev/null; then
        echo "❌ Frontend process died!"
        echo "📋 Last frontend logs:"
        tail -20 $FRONTEND_LOG
        exit 1
    fi
    sleep 10
done
