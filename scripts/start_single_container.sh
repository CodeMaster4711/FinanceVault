#!/bin/bash
set -e

# Start Frontend (Node.js) in background
echo "Starting Frontend on port 3000..."
cd /app/frontend
# Assuming the build output is in 'build' and we use the node adapter
# We can run it with node build/index.js or similar depending on adapter-node output
# adapter-node usually outputs to build/ directory with an index.js or similar.
# Let's assume standard adapter-node output structure.
# We need to set PORT=3000 explicitly
export PORT=3000
export ORIGIN=${FRONTEND_URL:-http://localhost:8000}
node build/index.js &
FRONTEND_PID=$!

# Wait for frontend to be ready (optional but good)
echo "Waiting for Frontend to start..."
sleep 2

# Start Backend
echo "Starting Backend on port 8000..."
cd /app
# Ensure backend knows where to proxy to (localhost:3000)
export FRONTEND_PROXY_URL=http://localhost:3000
./backend &
BACKEND_PID=$!

# Handle shutdown
trap "kill $FRONTEND_PID $BACKEND_PID; exit" SIGINT SIGTERM

# Wait for processes
wait $BACKEND_PID $FRONTEND_PID
