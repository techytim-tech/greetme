#!/bin/bash
# Package greetme as an RPM package

set -e

echo "Building greetme RPM package..."

# Check if rpmbuild is available
if ! command -v rpmbuild &> /dev/null; then
    echo "Error: rpmbuild not found. Please install rpm-build:"
    echo "  Fedora/RHEL: sudo dnf install rpm-build"
    echo "  openSUSE: sudo zypper install rpm-build"
    exit 1
fi

# Build release binary
echo "Building release binary..."
cargo build --release

# Create RPM build directories
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Compress man page
mkdir -p target/man
gzip -c man/greetme.1 > target/man/greetme.1.gz

# Create source tarball
VERSION="1.0.0"
TARBALL="greetme-${VERSION}.tar.gz"
echo "Creating source tarball..."

mkdir -p target/rpm-source/greetme-${VERSION}
cp target/release/greetme target/rpm-source/greetme-${VERSION}/
cp -r themes target/rpm-source/greetme-${VERSION}/
cp -r fonts target/rpm-source/greetme-${VERSION}/
cp -r examples target/rpm-source/greetme-${VERSION}/
cp README.md LICENSE target/rpm-source/greetme-${VERSION}/
cp target/man/greetme.1.gz target/rpm-source/greetme-${VERSION}/

cd target/rpm-source
tar czf ${TARBALL} greetme-${VERSION}
cp ${TARBALL} ~/rpmbuild/SOURCES/
cd ../..

# Copy spec file
cp greetme.spec ~/rpmbuild/SPECS/

# Build RPM
echo "Building RPM..."
rpmbuild -ba ~/rpmbuild/SPECS/greetme.spec

# Copy RPM to target directory
mkdir -p target/rpm
cp ~/rpmbuild/RPMS/x86_64/greetme-*.rpm target/rpm/ 2>/dev/null || true
cp ~/rpmbuild/RPMS/aarch64/greetme-*.rpm target/rpm/ 2>/dev/null || true

echo "✓ RPM package created successfully!"
echo "Package location: target/rpm/"
ls -lh target/rpm/*.rpm 2>/dev/null || echo "No RPM packages found in target/rpm/"
ls -lh ~/rpmbuild/RPMS/*/greetme-*.rpm
