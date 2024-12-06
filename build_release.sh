#!/bin/bash

# This script automates the process of building and packaging a release for RenderKit
# It handles different platforms, generates binaries, and creates release artifacts

# Exit immediately if any command fails - this helps catch errors early
set -e

# Ensure we're in the project's root directory to avoid path-related issues
cd "$(dirname "$0")"

# Clean up any previous build artifacts to start with a fresh build
# This prevents potential conflicts from old build files
cargo clean

# Detect the current computer's platform
# This helps us create platform-specific binaries
PLATFORM=$(uname -m)      # e.g., x86_64, arm64
OS=$(uname -s | tr '[:upper:]' '[:lower:]')  # e.g., darwin, linux

# Create a directory to store our release artifacts
# This keeps our build output organized
mkdir -p target/release

# Inform the user which platform we're building for
echo "Building for current platform: $PLATFORM-$OS..."

# Build the release version of the project
# The --release flag enables full optimizations
cargo build --release

# Find the binary we just built
# This is tricky because different platforms have different ways of marking executables
if [[ "$OS" == "darwin" ]]; then
    # On macOS, we look for files that are executable or are libraries
    # The grep -v '\.so' filters out shared libraries we don't want
    BINARY_PATH=$(find target/release -type f \( -perm +111 -o -name "*.dylib" \) | grep -v '\.so' | head -n 1)
else
    # On other platforms, we use a different method to find executables
    BINARY_PATH=$(find target/release -type f -perm /111 | grep -v '\.so' | head -n 1)
fi

# Check if we actually found a binary
if [ -z "$BINARY_PATH" ]; then
    echo "Error: No executable binary found in target/release"
    echo "Contents of target/release:"
    ls -la target/release
    exit 1  # Stop the script with an error
fi

# Determine a meaningful name for our binary based on the platform
# This helps users quickly identify which binary to use
if [[ "$OS" == "darwin" ]]; then
    # macOS specific naming
    if [[ "$PLATFORM" == "x86_64" ]]; then
        binary_name="renderkit-macos-x64"
    elif [[ "$PLATFORM" == "arm64" ]]; then
        binary_name="renderkit-macos-arm64"
    else
        binary_name="renderkit-macos-unknown"
    fi
elif [[ "$OS" == "linux" ]]; then
    # Linux specific naming
    if [[ "$PLATFORM" == "x86_64" ]]; then
        binary_name="renderkit-linux-x64"
    elif [[ "$PLATFORM" == "aarch64" ]]; then
        binary_name="renderkit-linux-arm64"
    else
        binary_name="renderkit-linux-unknown"
    fi
elif [[ "$OS" == "mingw"* || "$OS" == "msys"* || "$OS" == "windows"* ]]; then
    # Windows specific naming
    binary_name="renderkit-windows-x64.exe"
else
    binary_name="renderkit-unknown"
fi

# Sometimes we might find a library instead of an executable
# This block handles that case, especially on macOS
if [[ "$OS" == "darwin" ]]; then
    if [[ "$BINARY_PATH" == *".dylib" ]]; then
        echo "Found library instead of executable. Attempting to create executable."
        # Try to build a specific binary target
        cargo build --release --bin renderkit
        BINARY_PATH="target/release/renderkit"
    fi
fi

# Double-check that the binary actually exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

# Copy the binary to the release directory with our platform-specific name
cp "$BINARY_PATH" "target/release/$binary_name"

# Create a compressed tarball of the binary
# This makes it easy to distribute and download
tar -czvf "target/release/$binary_name.tar.gz" -C "target/release" "$binary_name"

# Generate cryptographic checksums
# This helps users verify the integrity of the download
cd target/release
shasum -a 256 "$binary_name" "$binary_name.tar.gz" > checksums.txt

# Print out success message with details
echo "Release build completed successfully!"
echo "Binary path: $BINARY_PATH"
echo "Binary name: $binary_name"
echo "Tarball: $binary_name.tar.gz"
echo "Checksums generated in checksums.txt"

# List the contents of the release directory for verification
ls -l target/release
