#!/bin/bash
# Build NetCtl packages using Docker

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RELEASES_DIR="$PROJECT_ROOT/releases"

# Create releases directory
mkdir -p "$RELEASES_DIR"

echo "========================================"
echo "Building NetCtl Linux Packages"
echo "========================================"
echo ""

# Build Debian package
echo "1. Building Debian/Ubuntu package..."
echo "   Dockerfile: packaging/docker/Dockerfile.debian"
echo ""

docker build \
  -f "$PROJECT_ROOT/packaging/docker/Dockerfile.debian" \
  -t netctl-debian:1.0.0 \
  "$PROJECT_ROOT"

docker create --name netctl-debian-build netctl-debian:1.0.0
docker cp netctl-debian-build:/build/netctl_1.0.0-1_amd64.deb "$RELEASES_DIR/"
docker rm netctl-debian-build

if [ -f "$RELEASES_DIR/netctl_1.0.0-1_amd64.deb" ]; then
  echo "   ✓ Debian package built: $RELEASES_DIR/netctl_1.0.0-1_amd64.deb"
  ls -lah "$RELEASES_DIR/netctl_1.0.0-1_amd64.deb"
else
  echo "   ✗ Failed to build Debian package"
  exit 1
fi

echo ""
echo "2. Building CentOS/RHEL package..."
echo "   Dockerfile: packaging/docker/Dockerfile.rpm"
echo ""

docker build \
  -f "$PROJECT_ROOT/packaging/docker/Dockerfile.rpm" \
  -t netctl-rpm:1.0.0 \
  "$PROJECT_ROOT"

docker create --name netctl-rpm-build netctl-rpm:1.0.0
docker cp netctl-rpm-build:/build/netctl-1.0.0-1.el8.x86_64.rpm "$RELEASES_DIR/"
docker rm netctl-rpm-build

if [ -f "$RELEASES_DIR/netctl-1.0.0-1.el8.x86_64.rpm" ]; then
  echo "   ✓ RPM package built: $RELEASES_DIR/netctl-1.0.0-1.el8.x86_64.rpm"
  ls -lah "$RELEASES_DIR/netctl-1.0.0-1.el8.x86_64.rpm"
else
  echo "   ✗ Failed to build RPM package"
  exit 1
fi

echo ""
echo "========================================"
echo "Package Build Complete"
echo "========================================"
echo ""
echo "Created packages:"
ls -lah "$RELEASES_DIR"/*.{deb,rpm} 2>/dev/null || true
echo ""
echo "Generate checksums:"
cd "$RELEASES_DIR"
sha256sum netctl_1.0.0-1_amd64.deb netctl-1.0.0-1.el8.x86_64.rpm > CHECKSUMS.txt
cat CHECKSUMS.txt
echo ""
echo "Packages ready for release in: $RELEASES_DIR"
