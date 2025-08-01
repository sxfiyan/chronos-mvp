#!/bin/bash

echo "=== Chronos MVP v0.1 - Forensic Timeline Generator Demo ==="
echo

# Check if the executable exists
if [ ! -f "./target/release/chronos" ]; then
    echo "Error: chronos executable not found. Please build the project first:"
    echo "cargo build --release"
    exit 1
fi

echo "1. Testing help command..."
./target/release/chronos --help
echo

echo "2. Testing error handling with non-existent file..."
./target/release/chronos nonexistent.E01
echo

echo "3. Testing with sample disk image..."
./target/release/chronos test.dd
echo

echo "4. Checking generated output..."
if [ -f "timeline.html" ]; then
    echo "✓ timeline.html generated successfully"
    echo "File size: $(ls -lh timeline.html | awk '{print $5}')"
    echo "Events found: $(grep -c '<tr>' timeline.html)"
else
    echo "✗ timeline.html not found"
fi

echo
echo "=== Demo completed ==="
echo "You can open timeline.html in a web browser to view the forensic timeline." 