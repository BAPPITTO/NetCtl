#!/bin/bash
# Build NetCtl Packages - Automation Script
# Builds Debian (.deb) and Red Hat (.rpm) packages for NetCtl

set -e

PROJECT_DIR="$(pwd)"
VERSION="1.0.0"
RELEASE_DIR="$PROJECT_DIR/releases"

echo "NetCtl Package Builder"
echo "======================"
echo "Version: $VERSION"
echo "Project: $PROJECT_DIR"
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# Check prerequisites
print_step "Checking prerequisites..."

command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "ERROR: npm not found"; exit 1; }

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if grep -q "Debian\|Ubuntu" /etc/os-release 2>/dev/null; then
        DISTRO="debian"
        command -v dpkg-buildpackage >/dev/null 2>&1 || print_info "Install: sudo apt install dpkg-dev"
    elif grep -q "CentOS\|Fedora\|RedHat" /etc/os-release 2>/dev/null; then
        DISTRO="rhel"
        command -v rpmbuild >/dev/null 2>&1 || print_info "Install: sudo yum install rpm-build"
    fi
else
    print_info "Cross-compilation may be needed for non-Linux systems"
    DISTRO="manual"
fi

print_success "Prerequisites checked"

# Create release directory
print_step "Setting up release directory..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"
print_success "Release directory created: $RELEASE_DIR"

# Build backend
print_step "Building backend binaries..."
cd "$PROJECT_DIR/backend"
cargo build --release --lib --bin netctl-daemon --bin netctl-cli --bin netctl-tui 2>&1 | tail -5
print_success "Backend build complete"

# Build frontend
print_step "Building frontend dashboard..."
cd "$PROJECT_DIR/frontend"
npm install --silent >/dev/null 2>&1 || true
npm run build >/dev/null 2>&1
print_success "Frontend build complete"

# Build Debian package
if [[ "$DISTRO" == "debian" || "$DISTRO" == "manual" ]]; then
    print_step "Building Debian package..."
    cd "$PROJECT_DIR"

    # Create debian source formats
    mkdir -p debian

    # Build with dpkg-buildpackage
    if command -v dpkg-buildpackage >/dev/null 2>&1; then
        # Clean previous build
        dpkg-buildpackage -T clean 2>/dev/null || true

        # Build binary packages
        debuild -b -uc -us 2>&1 | tail -10 || print_info "dpkg-buildpackage incomplete"

        # Copy to release folder
        if [ -f "../netctl_${VERSION}-1_amd64.deb" ]; then
            cp "../netctl_${VERSION}-1_amd64.deb" "$RELEASE_DIR/"
            print_success "Debian package: netctl_${VERSION}-1_amd64.deb"
        fi
    else
        print_info "dpkg-buildpackage not found - skipping Debian build"
    fi
fi

# Build RPM package
if [[ "$DISTRO" == "rhel" || "$DISTRO" == "manual" ]]; then
    print_step "Building Red Hat package..."
    cd "$PROJECT_DIR"

    # Setup RPM build environment
    mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS,tmp}

    # Determine version from Git
    GIT_VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "$VERSION")
    print_info "Using version: $GIT_VERSION"

    # Copy spec file and update version dynamically
    SPEC_FILE=packaging/rpm/netctl.spec
    TMP_SPEC=~/rpmbuild/SPECS/netctl.spec
    mkdir -p ~/rpmbuild/SPECS
    sed "s/^Version:.*$/Version: $GIT_VERSION/" "$SPEC_FILE" > "$TMP_SPEC"

    # Create source tarball
    tar czf ~/rpmbuild/SOURCES/netctl-${GIT_VERSION}.tar.gz \
        --exclude=.git \
        --exclude=target \
        --exclude=dist \
        --exclude=node_modules \
        --exclude=debian \
        --exclude=.DS_Store \
        . 2>/dev/null || true

    # Build RPM
    if command -v rpmbuild >/dev/null 2>&1; then
        rpmbuild -bb "$TMP_SPEC" 2>&1 | tail -10 || print_info "rpmbuild incomplete"

        # Copy to release folder
        RPM_FILE=$(find ~/rpmbuild/RPMS -name "netctl-${GIT_VERSION}-*.rpm" 2>/dev/null | head -1)
        if [ -n "$RPM_FILE" ]; then
            cp "$RPM_FILE" "$RELEASE_DIR/"
            print_success "RPM package: $(basename $RPM_FILE)"
        fi
    else
        print_info "rpmbuild not found - skipping RPM build"
    fi
fi

# Verify packages
print_step "Verifying built packages..."
if [ -d "$RELEASE_DIR" ] && [ -n "$(ls -A $RELEASE_DIR/*.deb $RELEASE_DIR/*.rpm 2>/dev/null)" ]; then
    echo ""
    echo "Generated packages:"
    ls -lh "$RELEASE_DIR"/*
    print_success "All packages built successfully"
else
    print_info "No packages generated - check build logs above"
fi

# Summary
echo ""
print_success "Build complete!"
echo ""
echo "Next steps:"
echo " 1. Test packages on target system"
echo " 2. Upload to package repositories (apt, yum)"
echo " 3. Create GitHub release with packages"
echo ""
echo "Installation test:"
echo " sudo dpkg -i $RELEASE_DIR/netctl*.deb"
echo " sudo rpm -i $RELEASE_DIR/netctl*.rpm"
echo ""