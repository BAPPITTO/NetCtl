#!/bin/bash
# Create and manage NetCtl releases

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION="${1:-1.0.0}"
RELEASE_DIR="$PROJECT_ROOT/releases/$VERSION"

# Color outputs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
  echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $1"
}

# Parse command
COMMAND="${1:-help}"
shift 2>/dev/null || true

case "$COMMAND" in
  create-tag)
    log_info "Creating git tag v$VERSION..."
    cd "$PROJECT_ROOT"
    git tag -a "v$VERSION" -m "Release $VERSION" || log_warn "Tag v$VERSION already exists"
    git push origin "v$VERSION" --force 2>/dev/null || log_warn "Could not push tag (may not have remote)"
    log_info "Tag created: v$VERSION"
    ;;
    
  trigger-workflow)
    log_info "Triggering GitHub Actions workflow..."
    log_warn "Requires 'gh' CLI to be installed"
    log_info "Install with: brew install gh"
    log_info "Then run: gh workflow run build-release.yml"
    ;;
    
  create-mock-packages)
    log_info "Creating mock package structure for testing..."
    
    mkdir -p "$RELEASE_DIR/debian/DEBIAN"
    mkdir -p "$RELEASE_DIR/debian/usr/bin"
    mkdir -p "$RELEASE_DIR/rpm/usr/bin"
    
    # Create mock debian control
    cat > "$RELEASE_DIR/debian/DEBIAN/control" << 'EOF'
Package: netctl
Version: 1.0.0-1
Architecture: amd64
Maintainer: Jericho Foster <jerichofoster@example.com>
Homepage: https://github.com/jerichofoster/NetCtl
Description: Network Control Engine
Depends: libc6
Priority: optional
Section: net
EOF

    # Create mock binaries (placeholders for testing)
    touch "$RELEASE_DIR/debian/usr/bin/netctl"
    touch "$RELEASE_DIR/debian/usr/bin/netctl-tui"
    chmod 755 "$RELEASE_DIR/debian/usr/bin/"*
    
    log_info "Mock packages created in: $RELEASE_DIR"
    ls -la "$RELEASE_DIR"
    ;;
    
  list-releases)
    log_info "Available releases:"
    if [ -d "$PROJECT_ROOT/releases" ]; then
      ls -1 "$PROJECT_ROOT/releases/"
    else
      log_warn "No releases directory found"
    fi
    ;;
    
  show-instructions)
    cat << EOF
${GREEN}NetCtl Release Instructions${NC}

${YELLOW}Automatic Release (Recommended):${NC}
1. Ensure GitHub CLI is installed:
   brew install gh
   
2. Create and push a version tag:
   git tag -a v1.0.0 -m "Release 1.0.0"
   git push origin v1.0.0
   
3. GitHub Actions will automatically:
   ✓ Build Debian package on Ubuntu
   ✓ Build RPM package on Rocky Linux
   ✓ Create GitHub release with packages
   ✓ Generate SHA256 checksums

${YELLOW}Manual Release (For testing):${NC}
1. This requires a Linux system with:
   - Rust toolchain
   - Node.js & npm
   - Build tools (make, gcc, etc.)

2. Run:
   bash packaging/docker/build-packages.sh
   
3. Packages will be in:
   releases/netctl_1.0.0-1_amd64.deb
   releases/netctl-1.0.0-1.el8.x86_64.rpm

${YELLOW}Package Verification:${NC}
   sha256sum -c releases/\$VERSION/CHECKSUMS.txt

${YELLOW}Installation from Release:${NC}
   # Ubuntu/Debian
   sudo apt install ./netctl_1.0.0-1_amd64.deb
   
   # CentOS/RHEL
   sudo rpm -i ./netctl-1.0.0-1.el8.x86_64.rpm

${YELLOW}First Run:${NC}
   sudo netctl-tui  # Interactive setup wizard
   sudo systemctl start netctl
   
   Dashboard: http://localhost:3001
EOF
    ;;
    
  *)
    cat << EOF
${GREEN}NetCtl Release Manager${NC}

Usage: $0 <command> [options]

Commands:
  create-tag              Create git tag for new release
  trigger-workflow        Trigger GitHub Actions build workflow
  create-mock-packages    Create mock package structure (for testing)
  list-releases           List existing releases
  show-instructions       Display detailed release instructions
  help                    Show this help message

Examples:
  # Prepare for release
  $0 create-tag
  
  # Check release status
  $0 list-releases
  
  # View full instructions
  $0 show-instructions

${YELLOW}Recommended Workflow:${NC}
1. Install GitHub CLI: brew install gh
2. Create tag: $0 create-tag
3. GitHub Actions builds packages automatically
4. Release appears on GitHub with packages
EOF
    ;;
esac
