#!/bin/bash
# Prepare data files for bundling in releases
# Creates a data directory with personas and .opencode files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="${1:-$PROJECT_DIR/target/data-bundle}"

echo "Preparing data bundle in $OUTPUT_DIR"

# Clean and create output directory
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Copy personas directory
if [ -d "$PROJECT_DIR/personas" ]; then
    echo "Copying personas..."
    cp -r "$PROJECT_DIR/personas" "$OUTPUT_DIR/"
else
    echo "Warning: personas directory not found"
fi

# Copy .opencode directory (only specific files)
if [ -d "$PROJECT_DIR/.opencode" ]; then
    echo "Copying .opencode configuration..."
    mkdir -p "$OUTPUT_DIR/.opencode"

    # Copy opencode.jsonc
    if [ -f "$PROJECT_DIR/.opencode/opencode.jsonc" ]; then
        cp "$PROJECT_DIR/.opencode/opencode.jsonc" "$OUTPUT_DIR/.opencode/"
    fi

    # Copy commands directory
    if [ -d "$PROJECT_DIR/.opencode/commands" ]; then
        cp -r "$PROJECT_DIR/.opencode/commands" "$OUTPUT_DIR/.opencode/"
    fi

    # Copy plugin directory
    if [ -d "$PROJECT_DIR/.opencode/plugin" ]; then
        cp -r "$PROJECT_DIR/.opencode/plugin" "$OUTPUT_DIR/.opencode/"
    fi
else
    echo "Warning: .opencode directory not found"
fi

echo "Data bundle prepared successfully:"
find "$OUTPUT_DIR" -type f | head -20
