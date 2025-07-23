#!/bin/bash

# EvorBrain Development Certificate Trust Script
# This script adds the development certificate to the system trust store

set -e

CERT_DIR="./certs"
CERT_NAME="evorbrain-dev"

echo "🔐 Trusting development certificates for EvorBrain..."

# Check if certificate exists
if [ ! -f "$CERT_DIR/$CERT_NAME.crt" ]; then
    echo "❌ Certificate not found. Please run generate-dev-certs.sh first."
    exit 1
fi

# Detect platform
OS="$(uname -s)"

case "$OS" in
    Linux*)
        echo "🐧 Detected Linux system..."
        
        # Try different Linux certificate stores
        if command -v update-ca-certificates &> /dev/null; then
            # Debian/Ubuntu
            echo "📦 Using update-ca-certificates (Debian/Ubuntu)..."
            sudo cp "$CERT_DIR/$CERT_NAME.crt" "/usr/local/share/ca-certificates/$CERT_NAME.crt"
            sudo update-ca-certificates
        elif command -v update-ca-trust &> /dev/null; then
            # RedHat/CentOS/Fedora
            echo "📦 Using update-ca-trust (RedHat/CentOS/Fedora)..."
            sudo cp "$CERT_DIR/$CERT_NAME.crt" "/etc/pki/ca-trust/source/anchors/$CERT_NAME.crt"
            sudo update-ca-trust
        else
            echo "⚠️  Could not detect certificate management system."
            echo "   Please manually add the certificate to your trust store."
        fi
        
        # Also add to Chrome/Chromium if available
        if [ -d "$HOME/.pki/nssdb" ]; then
            echo "🌐 Adding to Chrome/Chromium trust store..."
            certutil -d sql:$HOME/.pki/nssdb -A -t "P,," -n "$CERT_NAME" -i "$CERT_DIR/$CERT_NAME.crt" || true
        fi
        ;;
        
    Darwin*)
        echo "🍎 Detected macOS system..."
        echo "🔑 Adding certificate to macOS Keychain..."
        sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain "$CERT_DIR/$CERT_NAME.crt"
        ;;
        
    MINGW* | CYGWIN* | MSYS*)
        echo "🪟 Detected Windows system..."
        echo "📝 Adding certificate to Windows trust store..."
        certutil -addstore -f "ROOT" "$CERT_DIR/$CERT_NAME.crt"
        ;;
        
    *)
        echo "❌ Unknown operating system: $OS"
        echo "   Please manually add the certificate to your trust store."
        exit 1
        ;;
esac

echo "✅ Development certificate trusted successfully!"
echo ""
echo "⚠️  Note: You may need to restart your browser for changes to take effect."
echo "   Some browsers may still show warnings for self-signed certificates."