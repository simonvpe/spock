# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# The following guidelines are specific to BZR, GIT, HG and SVN packages.
# Other VCS sources are not natively supported by makepkg yet.

# Maintainer: Your Name <youremail@domain.com>
toolname=spock
pkgname=${toolname}-git
pkgver=r11.49f23d9
pkgrel=1
pkgdesc=""
arch=('x86_64')
url=""
license=('GPL')
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
    echo $bin >/tmp/pkgdir
    
    cargo install --root "${prefix}"

    install -d -m 755 "${share}"
    cp -r templates/* "${share}"
    chmod -R 755 "${share}"
}
