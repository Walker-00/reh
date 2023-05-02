# Maintainer: Linus Walker
pkgname=reh
pkgver=0.1.0
pkgrel=1
pkgdesc="Video To Live Wallpaper For x86_64 gnu linux"
license=("MIT")
arch=("x86_64")
makedepends=("cargo")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    return 0
}

package() {
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    cargo install --no-track --path . --root "$usrdir"
}

