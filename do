#!/bin/bash

# Set the target directory for release builds
TARGET_DIR="_temp/docker_image/solution"

# Set the name for the executable
EXECUTABLE_NAME="filler"

# Build the project in release mode
cargo build --release --manifest-path=filler/Cargo.toml

# Copy the executable to the specified location with the desired name
cp "filler/target/release/filler" "$TARGET_DIR/$EXECUTABLE_NAME"

# Print a message indicating successful build
echo "Build completed. Executable is in $TARGET_DIR/$EXECUTABLE_NAME"
