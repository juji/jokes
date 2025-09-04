#!/bin/bash

# Build the Docker image
echo "Building Docker image..."
docker build -t actix-api .

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Docker image built successfully!"
    echo ""
    echo "To run the container:"
    echo "  docker run -p 8080:8080 actix-api"
    echo ""
    echo "Or using docker-compose:"
    echo "  docker-compose up"
    echo ""
    echo "Or with custom port:"
    echo "  docker run -p 3000:8080 -e PORT=8080 actix-api"
else
    echo "❌ Docker build failed!"
    exit 1
fi