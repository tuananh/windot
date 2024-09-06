# Maintainer: Cherry <arch@sparklet.org>
_pkgname=windot
pkgname=$_pkgname-git
pkgver=0.2.1.58da11e
pkgrel=1
arch=(x86_64)
url="https://github.com/Lamby777/windot"
source=("$_pkgname::git+https://github.com/Lamby777/windot.git")
pkgdesc="A simple emoji picker."
md5sums=('SKIP')

depends=('gtk4')
makedepends=('gcc' 'cargo' 'git' 'pkgconf' 'libadwaita')
conflicts=('windot')
provides=("windot=${pkgver}")

# Fetch the current version using the latest commit hash
pkgver() {
    cd "$srcdir/$_pkgname"

    # get the crate version using grep and sed
    cargo_ver=$(grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
    
    # use the latest commit's rev# and hash
    git_rev=$(git rev-list --count HEAD)
    git_hash=$(git rev-parse --short HEAD)
    
    # requires the rev first for version sorting purposes
    echo "$cargo_ver.r$git_rev.$git_hash"
}

prepare() {
    cd "$srcdir/$_pkgname"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$srcdir/$_pkgname"
    make 
}

package() {
    cd "$srcdir/$_pkgname"
    export RUSTUP_TOOLCHAIN=stable

    make install DESTDIR="$pkgdir/" prefix="/usr"
}
