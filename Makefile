.PHONY: all build test clean install package-deb package-rpm package-arch lint help

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man/man1
SHAREDIR ?= $(PREFIX)/share/greetme

all: build

build:
	@echo "Building greetme..."
	cargo build --release

test:
	@echo "Running tests..."
	cargo test

lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/man target/rpm-source
	rm -f greetme-*.pkg.tar.zst

install: build
	@echo "Installing greetme to $(BINDIR)..."
	install -d $(BINDIR)
	install -m 0755 target/release/greetme $(BINDIR)/
	install -d $(MANDIR)
	gzip -c man/greetme.1 | install -m 0644 /dev/stdin $(MANDIR)/greetme.1.gz
	install -d $(SHAREDIR)/{themes,fonts,examples}
	install -m 0644 themes/* $(SHAREDIR)/themes/
	install -m 0644 fonts/* $(SHAREDIR)/fonts/
	install -m 0644 examples/* $(SHAREDIR)/examples/

uninstall:
	@echo "Uninstalling greetme..."
	rm -f $(BINDIR)/greetme
	rm -f $(MANDIR)/greetme.1.gz
	rm -rf $(SHAREDIR)

package-deb:
	@echo "Building Debian package..."
	./package-deb.sh

package-rpm:
	@echo "Building RPM package..."
	./package-rpm.sh

package-arch:
	@echo "Building Arch Linux package..."
	./package-arch.sh

help:
	@echo "Available targets:"
	@echo "  build        - Build release binary"
	@echo "  test         - Run tests"
	@echo "  lint         - Run clippy linter"
	@echo "  clean        - Remove build artifacts"
	@echo "  install      - Install to system (requires sudo)"
	@echo "  uninstall    - Remove from system (requires sudo)"
	@echo "  package-deb  - Create Debian .deb package"
	@echo "  package-rpm  - Create RPM package"
	@echo "  package-arch - Create Arch Linux package"
	@echo "  help         - Show this help message"