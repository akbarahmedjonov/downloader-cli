# Maintainer: Akbar Ahmedjonov <akbarahmedjonovdev@gmail.com>

pkgname=dw
pkgver=0.3.4
pkgrel=1
pkgdesc="A blazing fast, simple command-line file downloader written in Rust with a beautiful progress bar"
arch=('x86_64' 'aarch64')
url="https://github.com/akbarahmedjonov/downloader-cli"
license=('MIT')
depends=('gcc-libs' 'openssl')
makedepends=('cargo' 'gcc')
source=("$pkgname-$pkgver.tar.gz::https://github.com/akbarahmedjonov/downloader-cli/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 target/release/dw "$pkgdir/usr/bin/dw"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
