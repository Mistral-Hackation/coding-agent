#!/bin/bash
set -e

# Define image name
IMAGE_NAME="ubuntu25-builder"

echo "Building Docker image..."
# Use the same base as Dockerfile.test but geared for building
# We can actually just use Dockerfile.test as it has all dependencies and Rust
docker build -t $IMAGE_NAME -f Dockerfile.test .

echo "Running build container..."
# Run container, mount current directory to /app
# We override the CMD to build release
docker run --rm \
  -v "$(pwd):/app" \
  -w /app \
  $IMAGE_NAME \
  bash -c "cargo build --release"

echo "Build complete. Binary is at target/release/build123d_cad"

# Copy to Downloads
cp target/release/build123d_cad ~/Downloads/
echo "Copied binary to ~/Downloads/build123d_cad"
