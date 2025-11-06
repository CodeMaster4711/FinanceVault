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
echo ""

# Start backend in background with logging to stdout/stderr
echo "📦 Starting Backend on port 8000..."
cd /app

# Start backend with logs going to stdout (with prefix)
(RUST_LOG=$RUST_LOG DATABASE_URL=$DATABASE_URL ./backend 2>&1 | sed 's/^/[BACKEND] /') &
BACKEND_PID=$!

echo "   Backend PID: $BACKEND_PID"
echo ""

# Wait for backend to initialize
echo "⏳ Waiting for backend to initialize..."
sleep 5

if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "❌ Backend failed to start!"
    exit 1
fi

echo "✅ Backend process is running (PID: $BACKEND_PID)"
echo ""

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
    fi
done

echo ""

# Start frontend (SvelteKit with adapter-node)
echo "🎨 Starting Frontend on port 3000..."
cd /app/frontend

# Start frontend with logs going to stdout (with prefix) and verbose output
(PORT=3000 HOST=0.0.0.0 DEBUG=* node index.js 2>&1 | while IFS= read -r line; do
    # Filter out empty lines and add prefix
    if [ ! -z "$line" ]; then
        echo "[FRONTEND] $line"
    fi
done) &
FRONTEND_PID=$!

echo "   Frontend PID: $FRONTEND_PID"
echo ""

# Wait for frontend to start
sleep 3

# Check if frontend is responding
if curl -f http://localhost:3000/ 2>/dev/null > /dev/null; then
    echo "✅ Frontend health check passed"
else
    echo "⚠️  Frontend starting (may take a moment)"
fi

echo ""
echo "✅ Both services started successfully!"
echo "   🌐 Frontend: http://localhost:3000"
echo "   🔧 Backend:  http://localhost:8000"
echo "   📚 API Docs: http://localhost:8000/swagger-ui"
echo ""

# Function to handle shutdown
shutdown() {
    echo ""
    echo "🛑 Shutting down services..."
    kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
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
        exit 1
    fi
    if ! kill -0 $FRONTEND_PID 2>/dev/null; then
        echo "❌ Frontend process died!"
        exit 1
    fi
    sleep 10
done
