#!/bin/bash

# Exit on error
set -e

# Check for required tools
echo "--- Checking Required Tools ---"
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v rustup &> /dev/null; then
    echo "Error: rustup is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "--- Installing Build Dependencies ---"
# Install cargo-deb and cargo-packager if missing
if ! command -v cargo-deb &> /dev/null; then
    echo "Installing cargo-deb..."
    cargo install cargo-deb
fi

if ! command -v cargo-packager &> /dev/null; then
    echo "Installing cargo-packager..."
    cargo install cargo-packager
fi

echo "--- Building .deb Package ---"
cargo deb

# Check for Windows cross-compilation requirements
echo "--- Checking Windows Build Requirements ---"
WINDOWS_BUILD_AVAILABLE=true
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo "Adding Windows toolchain..."
    rustup target add x86_64-pc-windows-gnu
fi

if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Warning: mingw-w64 is not installed. Skipping Windows build."
    echo "To build Windows packages, install mingw-w64 using your package manager."
    echo "Example: sudo apt install mingw-w64"
    WINDOWS_BUILD_AVAILABLE=false
fi

# You need makensis to package NSIS installers on Linux, Tony.
if ! command -v makensis &> /dev/null; then
    echo "Warning: makensis is not installed. Skipping Windows packaging."
    echo "Run: sudo apt install nsis"
    WINDOWS_BUILD_AVAILABLE=false
fi

if [ "$WINDOWS_BUILD_AVAILABLE" = true ]; then
    echo "--- Building Windows Binary ---"
    # Actually telling Cargo to use the right linker so it doesn't fail miserably
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc cargo build --release --target x86_64-pc-windows-gnu
    
    echo "--- Packaging Windows .exe (NSIS) ---"
    cargo packager --release --target x86_64-pc-windows-gnu --formats nsis
    echo "Windows package completed."
else
    echo "Windows build skipped due to your missing dependencies."
fi

# Check for MacOS cross-compilation requirements
echo "--- Checking MacOS Build Requirements ---"
MAC_BUILD_AVAILABLE=true
if ! rustup target list | grep -q "x86_64-apple-darwin (installed)"; then
    echo "Adding MacOS toolchain..."
    rustup target add x86_64-apple-darwin || true
fi

if ! command -v osxcross-macports &> /dev/null && ! command -v o64-clang &> /dev/null; then
    echo "Warning: osxcross is not installed or available in PATH."
    echo "Cross-compiling to MacOS from Linux requires the macOS SDK and osxcross."
    echo "Skipping MacOS build locally. (If you run this script on a Mac, we could use native cargo build)."
    MAC_BUILD_AVAILABLE=false
fi

if [ "$MAC_BUILD_AVAILABLE" = true ]; then
    echo "--- Building MacOS Binary ---"
    cargo build --release --target x86_64-apple-darwin
    
    echo "--- Packaging MacOS Application ---"
    cargo packager --release --target x86_64-apple-darwin --formats app,dmg
    echo "MacOS package completed."
else
    echo "MacOS build skipped due to missing cross-compilation dependencies (osxcross)."
fi

echo "--- Build Complete ---"
echo "Linux package: target/debian/"
if [ "$WINDOWS_BUILD_AVAILABLE" = true ]; then
    # Pointing to the ACTUAL directory where cargo packager put the file
    echo "Windows package: target/x86_64-pc-windows-gnu/release/"
fi
if [ "$MAC_BUILD_AVAILABLE" = true ]; then
    echo "MacOS package: target/x86_64-apple-darwin/release/"
fi