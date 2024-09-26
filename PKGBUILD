pkgname=abrw-min
pkgver=2.3.5
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
  install -Dm644 "$srcdir/icons/icon.png" "$pkgdir/usr/share/pixmaps/myicon.png"

  install -Dm644 "$srcdir/icons/align-justify.svg" "$pkgdir/usr/share/pixmaps/align-justify.svg"
  install -Dm644 "$srcdir/icons/bookmark-check.svg" "$pkgdir/usr/share/pixmaps/bookmark-check.svg"
  install -Dm644 "$srcdir/icons/bookmark.svg" "$pkgdir/usr/share/pixmaps/bookmark.svg"
  install -Dm644 "$srcdir/icons/chevron-left.svg" "$pkgdir/usr/share/pixmaps/chevron-left.svg"
  install -Dm644 "$srcdir/icons/chevron-right.svg" "$pkgdir/usr/share/pixmaps/chevron-right.svg"
  install -Dm644 "$srcdir/icons/monitor-cog.svg" "$pkgdir/usr/share/pixmaps/monitor-cog.svg"
  install -Dm644 "$srcdir/icons/plus.svg" "$pkgdir/usr/share/pixmaps/plus.svg"
  install -Dm644 "$srcdir/icons/rotate-cw.svg" "$pkgdir/usr/share/pixmaps/rotate-cw.svg"
  install -Dm644 "$srcdir/icons/shield-ban.svg" "$pkgdir/usr/share/pixmaps/shield-ban.svg"
  install -Dm644 "$srcdir/icons/download.svg" "$pkgdir/usr/share/pixmaps/download.svg"
}
