# Maintainer: greetme contributors <greetme@example.com>
pkgname=greetme
pkgver=1.0.0
pkgrel=1
pkgdesc="A fast, themeable terminal greeting application"
arch=('x86_64' 'aarch64')
url="https://github.com/user/greetme"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/user/greetme/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    
    # Install man page
    install -Dm644 "man/$pkgname.1" "$pkgdir/usr/share/man/man1/$pkgname.1"
    gzip "$pkgdir/usr/share/man/man1/$pkgname.1"
    
    # Install themes
    install -d "$pkgdir/usr/share/$pkgname/themes"
    install -Dm644 themes/*.toml "$pkgdir/usr/share/$pkgname/themes/"
    
    # Install fonts
    install -d "$pkgdir/usr/share/$pkgname/fonts"
    install -Dm644 fonts/*.flf "$pkgdir/usr/share/$pkgname/fonts/"
    
    # Install examples
    install -d "$pkgdir/usr/share/$pkgname/examples"
    install -Dm644 examples/config.toml "$pkgdir/usr/share/$pkgname/examples/"
    
    # Install documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
