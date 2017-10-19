# Maintainer: Simon Pettersson <simon.v.pettersson@gmail.com>

# Maintainer: Your Name <youremail@domain.com>
toolname=spock
pkgname=${toolname}-git
pkgver=VERSION
pkgrel=1
pkgdesc="Project generator for C++"
arch=('x86_64')
url="https://github.com/simonvpe/spock"
license=('GPL3')
groups=()
depends=()
makedepends=('git' 'rust')
provides=("${pkgname%-VCS}")
conflicts=("${pkgname%-VCS}")
replaces=()
backup=()
options=()
install=
source=("${pkgname}::git+https://github.com/simonvpe/spock")
noextract=()
md5sums=('SKIP')

# Please refer to the 'USING VCS SOURCES' section of the PKGBUILD man page for
# a description of each element in the source array.

pkgver() {
    cd "${pkgname}"

# The examples below are not absolute and need to be adapted to each repo. The
# primary goal is to generate version numbers that will increase according to
# pacman's version comparisons with later commits to the repo. The format
# VERSION='VER_NUM.rREV_NUM.HASH', or a relevant subset in case VER_NUM or HASH
# are not available, is recommended.

    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
    cd "${pkgname}"
    cargo build --release
}

check() {
    cd "${pkgname}"
}

package() {
    cd "${pkgname}"
    
    prefix="${pkgdir}/usr"
    share="${prefix}/share/spock/"
    
    cargo install --root "${prefix}"

    install -d -m 755 "${share}"
    cp -r templates/* "${share}"
    chmod -R 755 "${share}"
}
