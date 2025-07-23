#!/bin/bash

# EvorBrain Development Certificate Generation Script
# This script generates self-signed certificates for local HTTPS development

set -e

CERT_DIR="./certs"
CERT_NAME="evorbrain-dev"
DAYS_VALID=365

echo "🔐 Generating development certificates for EvorBrain..."

# Create certificates directory if it doesn't exist
mkdir -p "$CERT_DIR"

# Generate private key
echo "📝 Generating private key..."
openssl genrsa -out "$CERT_DIR/$CERT_NAME.key" 2048

# Generate certificate signing request
echo "📋 Creating certificate signing request..."
openssl req -new -key "$CERT_DIR/$CERT_NAME.key" -out "$CERT_DIR/$CERT_NAME.csr" \
  -subj "/C=US/ST=Development/L=Local/O=EvorBrain/OU=Development/CN=localhost"

# Generate self-signed certificate
echo "🏷️  Generating self-signed certificate..."
openssl x509 -req -days $DAYS_VALID -in "$CERT_DIR/$CERT_NAME.csr" \
  -signkey "$CERT_DIR/$CERT_NAME.key" -out "$CERT_DIR/$CERT_NAME.crt" \
  -extfile <(printf "subjectAltName=DNS:localhost,IP:127.0.0.1")

# Generate PEM file (combined cert and key)
echo "🔗 Creating PEM file..."
cat "$CERT_DIR/$CERT_NAME.crt" "$CERT_DIR/$CERT_NAME.key" > "$CERT_DIR/$CERT_NAME.pem"

# Clean up CSR file
rm "$CERT_DIR/$CERT_NAME.csr"

# Set appropriate permissions
chmod 600 "$CERT_DIR/$CERT_NAME.key"
chmod 644 "$CERT_DIR/$CERT_NAME.crt"
chmod 600 "$CERT_DIR/$CERT_NAME.pem"

echo "✅ Development certificates generated successfully!"
echo ""
echo "Certificate files created in $CERT_DIR:"
echo "  - $CERT_NAME.key (private key)"
echo "  - $CERT_NAME.crt (certificate)"
echo "  - $CERT_NAME.pem (combined)"
echo ""
echo "⚠️  Remember: These are self-signed certificates for development only!"
echo "    You may need to add the certificate to your system's trust store."