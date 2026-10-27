# Maintainer: smtdfc <me.smtdfc@gmail.com>
pkgname=bakeryos-cli
pkgver=0.0.3
pkgrel=3
pkgdesc="CLI for BakeryOS"
arch=('x86_64')
url="https://github.com/bakeryos-project/bakeryos-cli"
license=('GPL-3.0-or-later')
depends=('gcc-libs' 'glibc' 'pacman')
makedepends=('cargo' 'git')
source=()
sha256sums=()
options=(!debug !strip)

build(){
   cd $startdir
   CFLAGS="" cargo build --release
}

package() {
   install -Dm755 "$startdir/target/release/bakeryos-cli" "$pkgdir/usr/bin/bakeryos"
   install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
