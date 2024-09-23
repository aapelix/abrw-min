pkgname=abrw-min
pkgver=1.6
pkgrel=1
pkgdesc="abrw"
arch=('x86_64')
url="https://abrw.aapelix.dev/"
license=('GPL')
depends=('gtk3' 'webkit2gtk')
source=("abrw-min.desktop")
sha256sums=('SKIP')

package() {
  mkdir -p "$pkgdir/usr/bin"
  install -Dm755 "$srcdir/../target/release/abrw-min" "$pkgdir/usr/bin/abrw-min"
  mkdir -p "$pkgdir/usr/share/applications"
  install -Dm644 "$srcdir/abrw-min.desktop" "$pkgdir/usr/share/applications/abrw-min.desktop"
  install -Dm644 "$srcdir/icon.png" "$pkgdir/usr/share/pixmaps/myicon.png"
}
