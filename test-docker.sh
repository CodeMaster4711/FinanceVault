#!/bin/bash

echo "🧪 Testing FinanceVault Docker Container..."

# Stop and remove old container
echo "🧹 Cleaning up old containers..."
docker stop financevault-test 2>/dev/null || true
docker rm financevault-test 2>/dev/null || true

# Build new image
echo "🏗️  Building Docker image..."
docker build -t financevault:test .

if [ $? -ne 0 ]; then
    echo "❌ Docker build failed!"
    exit 1
fi

echo "✅ Docker build successful!"

# Run container
echo "🚀 Starting container..."
docker run -d \
    -p 8000:8000 \
    -p 3000:3000 \
    -v financevault_data:/data \
    -e JWT_SECRET=$(openssl rand -hex 32) \
    -e RUST_LOG=debug \
    --name financevault-test \
    financevault:test

echo "⏳ Waiting 10 seconds for services to start..."
sleep 10

# Show logs
echo ""
echo "📋 Container Logs:"
echo "=================="
docker logs financevault-test

echo ""
echo "🔍 Testing Backend Health..."
if curl -f http://localhost:8000/ 2>/dev/null; then
    echo "✅ Backend is responding!"
    curl -s http://localhost:8000/ | jq '.' || echo "Response received but not JSON"
else
    echo "❌ Backend is not responding!"
fi

echo ""
echo "🔍 Testing Frontend Health..."
if curl -f http://localhost:3000/ 2>/dev/null > /dev/null; then
    echo "✅ Frontend is responding!"
else
    echo "❌ Frontend is not responding!"
fi

echo ""
echo "📊 Container Status:"
docker ps -a | grep financevault-test

echo ""
echo "💡 Commands:"
echo "  View logs:    docker logs -f financevault-test"
echo "  Stop:         docker stop financevault-test"
echo "  Remove:       docker rm financevault-test"
echo "  Shell access: docker exec -it financevault-test /bin/bash"
