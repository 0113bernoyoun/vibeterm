#!/bin/bash
# Verification script for tokio runtime fix

set -e

echo "=========================================="
echo "VibeTerm Tokio Runtime Fix Verification"
echo "=========================================="
echo

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ ERROR: cargo not found. Please install Rust toolchain."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

echo "✅ Cargo found: $(cargo --version)"
echo

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Not in project root. Please run from vibeterm directory."
    exit 1
fi

echo "✅ Project root detected"
echo

# Check for the critical changes
echo "Checking code changes..."

if grep -q "use std::sync::Arc;" src/app.rs && \
   grep -q "use tokio::runtime::Runtime;" src/app.rs && \
   grep -q "tokio_runtime: Arc<Runtime>" src/app.rs && \
   grep -q "runtime.spawn(async move {" src/app.rs; then
    echo "✅ All code changes detected"
else
    echo "❌ ERROR: Some code changes are missing"
    exit 1
fi

echo

# Build the project
echo "Building project in release mode..."
echo "This may take a few minutes..."
echo

if cargo build --release 2>&1 | tee /tmp/vibeterm_build.log; then
    echo
    echo "✅ Build successful!"
    echo

    # Run the application (non-blocking test)
    echo "Testing application startup..."
    echo "The app will launch. Check if it starts without panicking."
    echo "Press Ctrl+C to exit the app after verifying it works."
    echo
    read -p "Press Enter to launch VibeTerm..."

    cargo run --release

    echo
    echo "=========================================="
    echo "✅ VERIFICATION COMPLETE"
    echo "=========================================="
    echo
    echo "If the app started without the panic error:"
    echo "  'there is no reactor running, must be called from'"
    echo "  'the context of a Tokio 1.x runtime'"
    echo
    echo "Then the fix is successful!"

else
    echo
    echo "❌ Build failed. Check the error messages above."
    echo "Build log saved to: /tmp/vibeterm_build.log"
    exit 1
fi
