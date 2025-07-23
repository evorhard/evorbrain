#!/bin/bash

# Test script for Tauri window creation
# This script runs the Tauri development server and tests basic functionality

set -e

echo "🧪 Testing EvorBrain Tauri Window Creation..."
echo ""

# Check if we're in the right directory
if [ ! -f "pnpm-workspace.yaml" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    # Kill any running processes
    pkill -f "tauri dev" || true
    pkill -f "vite" || true
}

# Set up cleanup on exit
trap cleanup EXIT

echo "📦 Installing dependencies..."
pnpm install

echo ""
echo "🏗️  Building workspace packages..."
pnpm --filter '@evorbrain/*' build

echo ""
echo "🚀 Starting Tauri development server..."
echo "   The application window should open automatically."
echo "   Test the following:"
echo "   1. ✅ Window opens with correct size (1200x800)"
echo "   2. ✅ Window title shows 'EvorBrain'"
echo "   3. ✅ Click 'Test Greet Command' button"
echo "   4. ✅ Try window control buttons"
echo "   5. ✅ Resize the window (check console for events)"
echo ""
echo "   Press Ctrl+C to stop the test"
echo ""

# Run Tauri in development mode
cd apps/desktop
exec pnpm tauri:dev